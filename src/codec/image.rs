use std::error;
use std::io;
use std::marker::PhantomData;
use std::vec;

use super::{Decodable, DecodeError, Frame};
use crate::color;

use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use palette::FromColor;

pub struct ImageDecoder<C> {
    phantom: PhantomData<C>,
    image: DynamicImage,
}

impl<C> ImageDecoder<C>
where
    C: color::Color,
{
    pub fn new<R: io::BufRead + io::Seek>(
        read: R,
        format: ImageFormat,
    ) -> Result<Self, Box<dyn error::Error>> {
        let dec_impl = Reader::with_format(read, format);
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

        // let dec_impl = ImageDecoder {
        //     phantom: PhantomData,
        //     decoder: Reader::with_format(read, format),
        // };
        return Ok(ImageDecoder {
            phantom: PhantomData,
            image: decoded,
        });
    }
}

impl<C> Decodable for ImageDecoder<C>
where
    C: color::Color,
{
    type OutputColor = C;

    fn decode(&mut self) -> Result<Option<Frame<Self::OutputColor>>, Box<dyn error::Error>> {
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

        return Ok(Some(Frame{
            pixels,
            delay: 0,
            dispose: gif::DisposalMethod::Any,
            interlaced: false,
        }));
    }

    fn decode_all(
        &mut self,
    ) -> Result<Option<vec::Vec<Frame<Self::OutputColor>>>, Box<dyn error::Error>> {
        if let Some(frame) = self.decode()? {
        return Ok(Some(vec![frame]));
        }

        return Ok(None);
    }
}
