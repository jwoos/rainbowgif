use std::fs::File;
use std::vec;

use clap::{arg, command, value_parser, ArgMatches};
use gif;
use gif_dispose;
use palette;
use palette::{FromColor, IntoColor};

mod color;
mod file;

fn main_impl<H, C, L, A, Color>(matches: ArgMatches) -> Result<(), Box<dyn std::error::Error>>
where
    Color: FromColor<color::ColorType>
        + IntoColor<color::ColorType>
        + color::Componentize<H, C, L, A>
        + palette::WithAlpha<f64>
        + palette::Mix<Scalar = f64>
        + Clone
        + Sized,
    palette::rgb::Rgb<palette::encoding::Srgb, f64>:
        palette::convert::FromColorUnclamped<<Color as palette::WithAlpha<f64>>::Color>,
{
    let color_strings = matches.get_many::<String>("colors").unwrap();
    let mut color_vec: vec::Vec<Color> = vec::Vec::new();
    for color_string in color_strings {
        if color_string.len() != 6 {
            return Err(format!("Invalid color format {}", &color_string).into());
        }

        match color::from_hex(&color_string) {
            Ok(c) => color_vec.push(c),
            Err(e) => return Err(format!("{}: {}", e.to_string(), color_string).into()),
        }
    }

    let src_image_path = matches.get_one::<String>("input_file").unwrap();
    let decoder = file::GifDecoder::new(src_image_path)?;
    let dest_image_path = matches.get_one::<String>("output_file").unwrap();
    let encoder = file::GifEncoder::new(dest_image_path, decoder.get_dimensions())?;

    // TODO: figure out either how to generate colors without knowing the frame count OR figure out
    // how to get the frame count while streaming the decoding process (not decoding everything at
    // once)
    let frames: vec::Vec<file::Frame<color::ColorType>> = Iterator::collect(decoder.into_iter());
    let gradient_desc = color::GradientDescriptor::new(color_vec);
    let generator_type = matches
        .get_one::<color::GradientGeneratorType>("generator")
        .unwrap()
        .to_owned();
    let colors = gradient_desc.generate(frames.len(), generator_type);
    for (i, frame) in frames.into_iter().enumerate() {
        let new_color = &colors[i];

        let mut frame_pixels = vec::Vec::new();
        for pixel in frame.pixels {
            // create new pixel using color space from input pixel (RGBA)
            let original_pixel = Color::from_color(pixel);

            // blend
            let blended_pixel = color::blend_colors(&original_pixel, new_color);

            // convert it back to rgba
            // let new_pixel =
            //     color::ColorType::from_color(blended_pixel);
            frame_pixels.push(blended_pixel);
        }

        encoder.write(file::Frame {
            pixels: frame_pixels,
            delay: frame.delay,
            dispose: frame.dispose,
            interlaced: frame.interlaced,
        })?;
    }

    return Ok(());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(arg!(input_file: <INPUT_FILE> "The path to the input file"))
        .arg(arg!(output_file: <OUTPUT_FILE> "The path to the output file"))
        .arg(
            arg!(colors: -c --colors <COLORS> "The colors to use in the gradient")
                .value_delimiter(',')
                .default_value("FF0000,00FF00,0000FF")
                .help("The colors to use in the gradient"),
        )
        .arg(
            arg!(generator: -g --generator <GENERATOR> "The generator to use")
                .value_parser(value_parser!(color::GradientGeneratorType))
                .default_value("discrete")
                .help("The type of generator to use"),
        )
        .arg(
            arg!(color_space: -s --color_space <COLOR_SPACE> "The color space to use")
                .value_parser(value_parser!(color::ColorSpace))
                .default_value("lch")
                .help("The type of color space to use"),
        )
        .get_matches();

    match matches.get_one::<color::ColorSpace>("color_space").unwrap() {
        color::ColorSpace::HSL => main_impl::<
            palette::RgbHue<f64>,
            f64,
            f64,
            f64,
            palette::Hsla<palette::encoding::srgb::Srgb, f64>,
        >(matches),
        color::ColorSpace::HSV => main_impl::<
            palette::RgbHue<f64>,
            f64,
            f64,
            f64,
            palette::Hsva<palette::encoding::srgb::Srgb, f64>,
        >(matches),
        color::ColorSpace::LCH => main_impl::<
            palette::LabHue<f64>,
            f64,
            f64,
            f64,
            palette::Lcha<palette::white_point::D65, f64>,
        >(matches),
    }
}
