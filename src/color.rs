use palette::gradient;
use palette::{white_point, FromColor, Lch, Mix};
use std::vec;

#[cfg(not(feature = "linear_srgb"))]
pub type ColorType = palette::rgb::Srgba<f64>;

#[cfg(feature = "linear_srgb")]
pub type ColorType = palette::rgb::LinSrgba<f64>;

pub fn hex_to_color(
    color_string: &str,
) -> Result<Lch<white_point::D65, f64>, std::num::ParseIntError> {
    let r = u64::from_str_radix(&color_string[0..2], 16)? as f64;
    let g = u64::from_str_radix(&color_string[2..4], 16)? as f64;
    let b = u64::from_str_radix(&color_string[4..6], 16)? as f64;

    return Ok(Lch::from_color(ColorType::new(r, g, b, 255.0)));
}

pub fn blend_color(
    bottom: Lch<white_point::D65, f64>,
    top: Lch<white_point::D65, f64>,
) -> Lch<white_point::D65, f64> {
    let (_, top_c, top_h) = top.into_components();
    let (bottom_l, _, _) = bottom.into_components();

    return Lch::from_components((bottom_l, top_c, top_h));
}

pub struct GradientDescriptor {
    pub colors: vec::Vec<Lch<white_point::D65, f64>>,
    pub positions: vec::Vec<f64>,
}

struct GradientKeyFrame<'a> {
    color: &'a Lch<white_point::D65, f64>,
    position: f64,
}

pub enum GradientGeneratorType {
    // where the colors are calculated by global and local position
    Discrete,
    // where palette generates it, taking into account all colors
    Continuous,
}

impl GradientDescriptor {
    pub fn new(mut colors: vec::Vec<Lch<white_point::D65, f64>>) -> GradientDescriptor {
        colors.push(colors[0].clone());
        let rng = 0..colors.len();
        let length = std::cmp::max(colors.len() - 1, 1);
        return GradientDescriptor {
            colors,
            positions: rng.map(|i| (i as f64) / (length as f64)).collect(),
        };
    }

    pub fn generate(
        &self,
        frame_count: usize,
        generator_type: GradientGeneratorType,
    ) -> vec::Vec<Lch<white_point::D65, f64>> {
        return match generator_type {
            GradientGeneratorType::Continuous => self.generate_continuous(frame_count),
            GradientGeneratorType::Discrete => self.generate_discrete(frame_count),
        };
    }

    fn generate_continuous(&self, frame_count: usize) -> vec::Vec<Lch<white_point::D65, f64>> {
        let grad = gradient::Gradient::new(self.colors.clone());
        return grad.take(frame_count + 1).take(frame_count).collect();
    }

    fn generate_discrete(&self, frame_count: usize) -> vec::Vec<Lch<white_point::D65, f64>> {
        let mut gen = vec::Vec::<Lch<white_point::D65, f64>>::new();

        for i in 0..frame_count {
            let global_position = (i as f64) / (frame_count as f64);

            let (key_frame_src, key_frame_dest) = self.position_search(global_position);
            let local_position = (global_position - key_frame_src.position)
                / (key_frame_dest.position - key_frame_src.position);

            let src = key_frame_src.color;
            let dest = key_frame_dest.color;

            gen.push(src.mix(&dest, local_position));
        }

        return gen;
    }

    fn position_search<'a>(
        &'a self,
        position: f64,
    ) -> (GradientKeyFrame<'a>, GradientKeyFrame<'a>) {
        let base = 1.0 / ((self.colors.len() - 1) as f64);
        let lower_index = (position / base).floor() as usize;

        if lower_index == self.colors.len() - 1 {
            return (
                GradientKeyFrame {
                    color: &self.colors[lower_index],
                    position: self.positions[lower_index],
                },
                GradientKeyFrame {
                    color: &self.colors[0],
                    position: self.positions[0],
                },
            );
        }

        return (
            GradientKeyFrame {
                color: &self.colors[lower_index],
                position: self.positions[lower_index],
            },
            GradientKeyFrame {
                color: &self.colors[lower_index + 1],
                position: self.positions[lower_index + 1],
            },
        );
    }
}

#[cfg(test)]
mod tests {
    use palette::{FromColor, Lch};

    use crate::color;

    #[test]
    fn test_generate_discrete() {
        let grad_desc = color::GradientDescriptor::new(vec![
            Lch::from_color(color::ColorType::new(0., 0., 0., 1.)),
            Lch::from_color(color::ColorType::new(0.5, 0.5, 0.5, 1.)),
            Lch::from_color(color::ColorType::new(1., 1., 1., 1.)),
        ]);

        let colors = grad_desc.generate(12, color::GradientGeneratorType::Discrete);
        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].chroma, 0.0);
        assert_eq!(colors[4].chroma, 0.0);
        assert_eq!(colors[8].chroma, 0.0);
    }

    #[test]
    fn test_generate_continuous() {
        let grad_desc = color::GradientDescriptor::new(vec![
            Lch::from_color(color::ColorType::new(0., 0., 0., 1.)),
            Lch::from_color(color::ColorType::new(0.5, 0.5, 0.5, 1.)),
            Lch::from_color(color::ColorType::new(1., 1., 1., 1.)),
        ]);

        let colors = grad_desc.generate(12, color::GradientGeneratorType::Continuous);
        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].chroma, 0.0);
        assert_eq!(colors[4].chroma, 0.0);
        assert_eq!(colors[8].chroma, 0.0);
    }
}
