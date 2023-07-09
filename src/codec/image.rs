use std::collections;
use std::error;
use std::io;
use std::marker;
use std::vec;

use ::gif as gif_lib;
use image::io::Reader;
use image::{DynamicImage, ImageFormat};
use palette;

use super::{Decodable, DecodeError, Frame};
use crate::codec;
use crate::color;

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
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    type OutputColor = C;

    fn decode(&mut self) -> Result<Option<Frame<Self::OutputColor>>, Box<dyn error::Error>> {
        if self.decoded {
            return Ok(None);
        }

        let mut buf = self.image.to_rgba8();
        let mut transparent_indices = collections::hash_set::HashSet::new();

        // in a gif if a value isn't fully opaque, it's considered transparent
        for (i, mut pixel) in buf.pixels_mut().enumerate() {
            if pixel.0[3] != 255 {
                transparent_indices.insert(i);
                pixel.0[0] = 0;
                pixel.0[1] = 0;
                pixel.0[2] = 0;
                pixel.0[3] = 0;
            }
        }

        let (pixels, indices) = color::quantize::quantize(
            color::quantize::QuantizerType::IMAGEQUANT,
            buf.into_vec()[..]
                .chunks(4)
                .map(|chunk| {
                    let [r, g, b, a]: [u8; 4] = chunk.try_into().unwrap();
                    return (r, g, b, a);
                })
                .collect(),
            (self.image.width() as usize, self.image.height() as usize),
        )?;

        let transparent_index = pixels.iter().position(|&x| x.3 == 0).map(|i| i as u8);

        let quantized_pixels = pixels
            .into_iter()
            .map(|pixel| {
                return C::from_color(color::ColorType::new(
                    (pixel.0 as color::ScalarType) / 255.,
                    (pixel.1 as color::ScalarType) / 255.,
                    (pixel.2 as color::ScalarType) / 255.,
                    (pixel.3 as color::ScalarType) / 255.,
                ));
            })
            .collect::<vec::Vec<_>>();

        let pal = codec::Palette::new(quantized_pixels);

        self.decoded = true;

        return Ok(Some(Frame {
            delay: 0, // TODO #35
            dispose: gif_lib::DisposalMethod::Keep,
            origin: (0, 0),
            dimensions: self.get_dimensions(),
            palette: pal,
            pixels_indexed: indices,
            transparent_index,
            interlaced: false,
            needs_input: false,
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
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
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
