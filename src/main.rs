use std::cmp;
use std::error;
use std::fmt;
use std::fs;
use std::vec;

use clap::{arg, command, value_parser, ArgMatches};
use palette;
use palette::WithAlpha;

mod buffer;
mod codec;
mod color;
mod commandline;
mod error_utils;

fn mix_impl<C>(matches: ArgMatches, mix_fn: fn(&C, &C) -> C) -> Result<(), Box<dyn error::Error>>
where
    C: color::Color,
    palette::rgb::Rgb<color::EncodingType, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    let src_image_path = matches.get_one::<String>("input_file").unwrap();
    let src_data = buffer::Data::from_path(src_image_path)?;
    let mut decoder: Box<dyn codec::Decodable<OutputColor = C>> = {
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
    let input_colors = commandline::get_colors::<C>(&matches)?;
    let colors = commandline::get_gradient(&matches, input_colors, frames_len, loop_count);

    for l in 0usize..loop_count {
        for (i, frame) in frames.iter().enumerate() {
            let new_color = &colors[i + (frames_len * l)];

            let frame_pixels = frame
                .pixels
                .iter()
                .map(|pixel| {
                    return mix_fn(pixel, new_color);
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

fn mix_none<C>(matches: ArgMatches) -> Result<(), Box<dyn error::Error>>
where
    C: color::Color,
    palette::rgb::Rgb<color::EncodingType, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    return mix_impl(matches, |a: &C, _: &C| {
        return a.clone();
    });
}

fn mix_custom<H, C>(matches: ArgMatches) -> Result<(), Box<dyn error::Error>>
where
    C: color::Color
        + color::Componentize<H, color::ScalarType, color::ScalarType, color::ScalarType>
        + fmt::Debug,
    palette::rgb::Rgb<color::EncodingType, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    return mix_impl(matches, |a, b| {
        return color::blend_colors::<H, color::ScalarType, color::ScalarType, color::ScalarType, C>(
            a, b, true,
        );
    });
}

fn mix_lab(matches: ArgMatches) -> Result<(), Box<dyn error::Error>> {
    return mix_impl(
        matches,
        |a: &palette::Laba<color::WhitePoint, f64>, b: &palette::Laba<color::WhitePoint, f64>| {
            return palette::Laba::from_components((a.l, b.a, b.b, a.alpha));
        },
    );
}

fn mix_linear<C>(matches: ArgMatches) -> Result<(), Box<dyn error::Error>>
where
    C: color::Color,
    palette::rgb::Rgb<color::EncodingType, color::ScalarType>:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    // this isn't quite right, but again linear mixing might just not be ever
    return mix_impl(matches, |a: &C, b: &C| {
        let (_, a_alpha) = a.clone().split();
        if a_alpha <= 0.5 {
            return a.clone();
        }

        return a.mix(b, 0.2);
    });
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = command!()
        .arg(arg!(input_file: <INPUT_FILE> "The path to the input file"))
        .arg(arg!(output_file: <OUTPUT_FILE> "The path to the output file"))
        .arg(arg!(static: --static "Whether the input is static or not"))
        .arg(
            arg!(loop_count: --loop_count [LOOP_COUNT] "Number of times to loop for a GIF and for a static input, the resulting number of frames")
                .value_parser(clap::value_parser!(u64)
                .range(1..))
                .default_value("1")
        )
        .arg(
            arg!(colors: -c --colors [COLORS] "The colors to use in the gradient")
                .value_delimiter(',')
                .default_value("FF0000,00FF00,0000FF")
        )
        .arg(
            arg!(generator: -g --generator [GENERATOR] "The type generator to use")
                .value_parser(value_parser!(color::GradientGeneratorType))
                .default_value("discrete")
        )
        .arg(
            arg!(color_space: -s --color_space [COLOR_SPACE] "The color space to use")
                .value_parser(value_parser!(color::ColorSpace))
                .default_value("lch")
        )
        .arg(
            arg!(mixing_mode: -m --mixing_mode [MIXING_MODE] "What kind of mixing to use")
            .value_parser(value_parser!(color::MixingMode))
            .default_value("custom")
            )
        .get_matches();

    let color_space = matches.get_one::<color::ColorSpace>("color_space").unwrap();

    match matches.get_one::<color::MixingMode>("mixing_mode").unwrap() {
        color::MixingMode::None => match color_space {
            color::ColorSpace::HSL => {
                mix_none::<palette::Hsla<color::EncodingType, color::ScalarType>>(matches)
            }

            color::ColorSpace::HSV => {
                mix_none::<palette::Hsva<color::EncodingType, color::ScalarType>>(matches)
            }

            color::ColorSpace::LAB => {
                mix_none::<palette::Laba<color::WhitePoint, color::ScalarType>>(matches)
            }

            color::ColorSpace::LCH => {
                mix_none::<palette::Lcha<color::WhitePoint, color::ScalarType>>(matches)
            }

            _ => Err(Box::new(commandline::CommandlineError::IncompatibleValue(
                None,
                "Only HSL, HSV, LAB, and LCH are supported for custom mixing mode".to_owned(),
            ))),
        },

        color::MixingMode::Custom => match color_space {
            color::ColorSpace::HSL => mix_custom::<
                palette::RgbHue<color::ScalarType>,
                palette::Hsla<color::EncodingType, color::ScalarType>,
            >(matches),
            color::ColorSpace::HSV => mix_custom::<
                palette::RgbHue<color::ScalarType>,
                palette::Hsva<color::EncodingType, color::ScalarType>,
            >(matches),
            color::ColorSpace::LCH => mix_custom::<
                palette::LabHue<color::ScalarType>,
                palette::Lcha<color::WhitePoint, color::ScalarType>,
            >(matches),

            _ => Err(Box::new(commandline::CommandlineError::IncompatibleValue(
                None,
                "Only HSL, HSV, and LCH are supported for custom mixing mode".to_owned(),
            ))),
        },

        color::MixingMode::Lab => match color_space {
            color::ColorSpace::LAB => mix_lab(matches),

            _ => Err(Box::new(commandline::CommandlineError::IncompatibleValue(
                None,
                "Only LAB is supported for lab mixing mode".to_owned(),
            ))),
        },

        color::MixingMode::Linear => match color_space {
            color::ColorSpace::HSL => {
                mix_linear::<palette::Hsla<color::EncodingType, color::ScalarType>>(matches)
            }

            color::ColorSpace::HSV => {
                mix_linear::<palette::Hsva<color::EncodingType, color::ScalarType>>(matches)
            }

            color::ColorSpace::LAB => {
                mix_linear::<palette::Laba<color::WhitePoint, color::ScalarType>>(matches)
            }

            color::ColorSpace::LCH => {
                mix_linear::<palette::Lcha<color::WhitePoint, color::ScalarType>>(matches)
            }

            _ => Err(Box::new(commandline::CommandlineError::IncompatibleValue(
                None,
                "Only HSL, HSV, LAB, and LCH are supported for custom mixing mode".to_owned(),
            ))),
        },

        color::MixingMode::BlendOverlay => {
            Err(Box::new(commandline::CommandlineError::NotImplemented(
                None,
                "Blend overlay mixing is not implemented".to_owned(),
            )))
        }
    }
}
