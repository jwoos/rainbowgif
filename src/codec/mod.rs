pub mod gif;

use std::vec;

use crate::color;

use ::gif as gif_lib;

pub struct Frame<C>
where
    C: palette::FromColor<color::ColorType> + palette::IntoColor<color::ColorType>,
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

pub trait Decodable: IntoIterator {
    fn get_frames() -> dyn IntoIterator<Item = vec::Vec<u8>, IntoIter = vec::IntoIter<vec::Vec<u8>>>;
}

pub trait Encodable: FromIterator<vec::Vec<u8>> {
    fn write_frame(&mut self, frame: vec::Vec<u8>) -> ();
}
