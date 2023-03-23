use std::error;
use std::vec;

use imagequant;

pub enum Quantizer {
    IMAGEQUANT,
}

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

pub fn quantize(
    quantizer: Quantizer,
    img: vec::Vec<(u8, u8, u8, u8)>,
    dimensions: (usize, usize),
) -> Result<(vec::Vec<(u8, u8, u8, u8)>, vec::Vec<u8>), Box<dyn error::Error>> {
    return match quantizer {
        Quantizer::IMAGEQUANT => quantize_image_quant(img, dimensions),
    };
}
