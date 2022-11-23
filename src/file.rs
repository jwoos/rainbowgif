use std::cell::RefCell;
use std::fs::File;
use std::rc::Rc;
use std::vec;

use crate::color;

use gif;
use gif_dispose;
use palette;
use palette::FromColor;

pub struct Frame<C>
where
    C: palette::FromColor<color::ColorType> + palette::IntoColor<color::ColorType>,
{
    pub pixels: vec::Vec<C>,
    pub delay: u16,
    pub dispose: gif::DisposalMethod,
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

pub struct GifDecoder {
    decoder: Rc<RefCell<gif::Decoder<File>>>,
    screen: gif_dispose::Screen,
}

impl GifDecoder {
    pub fn new<P: AsRef<std::path::Path> + std::fmt::Display>(
        path: P,
    ) -> Result<GifDecoder, String> {
        let mut decoder_options = gif::DecodeOptions::new();
        decoder_options.set_color_output(gif::ColorOutput::Indexed);

        let img = match File::open(&path) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}: {}", e.to_string(), path)),
        };

        let decoder = match decoder_options.read_info(img) {
            Ok(dec) => RefCell::new(dec),
            Err(e) => return Err(e.to_string()),
        };

        let rc_decoder = Rc::new(decoder);

        return Ok(GifDecoder {
            decoder: rc_decoder.clone(),
            screen: gif_dispose::Screen::new_decoder(&rc_decoder.borrow()),
        });
    }

    pub fn get_width(&self) -> u16 {
        return self.decoder.borrow().width();
    }

    pub fn get_height(&self) -> u16 {
        return self.decoder.borrow().height();
    }

    pub fn get_dimensions(&self) -> (u16, u16) {
        let dec = self.decoder.borrow();

        return (dec.width(), dec.height());
    }
}

impl IntoIterator for GifDecoder {
    type Item = Frame<color::ColorType>;
    type IntoIter = GifDecoderImpl;

    fn into_iter(self) -> Self::IntoIter {
        return GifDecoderImpl { decoder: self };
    }
}

pub struct GifDecoderImpl {
    decoder: GifDecoder,
}

impl Iterator for GifDecoderImpl {
    type Item = Frame<color::ColorType>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Some(frame) = self
            .decoder
            .decoder
            .borrow_mut()
            .read_next_frame()
            .unwrap_or(None)
        {
            if let Ok(_) = self.decoder.screen.blit_frame(&frame) {
                let mut pixels = vec::Vec::new();
                for pixel in self.decoder.screen.pixels.pixels() {
                    pixels.push(color::ColorType::new(
                        (pixel.r as f64) / 255.,
                        (pixel.g as f64) / 255.,
                        (pixel.b as f64) / 255.,
                        (pixel.a as f64) / 255.,
                    ));
                }

                return Some(Frame {
                    pixels,
                    delay: frame.delay,
                    dispose: frame.dispose,
                    interlaced: frame.interlaced,
                });
            }
        }
        return None;
    }
}

pub struct GifEncoder {
    encoder: RefCell<gif::Encoder<File>>,
    width: u16,
    height: u16,
}

impl<'a> GifEncoder {
    pub fn new<P: AsRef<std::path::Path> + std::fmt::Display>(
        path: P,
        (width, height): (u16, u16),
    ) -> Result<Self, String> {
        let file = match File::create(&path) {
            Ok(f) => f,
            Err(e) => return Err(format!("{}: {}", e.to_string(), path)),
        };

        let encoder_impl = match gif::Encoder::new(file, width, height, &[]) {
            Ok(enc) => RefCell::new(enc),
            Err(e) => return Err(e.to_string()),
        };

        if let Err(e) = encoder_impl.borrow_mut().set_repeat(gif::Repeat::Infinite) {
            return Err(e.to_string());
        }

        return Ok(GifEncoder {
            encoder: encoder_impl,
            width,
            height,
        });
    }

    pub fn write<C>(&self, frame: Frame<C>) -> Result<(), String>
    where
        C: palette::FromColor<color::ColorType>
            + palette::IntoColor<color::ColorType>
            + palette::WithAlpha<f64>,
        palette::rgb::Rgb<palette::encoding::Srgb, f64>:
            palette::convert::FromColorUnclamped<<C as palette::WithAlpha<f64>>::Color>,
    {
        let mut pixels = vec::Vec::new();
        for pixel in frame.pixels {
            let temp_pixel = color::ColorType::from_color(pixel);
            pixels.push((temp_pixel.color.red * 255.) as u8);
            pixels.push((temp_pixel.color.green * 255.) as u8);
            pixels.push((temp_pixel.color.blue * 255.) as u8);
            pixels.push((temp_pixel.alpha * 255.) as u8);
        }

        let mut new_frame = gif::Frame::from_rgba(self.width, self.height, &mut pixels);
        new_frame.delay = frame.delay;
        new_frame.dispose = frame.dispose;
        new_frame.interlaced = frame.interlaced;

        if let Err(e) = self.encoder.borrow_mut().write_frame(&new_frame) {
            return Err(e.to_string());
        }

        return Ok(());
    }
}

// impl<'a, C> FromIterator<Frame<C>> for GifEncoder<'a>
// where
//     C: palette::IntoColor<color::ColorType>
//         + palette::FromColor<color::ColorType>
//         + palette::WithAlpha<f64>,
//     palette::rgb::Rgb<palette::encoding::Srgb, f64>:
//         palette::convert::FromColorUnclamped<<C as palette::WithAlpha<f64>>::Color>,
// {
//     fn from_iter<T>(iter: T) -> Self
//     where
//         T: IntoIterator<Item = Frame<C>>,
//     {
//     }
// }
