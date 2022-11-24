use std::cell::RefCell;
use std::error;
use std::fs::File;
use std::marker::PhantomData;
use std::rc::Rc;
use std::vec;

use super::Frame;
use crate::color;

use palette::FromColor;

pub struct GifDecoder<C> {
    phantom: PhantomData<C>,
    decoder: Rc<RefCell<gif::Decoder<File>>>,
    screen: gif_dispose::Screen,
}

impl<C> GifDecoder<C>
where
    C: color::Color,
{
    pub fn new<P: AsRef<std::path::Path> + std::fmt::Display>(path: P) -> Result<Self, String> {
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

    pub fn get_dimensions(&self) -> (u16, u16) {
        let dec_ref = self.decoder.borrow();
        return (dec_ref.width(), dec_ref.height());
    }
}

impl<C> IntoIterator for GifDecoder<C>
where
    C: color::Color,
{
    type Item = Frame<C>;
    type IntoIter = GifDecoderImpl<C>;

    fn into_iter(self) -> Self::IntoIter {
        return GifDecoderImpl { decoder: self };
    }
}

impl<C> super::Decodable for GifDecoder<C>
where
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

        if let Ok(_) = self.screen.blit_frame(&frame) {
            let mut pixels = vec::Vec::new();
            for pixel in self.screen.pixels.pixels() {
                pixels.push(C::from_color(color::ColorType::new(
                    (pixel.r as color::ScalarType) / 255.,
                    (pixel.g as color::ScalarType) / 255.,
                    (pixel.b as color::ScalarType) / 255.,
                    (pixel.a as color::ScalarType) / 255.,
                )));
            }

            return Ok(Some(Frame {
                pixels,
                delay: frame.delay,
                dispose: frame.dispose,
                interlaced: frame.interlaced,
            }));
        }

        return Err(Box::from("Could not blit frame"));
    }

    // TODO
    fn decode_all(&mut self) -> Result<Option<vec::Vec<Frame<C>>>, Box<dyn error::Error>> {
        return Ok(None);
    }
}

pub struct GifDecoderImpl<C> {
    decoder: GifDecoder<C>,
}

impl<C> Iterator for GifDecoderImpl<C>
where
    C: color::Color,
{
    type Item = Frame<C>;

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
                    pixels.push(C::from_color(color::ColorType::new(
                        (pixel.r as color::ScalarType) / 255.,
                        (pixel.g as color::ScalarType) / 255.,
                        (pixel.b as color::ScalarType) / 255.,
                        (pixel.a as color::ScalarType) / 255.,
                    )));
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

pub struct GifEncoder<C> {
    phantom: PhantomData<C>,
    encoder: RefCell<gif::Encoder<File>>,
    width: u16,
    height: u16,
}

impl<'a, C> GifEncoder<C>
where
    C: color::Color,
{
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
            phantom: PhantomData,
            encoder: encoder_impl,
            width,
            height,
        });
    }

    pub fn write(&self, frame: Frame<C>) -> Result<(), String>
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
            return Err(e.to_string());
        }

        return Ok(());
    }
}

impl<C> super::Encodable for GifEncoder<C>
where
    C: color::Color,
    palette::rgb::Rgb<palette::encoding::Srgb, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type InputColor = C;

    fn encode(&self, frame: Frame<C>) -> Result<(), String> {
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
