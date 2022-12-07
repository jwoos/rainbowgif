use std::error;
use std::io;
use std::marker;
use std::vec;

use super::{Decodable, DecodeError, Frame};
use crate::color;

use image::io::Reader;
use image::{DynamicImage, ImageFormat};

pub struct ImageDecoder<C> {
    phantom: marker::PhantomData<C>,
    image: DynamicImage,
    decoded: bool,
}

impl<C> ImageDecoder<C>
where
    C: color::Color,
{
    fn new_impl<R: io::BufRead + io::Seek>(
        dec_impl: Reader<R>,
    ) -> Result<Self, Box<dyn error::Error>> {
        if dec_impl.format().is_none() {
            return Err(Box::new(DecodeError::Read(
                None,
                "Passed in Read doesn't yield valid format data".to_owned(),
            )));
        }

        let decoded = match dec_impl.decode() {
            Ok(frame) => frame,
            Err(e) => {
                return Err(Box::new(DecodeError::Read(
                    Some(Box::new(e)),
                    "Unable to decode properly".to_owned(),
                )))
            }
        };

        return Ok(ImageDecoder {
            phantom: marker::PhantomData,
            image: decoded,
            decoded: false,
        });
    }

    pub fn new<R: io::BufRead + io::Seek>(
        read: R,
        format: Option<ImageFormat>,
    ) -> Result<Self, Box<dyn error::Error>> {
        let dec_impl = {
            if let Some(image_format) = format {
                Reader::with_format(read, image_format)
            } else {
                Reader::new(read).with_guessed_format()?
            }
        };
        return Self::new_impl(dec_impl);
    }
}

impl<C> Decodable for ImageDecoder<C>
where
    C: color::Color,
{
    type OutputColor = C;

    fn decode(&mut self) -> Result<Option<Frame<Self::OutputColor>>, Box<dyn error::Error>> {
        if self.decoded {
            return Ok(None);
        }

        let img = self.image.clone();
        let buf = img.into_rgba32f();

        let mut pixels = vec::Vec::new();
        for (_, _, pixel) in buf.enumerate_pixels() {
            pixels.push(C::from_color(color::ColorType::new(
                pixel.0[0] as f64,
                pixel.0[1] as f64,
                pixel.0[2] as f64,
                pixel.0[3] as f64,
            )));
        }

        self.decoded = true;

        return Ok(Some(Frame {
            pixels,
            delay: 0,
            dispose: gif::DisposalMethod::Any,
            interlaced: false,
        }));
    }

    fn decode_all(
        &mut self,
    ) -> Result<Option<vec::Vec<Frame<Self::OutputColor>>>, Box<dyn error::Error>> {
        if self.decoded {
            return Ok(None);
        }

        if let Some(frame) = self.decode()? {
            return Ok(Some(vec![frame]));
        }

        return Ok(None);
    }

    fn get_dimensions(&self) -> (u16, u16) {
        return (self.image.width() as u16, self.image.height() as u16);
    }
}

impl<C> IntoIterator for ImageDecoder<C>
where
    C: color::Color,
{
    type Item = Frame<C>;
    type IntoIter = ImageDecoderIter<C>;

    fn into_iter(self) -> Self::IntoIter {
        return ImageDecoderIter { decoder: self };
    }
}

pub struct ImageDecoderIter<C> {
    decoder: ImageDecoder<C>,
}

impl<C> Iterator for ImageDecoderIter<C>
where
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
