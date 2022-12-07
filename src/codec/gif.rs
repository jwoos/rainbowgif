use std::cell::RefCell;
use std::error;
use std::io;
use std::marker::PhantomData;
use std::rc::Rc;
use std::vec;

use super::{Decodable, DecodeError, EncodeError, Frame};
use crate::color;

use palette::FromColor;

pub struct GifDecoder<R: io::Read, C> {
    phantom: PhantomData<C>,
    decoder: Rc<RefCell<gif::Decoder<R>>>,
    screen: gif_dispose::Screen,
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
            Ok(dec) => RefCell::new(dec),
            Err(e) => {
                return Err(Box::new(DecodeError::Read(
                    Some(Box::new(e)),
                    "Could not read image".to_owned(),
                )));
            }
        };

        let rc_decoder = Rc::new(decoder);

        return Ok(GifDecoder {
            phantom: PhantomData,
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
}

impl<C, R> IntoIterator for GifDecoder<R, C>
where
    R: io::Read,
    C: color::Color,
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
{
    type OutputColor = C;

    fn decode(&mut self) -> Result<Option<Frame<C>>, Box<dyn error::Error>> {
        let mut dec_ref = self.decoder.borrow_mut();
        let frame = match dec_ref.read_next_frame() {
            Ok(f) => {
                if let Some(f_result) = f {
                    f_result
                } else {
                    return Ok(None);
                }
            }
            Err(e) => return Err(Box::new(e)),
        };

        return match self.screen.blit_frame(&frame) {
            Ok(_) => {
                let mut pixels = vec::Vec::new();
                for pixel in self.screen.pixels.pixels() {
                    pixels.push(C::from_color(color::ColorType::new(
                        (pixel.r as color::ScalarType) / 255.,
                        (pixel.g as color::ScalarType) / 255.,
                        (pixel.b as color::ScalarType) / 255.,
                        (pixel.a as color::ScalarType) / 255.,
                    )));
                }

                Ok(Some(Frame {
                    pixels,
                    delay: frame.delay,
                    dispose: frame.dispose,
                    interlaced: frame.interlaced,
                }))
            }
            Err(e) => Err(Box::new(DecodeError::FrameRead(
                Some(Box::new(e)),
                "Could not blit frame".to_owned(),
            ))),
        };
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
        let dec_ref = self.decoder.borrow();
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
{
    type Item = Frame<C>;

    fn next(&mut self) -> Option<Self::Item> {
        if let Ok(res) = self.decoder.decode() {
            return res;
        }

        return None;
    }
}

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
        palette::rgb::Rgb<palette::encoding::Srgb, color::ScalarType>:
            palette::convert::FromColorUnclamped<
                <C as palette::WithAlpha<color::ScalarType>>::Color,
            >,
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
    palette::rgb::Rgb<palette::encoding::Srgb, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type InputColor = C;

    fn encode(&self, frame: Frame<C>) -> Result<(), Box<dyn error::Error>> {
        // TODO probably should have own error type
        return self.write(frame);
    }

    // TODO
    fn encode_all(&self, frames: vec::Vec<Frame<C>>) -> Result<(), Box<dyn error::Error>> {
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
