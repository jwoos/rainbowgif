use std::error;
use std::fmt;
use std::vec;

use crate::color;

use ::gif as gif_lib;

pub mod gif;

#[derive(Debug)]
pub enum DecodeError {
    Init(Option<Box<dyn error::Error + 'static>>, String),
    Read(Option<Box<dyn error::Error + 'static>>, String),
    FrameRead(Option<Box<dyn error::Error + 'static>>, String),
}

impl fmt::Display for DecodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Init(_, desc) => {
                f.write_str(format!("Error initializing decoder: {desc}").as_str())
            }
            Self::Read(_, desc) => f.write_str(format!("Error reading data: {desc}").as_str()),
            Self::FrameRead(_, desc) => {
                f.write_str(format!("Error reading frame: {desc}").as_str())
            }
        };
    }
}

impl error::Error for DecodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        return match self {
            Self::Init(src, _) => src.as_ref().map(|e| e.as_ref()),
            Self::Read(src, _) => src.as_ref().map(|e| e.as_ref()),
            Self::FrameRead(src, _) => src.as_ref().map(|e| e.as_ref()),
        };
    }
}

#[derive(Debug)]
pub enum EncodeError {
    Init(Option<Box<dyn error::Error + 'static>>, String),
    Write(Option<Box<dyn error::Error + 'static>>, String),
    FrameWrite(Option<Box<dyn error::Error + 'static>>, String),
}

impl fmt::Display for EncodeError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        return match self {
            Self::Init(_, desc) => f.write_str(format!("Error initializing: {desc}").as_str()),
            Self::Write(_, desc) => f.write_str(format!("Error write data: {desc}").as_str()),
            Self::FrameWrite(_, desc) => {
                f.write_str(format!("Error writing frame: {desc}").as_str())
            }
        };
    }
}

impl error::Error for EncodeError {
    fn source(&self) -> Option<&(dyn error::Error + 'static)> {
        return match self {
            Self::Init(src, _) => src.as_ref().map(|e| e.as_ref()),
            Self::Write(src, _) => src.as_ref().map(|e| e.as_ref()),
            Self::FrameWrite(src, _) => src.as_ref().map(|e| e.as_ref()),
        };
    }
}

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

pub trait Decodable: IntoIterator<Item = Frame<Self::OutputColor>>
where
    <Self as Decodable>::OutputColor: color::Color,
{
    type OutputColor;

    fn decode(&mut self) -> Result<Option<Frame<Self::OutputColor>>, Box<dyn error::Error>>;

    fn decode_all(
        &mut self,
    ) -> Result<Option<vec::Vec<Frame<Self::OutputColor>>>, Box<dyn error::Error>>;
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
