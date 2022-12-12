use std::vec;

use clap::{builder::PossibleValue, ValueEnum};
use palette::gradient;
use palette::{encoding, white_point, FromColor, Hsla, Hsva, LabHue, Lcha, Mix, RgbHue};

use crate::commandline;

pub type ScalarType = f64;

// TODO look into using linear
// pub type ColorType = palette::rgb::LinSrgba<ScalarType>;

pub type ColorType = palette::rgb::Srgba<ScalarType>;

pub trait Color:
    palette::FromColor<ColorType>
    + palette::convert::FromColorUnclamped<ColorType>
    + palette::IntoColor<ColorType>
    + palette::convert::IntoColorUnclamped<ColorType>
    + palette::WithAlpha<ScalarType>
    + palette::Mix<Scalar = ScalarType>
    + Clone
    + Sized
{
}

impl<T> Color for T where
    T: palette::FromColor<ColorType>
        + palette::convert::FromColorUnclamped<ColorType>
        + palette::IntoColor<ColorType>
        + palette::convert::IntoColorUnclamped<ColorType>
        + palette::WithAlpha<ScalarType>
        + palette::Mix<Scalar = ScalarType>
        + Clone
        + Sized
{
}

commandline::define_cli_enum!(MixingMode, {
    Custom: ("custom", "Mixes the color by taking the hue component of the other color, keeping the base luma and chroma"),
    Lab: ("lab", "Mixes the color by taking the color components of the other color, keeping the base lightness"),
    Linear: ("linear", "Uses palettee for linear mixing"),
    BlendOverlay: ("blend_overlay", "Uses blending: overlay"),
});

commandline::define_cli_enum!(ColorSpace, {
    HSL: ("hsl", "The HSL color space can be seen as a cylindrical version of RGB, where the hue is the angle around the color cylinder, the saturation is the distance from the center, and the lightness is the height from the bottom."),
    HSV: ("hsv", "HSV is a cylindrical version of RGB and it’s very similar to HSL. The difference is that the value component in HSV determines the brightness of the color, and not the lightness."),
    LCH: ("lch", "L*C*h° shares its range and perceptual uniformity with L*a*b*, but it’s a cylindrical color space, like HSL and HSV. This gives it the same ability to directly change the hue and colorfulness of a color, while preserving other visual aspects."),
    RGB: ("rgb", "RGB"),
    LAB: ("lab", "The CIE L*a*b* (CIELAB) color space")
});

pub fn from_hex<C>(color_string: &str) -> Result<C, std::num::ParseIntError>
where
    C: FromColor<ColorType>,
{
    let r = u8::from_str_radix(&color_string[0..2], 16)? as ScalarType;
    let g = u8::from_str_radix(&color_string[2..4], 16)? as ScalarType;
    let b = u8::from_str_radix(&color_string[4..6], 16)? as ScalarType;

    return Ok(C::from_color(ColorType::new(r, g, b, 255.0)));
}

pub trait Componentize<H, C, L, A> {
    fn get_components(&self) -> (H, C, L, A);

    fn from_components(h: H, c: C, l: L, a: A) -> Self;
}

impl Componentize<LabHue<ScalarType>, ScalarType, ScalarType, ScalarType>
    for Lcha<white_point::D65, ScalarType>
{
    fn get_components(&self) -> (LabHue<ScalarType>, ScalarType, ScalarType, ScalarType) {
        let (l, c, h, a) = self.into_components();
        return (h, c, l, a);
    }

    fn from_components(h: LabHue<ScalarType>, c: ScalarType, l: ScalarType, a: ScalarType) -> Self {
        return Lcha::from_components((l, c, h, a));
    }
}

impl Componentize<RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType>
    for Hsla<encoding::Srgb, ScalarType>
{
    fn get_components(&self) -> (RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType) {
        return self.into_components();
    }

    fn from_components(h: RgbHue<ScalarType>, c: ScalarType, l: ScalarType, a: ScalarType) -> Self {
        return Hsla::from_components((h, c, l, a));
    }
}

impl Componentize<RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType>
    for Hsva<encoding::Srgb, ScalarType>
{
    fn get_components(&self) -> (RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType) {
        return self.into_components();
    }

    fn from_components(h: RgbHue<ScalarType>, c: ScalarType, l: ScalarType, a: ScalarType) -> Self {
        return Hsva::from_components((h, c, l, a));
    }
}

pub fn blend_colors<H, C, L, A, Color: Componentize<H, C, L, A>>(
    bottom: &Color,
    top: &Color,
    include_chroma: bool,
) -> Color {
    if include_chroma {
        let (top_h, top_c, _, _) = top.get_components();
        let (_, _, bottom_l, bottom_a) = bottom.get_components();

        return Color::from_components(top_h, top_c, bottom_l, bottom_a);
    } else {
        let (top_h, _, _, _) = top.get_components();
        let (_, bottom_c, bottom_l, bottom_a) = bottom.get_components();

        return Color::from_components(top_h, bottom_c, bottom_l, bottom_a);
    }
}

commandline::define_cli_enum!(GradientGeneratorType, {
    Discrete: ("discrete", "Where the colors are calculated by global and local position"),
    Continuous: ("continuous", "where palette generates it, taking into account all colors"),
});

struct GradientKeyFrame<'a, C>
where
    C: Mix<Scalar = ScalarType> + Sized,
{
    color: &'a C,
    position: ScalarType,
}

pub struct GradientDescriptor<C: Mix<Scalar = ScalarType> + Sized> {
    pub colors: vec::Vec<C>,
    pub positions: vec::Vec<ScalarType>,
}

impl<C: Mix<Scalar = ScalarType> + Sized + Clone> GradientDescriptor<C> {
    pub fn new(mut colors: vec::Vec<C>) -> GradientDescriptor<C> {
        colors.push(colors[0].clone());
        let rng = 0..colors.len();
        let length = std::cmp::max(colors.len() - 1, 1);
        return GradientDescriptor {
            colors,
            positions: rng
                .map(|i| (i as ScalarType) / (length as ScalarType))
                .collect(),
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
            let global_position = (i as ScalarType) / (frame_count as ScalarType);

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
        position: ScalarType,
    ) -> (GradientKeyFrame<'a, C>, GradientKeyFrame<'a, C>) {
        let base = 1.0 / ((self.colors.len() - 1) as ScalarType);
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
