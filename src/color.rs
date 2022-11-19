use clap::{builder::PossibleValue, ValueEnum};
use palette::gradient;
use palette::{encoding, white_point, FromColor, Hsla, Hsva, LabHue, Lcha, Mix, RgbHue};
use std::vec;

#[cfg(not(feature = "linear_srgb"))]
pub type ColorType = palette::rgb::Srgba<f64>;

#[cfg(feature = "linear_srgb")]
pub type ColorType = palette::rgb::LinSrgba<f64>;

#[derive(Clone, Copy)]
pub enum ColorSpace {
    HSL,
    HSV,
    LCH,
}

impl ValueEnum for ColorSpace {
    fn value_variants<'a>() -> &'a [Self] {
        &[ColorSpace::HSL, ColorSpace::HSV, ColorSpace::LCH]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            ColorSpace::HSL => PossibleValue::new("hsl").help("HSL"),
            ColorSpace::HSV => PossibleValue::new("hsv").help("HSV"),
            ColorSpace::LCH => {
                PossibleValue::new("lch").help("CIE L*C*h°, a polar version of CIE L*a*b*")
            }
        })
    }
}

impl std::fmt::Display for ColorSpace {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for ColorSpace {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

pub fn from_hex<C>(color_string: &str) -> Result<C, std::num::ParseIntError>
where
    C: FromColor<ColorType>,
{
    let r = u8::from_str_radix(&color_string[0..2], 16)? as f64;
    let g = u8::from_str_radix(&color_string[2..4], 16)? as f64;
    let b = u8::from_str_radix(&color_string[4..6], 16)? as f64;

    return Ok(C::from_color(ColorType::new(r, g, b, 255.0)));
}

pub trait Componentize<H, C, L, A> {
    fn get_components(&self) -> (H, C, L, A);

    fn from_components(h: H, c: C, l: L, a: A) -> Self;
}

impl Componentize<LabHue<f64>, f64, f64, f64> for Lcha<white_point::D65, f64> {
    fn get_components(&self) -> (LabHue<f64>, f64, f64, f64) {
        let (l, c, h, a) = self.into_components();
        return (h, c, l, a);
    }

    fn from_components(h: LabHue<f64>, c: f64, l: f64, a: f64) -> Self {
        return Lcha::from_components((l, c, h, a));
    }
}

impl Componentize<RgbHue<f64>, f64, f64, f64> for Hsla<encoding::Srgb, f64> {
    fn get_components(&self) -> (RgbHue<f64>, f64, f64, f64) {
        return self.into_components();
    }

    fn from_components(h: RgbHue<f64>, c: f64, l: f64, a: f64) -> Self {
        return Hsla::from_components((h, c, l, a));
    }
}

impl Componentize<RgbHue<f64>, f64, f64, f64> for Hsva<encoding::Srgb, f64> {
    fn get_components(&self) -> (RgbHue<f64>, f64, f64, f64) {
        return self.into_components();
    }

    fn from_components(h: RgbHue<f64>, c: f64, l: f64, a: f64) -> Self {
        return Hsva::from_components((h, c, l, a));
    }
}

pub fn blend_colors<H, C, L, A, Color: Componentize<H, C, L, A>>(
    bottom: &Color,
    top: &Color,
) -> Color {
    let (top_h, _, _, _) = top.get_components();
    let (_, bottom_c, bottom_l, bottom_a) = bottom.get_components();

    return Color::from_components(top_h, bottom_c, bottom_l, bottom_a);
}

#[derive(Clone, Copy)]
pub enum GradientGeneratorType {
    // where the colors are calculated by global and local position
    Discrete,
    // where palette generates it, taking into account all colors
    Continuous,
}

impl ValueEnum for GradientGeneratorType {
    fn value_variants<'a>() -> &'a [Self] {
        &[
            GradientGeneratorType::Discrete,
            GradientGeneratorType::Continuous,
        ]
    }

    fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
        Some(match self {
            GradientGeneratorType::Discrete => PossibleValue::new("discrete")
                .help("Colors are calculated by global and local position"),
            GradientGeneratorType::Continuous => PossibleValue::new("continuous")
                .help("Palette generates the gradient, taking into account all of the colors"),
        })
    }
}

impl std::fmt::Display for GradientGeneratorType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.to_possible_value()
            .expect("no values are skipped")
            .get_name()
            .fmt(f)
    }
}

impl std::str::FromStr for GradientGeneratorType {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        for variant in Self::value_variants() {
            if variant.to_possible_value().unwrap().matches(s, false) {
                return Ok(*variant);
            }
        }
        Err(format!("Invalid variant: {}", s))
    }
}

struct GradientKeyFrame<'a, C>
where
    C: Mix<Scalar = f64> + Sized,
{
    color: &'a C,
    position: f64,
}

pub struct GradientDescriptor<C: Mix<Scalar = f64> + Sized> {
    pub colors: vec::Vec<C>,
    pub positions: vec::Vec<f64>,
}

impl<C: Mix<Scalar = f64> + Sized + Clone> GradientDescriptor<C> {
    pub fn new(mut colors: vec::Vec<C>) -> GradientDescriptor<C> {
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
    ) -> vec::Vec<C> {
        return match generator_type {
            GradientGeneratorType::Continuous => self.generate_continuous(frame_count),
            GradientGeneratorType::Discrete => self.generate_discrete(frame_count),
        };
    }

    fn generate_continuous(&self, frame_count: usize) -> vec::Vec<C> {
        let grad = gradient::Gradient::new(self.colors.clone());
        return grad.take(frame_count + 1).take(frame_count).collect();
    }

    fn generate_discrete(&self, frame_count: usize) -> vec::Vec<C> {
        let mut gen = vec::Vec::<C>::new();

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
    ) -> (GradientKeyFrame<'a, C>, GradientKeyFrame<'a, C>) {
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
    use palette::{FromColor, Lcha};

    use crate::color;

    #[test]
    fn test_generate_discrete() {
        let grad_desc = color::GradientDescriptor::new(vec![
            Lcha::from_color(color::ColorType::new(0., 0., 0., 1.)),
            Lcha::from_color(color::ColorType::new(0.5, 0.5, 0.5, 1.)),
            Lcha::from_color(color::ColorType::new(1., 1., 1., 1.)),
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
            Lcha::from_color(color::ColorType::new(0., 0., 0., 1.)),
            Lcha::from_color(color::ColorType::new(0.5, 0.5, 0.5, 1.)),
            Lcha::from_color(color::ColorType::new(1., 1., 1., 1.)),
        ]);

        let colors = grad_desc.generate(12, color::GradientGeneratorType::Continuous);
        assert_eq!(colors.len(), 12);

        assert_eq!(colors[0].chroma, 0.0);
        assert_eq!(colors[4].chroma, 0.0);
        assert_eq!(colors[8].chroma, 0.0);
    }
}
