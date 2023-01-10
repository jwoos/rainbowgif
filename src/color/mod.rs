use clap::{builder::PossibleValue, ValueEnum};
use palette::{Clamp, FromColor, Hsla, Hsva, LabHue, Lcha, RgbHue};

use crate::commandline;

pub mod gradient;
pub mod quantize;

pub type ScalarType = f32;
// TODO put behind a feature
// pub type ScalarType = f64;

// TODO put behind a feature
// pub type ColorType = palette::rgb::GammaSrgba<ScalarType>;
// pub type ColorType = palette::rgb::LinSrgba<ScalarType>;
pub type ColorType = palette::rgb::Srgba<ScalarType>;

// pub type EncodingType = palette::encoding::Gamma<palette::encoding::Srgb>;
// pub type EncodingType = palette::encoding::linear::Linear<palette::encoding::Srgb>;
pub type EncodingType = palette::encoding::Srgb;

pub type WhitePoint = palette::white_point::D65;

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
    None: ("none", "Doesn't actually mix and returns the original colors"),
    Custom: ("custom", "Mixes the color by taking the hue component of the other color, keeping the base luma and chroma"),
    Lab: ("lab", "Mixes the color by taking the color components of the other color, keeping the base lightness"),
    Linear: ("linear", "Uses palettee for linear mixing"),
    BlendOverlay: ("blend_overlay", "Uses blending: overlay"),
});

commandline::define_cli_enum!(ColorSpace, {
    HSL: (
        "hsl",
        "The HSL color space can be seen as a cylindrical version of RGB, where the hue is the angle around the color cylinder, the saturation is the distance from the center, and the lightness is the height from the bottom."
    ),
    HSV: (
        "hsv",
        "HSV is a cylindrical version of RGB and it’s very similar to HSL. The difference is that the value component in HSV determines the brightness of the color, and not the lightness."
    ),
    LCH: (
        "lch",
        "L*C*h° shares its range and perceptual uniformity with L*a*b*, but it’s a cylindrical color space, like HSL and HSV. This gives it the same ability to directly change the hue and colorfulness of a color, while preserving other visual aspects."
    ),
    RGB: (
        "rgb",
        "RGB"
    ),
    LAB: (
        "lab",
        "The CIE L*a*b* (CIELAB) color space"
    )
});

pub fn from_hex<C>(color_string: &str) -> Result<C, std::num::ParseIntError>
where
    C: FromColor<ColorType>,
{
    let r = u8::from_str_radix(&color_string[0..2], 16)? as ScalarType;
    let g = u8::from_str_radix(&color_string[2..4], 16)? as ScalarType;
    let b = u8::from_str_radix(&color_string[4..6], 16)? as ScalarType;

    // expects values in (0, 1)
    let temp_color = ColorType::new(r / 255.0, g / 255.0, b / 255.0, 1.0);
    return Ok(C::from_color(temp_color));
}

pub trait Componentize<H, C, L, A> {
    fn get_components(&self) -> (H, C, L, A);

    fn from_components(h: H, c: C, l: L, a: A) -> Self;
}

impl Componentize<LabHue<ScalarType>, ScalarType, ScalarType, ScalarType>
    for Lcha<WhitePoint, ScalarType>
{
    fn get_components(&self) -> (LabHue<ScalarType>, ScalarType, ScalarType, ScalarType) {
        let (l, c, h, a) = self.into_components();
        return (h, c, l, a);
    }

    fn from_components(h: LabHue<ScalarType>, c: ScalarType, l: ScalarType, a: ScalarType) -> Self {
        return Lcha::from_components((l, c, h, a)).clamp();
    }
}

impl Componentize<RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType>
    for Hsla<EncodingType, ScalarType>
{
    fn get_components(&self) -> (RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType) {
        return self.into_components();
    }

    fn from_components(h: RgbHue<ScalarType>, c: ScalarType, l: ScalarType, a: ScalarType) -> Self {
        return Hsla::from_components((h, c, l, a)).clamp();
    }
}

impl Componentize<RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType>
    for Hsva<EncodingType, ScalarType>
{
    fn get_components(&self) -> (RgbHue<ScalarType>, ScalarType, ScalarType, ScalarType) {
        return self.into_components();
    }

    fn from_components(h: RgbHue<ScalarType>, c: ScalarType, l: ScalarType, a: ScalarType) -> Self {
        return Hsva::from_components((h, c, l, a)).clamp();
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
