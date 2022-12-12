use std::error;
use std::fmt;
use std::vec;

use crate::{color, error_utils};

use ::gif as gif_lib;

pub mod gif;
pub mod image;

error_utils::define_error!(DecodeError, {
    Init: "Error initializing decoder",
    Read: "Error reading data",
    FrameRead: "Error reading frame",
});

error_utils::define_error!(EncodeError, {
    Init: "Error initializing encoder",
    Write: "Error write data",
    FrameWrite: "Error write frame",
});

pub struct Frame<C>
where
    C: color::Color,
{
    pub pixels: vec::Vec<C>,
    pub delay: u16,
    pub dispose: gif_lib::DisposalMethod,
    pub interlaced: bool,
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
