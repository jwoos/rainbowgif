use std::collections::hash_map;
use std::error;
use std::vec;

use imagequant;

crate::error_utils::define_error!(
    QuantizeError, {
        InvalidType: "The given quantizer is not a valid one",
    }
);

#[derive(Debug)]
pub enum QuantizerType {
    IDENTITY,
    IMAGEQUANT,
    SCALAR,
}

pub struct Quantizer {
    pub max_color_count: usize,
    pub quantizer_type: QuantizerType,
}

impl Quantizer {
    pub fn new(max_color_count: usize, quantizer_type: QuantizerType) -> Self {
        return Quantizer {
            max_color_count,
            quantizer_type,
        };
    }

    pub fn run(
        &self,
        img: vec::Vec<(u8, u8, u8, u8)>,
        dimensions: (usize, usize),
    ) -> Result<(vec::Vec<(u8, u8, u8, u8)>, vec::Vec<u8>), Box<dyn error::Error>> {
        return match self.quantizer_type {
            QuantizerType::IDENTITY => quantize_identity(img, dimensions),
            QuantizerType::IMAGEQUANT => quantize_image_quant(img, dimensions),
            _ => Err(Box::new(QuantizeError::InvalidType(
                None,
                format!("Invalid quantizer {:?}", self.quantizer_type),
            ))),
        };
    }
}

/* helper method to just transform input to the appropriate output.
 * Generates a proper palette from any given input. size is not taken into consideration and it's
 * the caller's responsibility to adjust as needed.
 */
pub fn quantize_identity(
    img: vec::Vec<(u8, u8, u8, u8)>,
    dimensions: (usize, usize),
) -> Result<(vec::Vec<(u8, u8, u8, u8)>, vec::Vec<usize>), Box<dyn error::Error>> {
    let mut color_map: hash_map::HashMap<(u8, u8, u8, u8), usize> = hash_map::HashMap::new();
    let mut palette_list = vec::Vec::new();
    let mut indexed_pixels = vec::Vec::new();
    indexed_pixels.resize(dimensions.0 * dimensions.1, 0);
    for (i, color) in img.into_iter().enumerate() {
        if !color_map.contains_key(&color) {
            palette_list.push(color.clone());
            color_map.insert(color, palette_list.len() - 1);
        }

        indexed_pixels[i] = *color_map.get(&color).unwrap();
    }

    return Ok((palette_list, indexed_pixels));
}

// pub fn quantize_scalar(
//     img: vec::Vec<(u8, u8, u8, u8)>
//                       )

pub fn quantize_image_quant(
    img: vec::Vec<(u8, u8, u8, u8)>,
    dimensions: (usize, usize),
) -> Result<(vec::Vec<(u8, u8, u8, u8)>, vec::Vec<u8>), Box<dyn error::Error>> {
    let mut liq = imagequant::new();
    liq.set_speed(5)?;
    liq.set_quality(0, 100)?;

    let ref mut img = liq.new_image(
        img.into_iter().map(|e| e.into()).collect::<vec::Vec<_>>(),
        dimensions.0,
        dimensions.1,
        0.0,
    )?;

    let mut res = liq.quantize(img)?;
    res.set_dithering_level(1.0)?;

    let (palette, pixels) = res.remapped(img).unwrap();

    Ok((
        palette
            .into_iter()
            .map(|pixel| {
                return (pixel.r, pixel.g, pixel.b, pixel.a);
            })
            .collect(),
        pixels,
    ))
}
