use std::error;

use clap::{arg, command};
use image::{self, GenericImage, GenericImageView};
use palette::{self, FromColor};

fn main_impl<C>(matches: clap::ArgMatches) -> Result<(), Box<dyn error::Error>>
where
    C: rainbowgif::color::Color,
{
    let input_colors = rainbowgif::commandline::get_colors::<palette::Lcha>(&matches)?;

    let steps = matches.get_one::<u32>("count").unwrap();
    let colors = rainbowgif::commandline::get_gradient(&matches, input_colors, *steps as usize, 1);

    let original_width = matches.get_one::<u32>("width").unwrap().to_owned();
    let increment = original_width / steps;
    let width = increment * steps;

    let mut image =
        image::ImageBuffer::new(width, matches.get_one::<u32>("height").unwrap().to_owned());

    for (i, color) in colors.into_iter().enumerate() {
        let srgb_color = rainbowgif::color::ColorType::from_color(color);
        println!(
            "({}, {}), ({}, {}) - {:?}",
            (i as u32) * increment,
            0,
            ((i as u32) + 1) * increment,
            image.height(),
            srgb_color,
        );
        let mut sub_image = image.sub_image((i as u32) * increment, 0, increment, image.height());
        let (width, height) = sub_image.dimensions();
        for x in 0..width {
            for y in 0..height {
                sub_image.put_pixel(
                    x,
                    y,
                    image::Rgba::from([
                        (srgb_color.red * 255.) as u8,
                        (srgb_color.green * 255.) as u8,
                        (srgb_color.blue * 255.) as u8,
                        255,
                    ]),
                );
            }
        }
    }

    match image.save(matches.get_one::<String>("output_file").unwrap()) {
        Ok(_) => Ok(()),
        Err(e) => {
            println!("Error encoding image: {}", e);
            Err(Box::new(e))
        }
    }
}

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = command!()
        .arg(arg!(colors: <COLORS> "Colors to generate gradient for").value_delimiter(','))
        .arg(arg!(output_file: <OUTPUT_FILE> "The path to the output file"))
        .arg(
            arg!(count: <COUNT> "Number of colors to display")
                .value_parser(clap::value_parser!(u32).range(1..)),
        )
        .arg(
            arg!(width: --width [WIDTH] "Width of the image")
                .value_parser(clap::value_parser!(u32).range(1..))
                .default_value("512"),
        )
        .arg(
            arg!(
            height: --height [HEIGHT] "Height of the image"
            )
            .value_parser(clap::value_parser!(u32).range(1..))
            .default_value("512"),
        )
        .arg(
            arg!(color_space: -s --color_space [COLOR_SPACE] "The color space to use")
                .value_parser(clap::value_parser!(rainbowgif::color::ColorSpace))
                .default_value("lch"),
        )
        .arg(
            arg!(generator: -g --generator [GENERATOR] "The type generator to use")
                .value_parser(clap::value_parser!(
                    rainbowgif::color::GradientGeneratorType
                ))
                .default_value("discrete"),
        )
        .get_matches();

    let color_space = matches
        .get_one::<rainbowgif::color::ColorSpace>("color_space")
        .unwrap();

    match color_space {
        rainbowgif::color::ColorSpace::HSL => main_impl::<
            palette::Hsla<rainbowgif::color::EncodingType, rainbowgif::color::ScalarType>,
        >(matches),

        rainbowgif::color::ColorSpace::HSV => main_impl::<
            palette::Hsva<rainbowgif::color::EncodingType, rainbowgif::color::ScalarType>,
        >(matches),

        rainbowgif::color::ColorSpace::LAB => main_impl::<
            palette::Laba<rainbowgif::color::WhitePoint, rainbowgif::color::ScalarType>,
        >(matches),

        rainbowgif::color::ColorSpace::LCH => main_impl::<
            palette::Lcha<rainbowgif::color::WhitePoint, rainbowgif::color::ScalarType>,
        >(matches),

        _ => Err(Box::new(
            rainbowgif::commandline::CommandlineError::IncompatibleValue(
                None,
                "Only HSL, HSV, LAB, and LCH are supported".to_owned(),
            ),
        )),
    }
}
