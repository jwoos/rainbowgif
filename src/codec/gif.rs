use std::cell::RefCell;
use std::error;
use std::io;
use std::marker::PhantomData;
use std::vec;

use super::{Decodable, DecodeError, EncodeError, Frame, Palette};
use crate::color;

use palette::FromColor;

pub struct GifDecoder<R: io::Read, C> {
    phantom: PhantomData<C>,
    decoder: gif::Decoder<R>,
}

impl<R, C> GifDecoder<R, C>
where
    R: io::Read,
    C: color::Color,
{
    pub fn new(read: R) -> Result<Self, Box<dyn error::Error>> {
        let mut decoder_options = gif::DecodeOptions::new();
        decoder_options.set_color_output(gif::ColorOutput::Indexed);

        let decoder = match decoder_options.read_info(read) {
            Ok(dec) => dec,
            Err(e) => {
                return Err(Box::new(DecodeError::Read(
                    Some(Box::new(e)),
                    "Could not read image".to_owned(),
                )));
            }
        };

        return Ok(GifDecoder {
            phantom: PhantomData,
            decoder,
        });
    }

    #[allow(dead_code)]
    pub fn get_width(&self) -> u16 {
        return self.decoder.width();
    }

    #[allow(dead_code)]
    pub fn get_height(&self) -> u16 {
        return self.decoder.height();
    }
}

impl<C, R> IntoIterator for GifDecoder<R, C>
where
    R: io::Read,
    C: color::Color,
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type Item = Frame<C>;
    type IntoIter = GifDecoderImpl<R, C>;

    fn into_iter(self) -> Self::IntoIter {
        return GifDecoderImpl { decoder: self };
    }
}

impl<R, C> super::Decodable for GifDecoder<R, C>
where
    R: io::Read,
    C: color::Color,
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type OutputColor = C;

    fn decode(&mut self) -> Result<Option<Frame<C>>, Box<dyn error::Error>> {
        let global_pal = self
            .decoder
            .global_palette()
            .map(|e| e.to_vec())
            .unwrap_or(vec::Vec::new());
        let frame = match self.decoder.read_next_frame() {
            Ok(f) => {
                if let Some(f_result) = f {
                    f_result
                } else {
                    return Ok(None);
                }
            }
            Err(e) => return Err(Box::new(e)),
        };

        let pal = if let Some(pal) = &frame.palette {
            Palette::<C>::from_gif_format(&pal[..])
        } else {
            if global_pal.len() == 0 {
                return Err(Box::new(DecodeError::InvalidData(
                    None,
                    "Frame had no valid global palette to fall back to".to_owned(),
                )));
            }

            Palette::from_gif_format(&global_pal[..])
        };

        // We don't need to deal with disposal and such since it'll just be encoded back into a gif
        return Ok(Some(Frame {
            delay: frame.delay,
            dispose: frame.dispose,
            origin: (frame.left, frame.top),
            dimensions: (frame.width, frame.height),
            palette: (pal),
            pixels_indexed: frame.buffer.to_vec(),
            transparent_index: frame.transparent,
            interlaced: frame.interlaced,
            needs_input: frame.needs_user_input,
        }));
    }

    fn decode_all(&mut self) -> Result<Option<vec::Vec<Frame<C>>>, Box<dyn error::Error>> {
        let mut frames = vec::Vec::new();

        loop {
            let opt = match self.decode() {
                Ok(opt) => opt,
                Err(e) => return Err(e),
            };

            if let Some(frame) = opt {
                frames.push(frame);
            } else {
                break;
            }
        }

        if frames.len() != 0 {
            return Ok(Some(frames));
        }

        return Ok(None);
    }

    fn get_dimensions(&self) -> (u16, u16) {
        let dec_ref = &self.decoder;
        return (dec_ref.width(), dec_ref.height());
    }
}

pub struct GifDecoderImpl<R: io::Read, C> {
    decoder: GifDecoder<R, C>,
}

impl<C, R> Iterator for GifDecoderImpl<R, C>
where
    R: io::Read,
    C: color::Color,
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type Item = Frame<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(res) = self.decoder.decode() {
            return res;
        }

        return None;
    }
}

#[allow(dead_code)]
pub struct GifEncoder<W: io::Write, C> {
    phantom: PhantomData<C>,
    encoder: RefCell<gif::Encoder<W>>,
    width: u16,
    height: u16,
}

impl<'a, W, C> GifEncoder<W, C>
where
    W: io::Write,
    C: color::Color,
{
    pub fn new(w: W, (width, height): (u16, u16)) -> Result<Self, String> {
        let encoder_impl = match gif::Encoder::new(w, width, height, &[]) {
            Ok(enc) => RefCell::new(enc),
            Err(e) => return Err(e.to_string()),
        };

        if let Err(e) = encoder_impl.borrow_mut().set_repeat(gif::Repeat::Infinite) {
            return Err(e.to_string());
        }

        return Ok(GifEncoder {
            phantom: PhantomData,
            encoder: encoder_impl,
            width,
            height,
        });
    }

    pub fn write(&self, frame: Frame<C>) -> Result<(), Box<dyn error::Error>>
    where
        C: color::Color,
        palette::rgb::Rgb<color::EncodingType, color::ScalarType>:
            palette::convert::FromColorUnclamped<
                <C as palette::WithAlpha<color::ScalarType>>::Color,
            >,
    {
        let pal = frame
            .palette
            .colors
            .into_iter()
            .flat_map(|c| {
                let temp_color = color::ColorType::from_color(c);
                return [
                    (temp_color.color.red * 255.) as u8,
                    (temp_color.color.green * 255.) as u8,
                    (temp_color.color.blue * 255.) as u8,
                ];
            })
            .collect::<Vec<_>>();

        let mut new_frame = gif::Frame::from_palette_pixels(
            frame.dimensions.0,
            frame.dimensions.1,
            &&frame.pixels_indexed[..],
            &pal[..],
            frame.transparent_index,
        );
        new_frame.left = frame.origin.0;
        new_frame.top = frame.origin.1;
        new_frame.delay = frame.delay;
        new_frame.dispose = frame.dispose;
        new_frame.interlaced = frame.interlaced;
        new_frame.needs_user_input = frame.needs_input;

        if let Err(e) = self.encoder.borrow_mut().write_frame(&new_frame) {
            return Err(Box::new(EncodeError::FrameWrite(
                Some(Box::new(e)),
                "write_frame errored".to_owned(),
            )));
        }

        return Ok(());
    }

    pub fn into_inner(self) -> Result<W, io::Error> {
        return self.encoder.into_inner().into_inner();
    }
}

impl<W, C> super::Encodable for GifEncoder<W, C>
where
    W: io::Write,
    C: color::Color,
    palette::rgb::Rgb<color::EncodingType, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type InputColor = C;

    fn encode(&self, frame: Frame<C>) -> Result<(), Box<dyn error::Error>> {
        // TODO probably should have own error type
        return self.write(frame);
    }

    fn encode_all(&self, frames: vec::Vec<Frame<C>>) -> Result<(), Box<dyn error::Error>> {
        for frame in frames {
            self.write(frame)?;
        }
        return Ok(());
    }
}

/* impl<C> FromIterator<Frame<C>> for GifEncoder
where
    C: color::Color + palette::WithAlpha<color::ScalarType>,
    palette::rgb::Rgb<palette::encoding::Srgb, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    fn from_iter<T>(iter: T) -> Self
    where
        T: IntoIterator<Item = Frame<C>>,
    {
        return GifEncoder {};
    }
} */
