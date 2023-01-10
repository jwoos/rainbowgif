use std::error;
use std::vec;

use ::gif as gif_lib;
use palette::FromColor;

use crate::{color, error_utils};

pub mod gif;
pub mod image;

error_utils::define_error!(DecodeError, {
    Init: "Error initializing decoder",
    Read: "Error reading data",
    FrameRead: "Error reading frame",
    InvalidData: "Invalid data",
});

error_utils::define_error!(EncodeError, {
    Init: "Error initializing encoder",
    Write: "Error write data",
    FrameWrite: "Error write frame",
});

// TODO make private after iterable
#[derive(Clone)]
pub struct Palette<C> {
    pub colors: vec::Vec<C>,
}

// TODO implement IntoIterator and Iterator trait
impl<C> Palette<C>
where
    C: color::Color,
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    pub fn new(colors: vec::Vec<C>) -> Self {
        assert!(
            colors.len() <= 256,
            "Expected <= 255 but got {}",
            colors.len()
        );

        return Palette { colors };
    }

    pub fn from_gif_format(colors: &[u8]) -> Self {
        return Palette {
            colors: colors[..]
                .chunks(3)
                .map(|chunk| {
                    let [r, g, b]: [u8; 3] = chunk.try_into().unwrap();
                    return C::from_color(color::ColorType::new(
                        r as color::ScalarType / 255.,
                        g as color::ScalarType / 255.,
                        b as color::ScalarType / 255.,
                        1.,
                    ));
                })
                .collect(),
        };
    }

    #[allow(dead_code)]
    pub fn into_gif_format(self) -> vec::Vec<u8> {
        return self
            .colors
            .into_iter()
            .flat_map(|c| {
                let c_prime = color::ColorType::from_color(c);
                return [
                    (c_prime.red * 255.) as u8,
                    (c_prime.green * 255.) as u8,
                    (c_prime.blue * 255.) as u8,
                ];
            })
            .collect();
    }
}

#[derive(Clone)]
pub struct Frame<C>
where
    C: color::Color,
{
    pub delay: u16,
    pub dispose: gif_lib::DisposalMethod,

    // sometimes the frame doesn't fill the whole screen
    pub origin: (u16, u16),
    pub dimensions: (u16, u16),

    // local palette, limited to 255 colors
    // while GIFs technically can have a global palette, this is easier
    // if it has no local palette, it'll take the global palette
    // TODO look into maybe only using global palette
    pub palette: Palette<C>,

    // pixels indexing into palette (local if present, otherwise global)
    pub pixels_indexed: vec::Vec<u8>,

    // transparent pixel index, if available
    pub transparent_index: Option<u8>,

    // these are rarely used
    pub interlaced: bool,
    pub needs_input: bool,
}

// TODO: figure this out
// impl<C, T> Frame<C>
// where
//     C: palette::FromColor<color::ColorType> + palette::IntoColor<color::ColorType>,
//     T: palette::FromColor<C> + palette::IntoColor<C>,
// {
//     pub fn to_colorspace(self, color_space: color::ColorSpace) -> Frame<T> {
//         return Frame {
//             pixels: self.pixels.into_iter().map(|pixel| { return T::from_color(pixel);}).collect(),
//             delay: self.delay,
//             dispose: self.dispose,
//             interlaced: self.interlaced,
//         };
//     }
// }

pub trait Decodable
where
    <Self as Decodable>::OutputColor: color::Color,
{
    type OutputColor;

    fn decode(&mut self) -> Result<Option<Frame<Self::OutputColor>>, Box<dyn error::Error>>;

    fn decode_all(
        &mut self,
    ) -> Result<Option<vec::Vec<Frame<Self::OutputColor>>>, Box<dyn error::Error>>;

    fn get_dimensions(&self) -> (u16, u16);
}

// can't use FromIterator as a super trait, as it requires more than just an iterator to encode all
// the data
pub trait Encodable
where
    <Self as Encodable>::InputColor: color::Color,
{
    type InputColor;

    fn encode(&self, frame: Frame<Self::InputColor>) -> Result<(), Box<dyn error::Error>>;

    fn encode_all(
        &self,
        frames: vec::Vec<Frame<Self::InputColor>>,
    ) -> Result<(), Box<dyn error::Error>>;
}
