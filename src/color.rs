use palette::rgb::LinSrgba;
use palette::{white_point, FromColor, Gradient, Lch};
use std::num;
use std::vec;

pub fn hex_to_color(
    color_string: &str,
) -> Result<Lch<white_point::D65, f64>, std::num::ParseIntError> {
    let r = u64::from_str_radix(&color_string[0..2], 16)? as f64;
    let g = u64::from_str_radix(&color_string[2..4], 16)? as f64;
    let b = u64::from_str_radix(&color_string[4..6], 16)? as f64;

    return Ok(Lch::from_color(LinSrgba::new(r, g, b, 255.0)));
}

pub fn blend_color(
    bottom: Lch<white_point::D65, f64>,
    top: Lch<white_point::D65, f64>,
) -> Lch<white_point::D65, f64> {
    let (_, topC, topH) = top.into_components();
    let (bottomL, _, _) = bottom.into_components();

    return Lch::from_components((bottomL, topC, topH));
}

pub struct GradientDescriptor {
    pub colors: vec::Vec<Lch<white_point::D65, f64>>,
}

impl GradientDescriptor {
    pub fn new(colors: vec::Vec<Lch<white_point::D65, f64>>) -> GradientDescriptor {
        return GradientDescriptor { colors };
    }

    pub fn generate(&self, frame_count: usize) -> vec::Vec<Lch<white_point::D65, f64>> {
        let mut gen = vec::Vec::<Lch<white_point::D65, f64>>::new();

        // this only counts the number of colors forward, excluding color itself and the
        // destination
        let frames_per_color = (frame_count + 1) / (self.colors.len() + 1);
        println!("frames per color {}", frames_per_color);
        for (i, color) in self.colors.iter().enumerate() {
            let src = color;
            let dest = if i == self.colors.len() - 1 {
                &self.colors[0]
            } else {
                &self.colors[i + 1]
            };
            let grad_iter = Gradient::new(vec![src.clone(), dest.clone()]);
            println!("{}", frames_per_color);

            // generate [src, dest]
            let colors = grad_iter.take(frames_per_color + 2).collect::<Vec<_>>();
            // push [src, dest)
            for j in 0..colors.len() - 1 {
                println!("{}", j);
                gen.push(colors[j]);
            }
        }

        return gen;
    }
}

mod tests {
    use palette::rgb::LinSrgba;
    use palette::{FromColor, Lch};

    use crate::color;

    #[test]
    fn test_generate() {
        let grad_desc = color::GradientDescriptor::new(vec![
            Lch::from_color(LinSrgba::new(0., 0., 0., 1.)),
            Lch::from_color(LinSrgba::new(0.5, 0.5, 0.5, 1.)),
            Lch::from_color(LinSrgba::new(1., 1., 1., 1.)),
        ]);

        let colors = grad_desc.generate(12);
        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].chroma, 0.0);
        assert_eq!(colors[4].chroma, 0.0);
        assert_eq!(colors[8].chroma, 0.0);
    }
}
