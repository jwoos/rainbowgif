use std::cmp;
use std::error;
use std::fmt;
use std::fs;
use std::vec;

use clap::{arg, command, value_parser, ArgMatches};
use palette;

mod buffer;
mod codec;
mod color;
mod commandline;

fn main_impl<H, C, L, A, Color>(matches: ArgMatches) -> Result<(), Box<dyn error::Error>>
where
    Color: color::Color
        + color::Componentize<H, C, L, A>
        + palette::Mix<Scalar = color::ScalarType>
        + fmt::Debug
        + Clone
        + Sized,
    palette::rgb::Rgb<palette::encoding::Srgb, color::ScalarType>:
        palette::convert::FromColorUnclamped<
            <Color as palette::WithAlpha<color::ScalarType>>::Color,
        >,
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
    let src_data = buffer::Data::from_path(src_image_path)?;
    let mut decoder: Box<dyn codec::Decodable<OutputColor = Color>> = {
        if matches.get_flag("static") {
            Box::new(codec::image::ImageDecoder::new(src_data.buffer, None)?)
        } else {
            Box::new(codec::gif::GifDecoder::new(src_data.buffer)?)
        }
    };

    let dest_image_path = matches.get_one::<String>("output_file").unwrap();
    let mut dest_data = buffer::Data::new();
    let encoder = codec::gif::GifEncoder::new(dest_data.buffer, decoder.get_dimensions())?;

    let loop_count = cmp::max(
        matches.get_one::<u64>("loop_count").unwrap().to_owned() as usize,
        1usize,
    );
    // TODO: figure out either how to generate colors without knowing the frame count OR figure out
    // how to get the frame count while streaming the decoding process (not decoding everything at
    // once)

    // automatically transform to the specified color space in the decoder
    let frames: vec::Vec<_> = decoder.decode_all()?.unwrap();
    let frames_len = frames.len();
    let gradient_desc = color::GradientDescriptor::new(color_vec);
    let generator_type = matches
        .get_one::<color::GradientGeneratorType>("generator")
        .unwrap()
        .to_owned();
    let colors = gradient_desc.generate(frames_len * loop_count, generator_type);

    for l in 0usize..loop_count {
        for (i, frame) in frames.iter().enumerate() {
            let new_color = &colors[i + (frames_len * l)];

            let frame_pixels = frame
                .pixels
                .iter()
                .map(|pixel| {
                    return color::blend_colors(pixel, new_color);
                })
                .collect();

            encoder.write(codec::Frame {
                pixels: frame_pixels,
                delay: frame.delay,
                dispose: frame.dispose,
                interlaced: frame.interlaced,
            })?;
        }
    }

    dest_data.buffer = encoder.into_inner()?;
    let _ = fs::write(dest_image_path, dest_data.buffer.get_ref())?;

    return Ok(());
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = command!()
        .arg(arg!(input_file: <INPUT_FILE> "The path to the input file"))
        .arg(arg!(output_file: <OUTPUT_FILE> "The path to the output file"))
        .arg(arg!(static: --static "Whether the input is static or not"))
        .arg(arg!(loop_count: --loop_count [LOOP_COUNT] "Number of times to loop for a GIF and for a static input, the resulting number of frames").value_parser(clap::value_parser!(u64).range(1..)).default_value("1"))
        .arg(
            arg!(colors: -c --colors [COLORS] "The colors to use in the gradient")
                .value_delimiter(',')
                .default_value("FF0000,00FF00,0000FF")
        )
        .arg(
            arg!(generator: -g --generator [GENERATOR] "The type generator to use")
                .value_parser(value_parser!(color::GradientGeneratorType))
                .default_value("Discrete")
        )
        .arg(
            arg!(color_space: -s --color_space [COLOR_SPACE] "The color space to use")
                .value_parser(value_parser!(color::ColorSpace))
                .default_value("LCH")
        )
        .arg(
            arg!(mixing_mode: -m --mixing_mode [MIXING_MODE] "What kind of mixing to use")
            .value_parser(value_parser!(color::MixingMode))
            .default_value("Custom")
            )
        .get_matches();

    match matches.get_one::<color::ColorSpace>("color_space").unwrap() {
        color::ColorSpace::HSL => main_impl::<
            palette::RgbHue<color::ScalarType>,
            color::ScalarType,
            color::ScalarType,
            color::ScalarType,
            palette::Hsla<palette::encoding::srgb::Srgb, color::ScalarType>,
        >(matches),
        color::ColorSpace::HSV => main_impl::<
            palette::RgbHue<color::ScalarType>,
            color::ScalarType,
            color::ScalarType,
            color::ScalarType,
            palette::Hsva<palette::encoding::srgb::Srgb, color::ScalarType>,
        >(matches),
        color::ColorSpace::LCH => main_impl::<
            palette::LabHue<color::ScalarType>,
            color::ScalarType,
            color::ScalarType,
            color::ScalarType,
            palette::Lcha<palette::white_point::D65, color::ScalarType>,
        >(matches),
    }
}
