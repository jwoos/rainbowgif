use std::fs::File;
use std::vec;

use clap::{arg, command, value_parser, ArgMatches};
use gif;
use gif_dispose;
use palette;
use palette::{FromColor, IntoColor};

mod color;

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
    let src_image = match File::open(src_image_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}: {}", e.to_string(), src_image_path).into()),
    };

    let mut decoder_options = gif::DecodeOptions::new();
    decoder_options.set_color_output(gif::ColorOutput::Indexed);
    let mut decoder = decoder_options.read_info(src_image)?;
    let mut screen = gif_dispose::Screen::new_decoder(&decoder);

    // decode all the frames into a vec
    let mut frames = vec::Vec::new();
    while let Some(frame) = decoder.read_next_frame()? {
        screen.blit_frame(&frame)?;
        frames.push((frame.clone(), screen.pixels.clone()));
    }

    let dest_image_path = matches.get_one::<String>("output_file").unwrap();
    let mut dest_image = File::create(dest_image_path)?;
    let mut encoder = gif::Encoder::new(&mut dest_image, decoder.width(), decoder.height(), &[])?;
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    let gradient_desc = color::GradientDescriptor::new(color_vec);
    let generator_type = matches
        .get_one::<color::GradientGeneratorType>("generator")
        .unwrap()
        .to_owned();
    let colors = gradient_desc.generate(frames.len(), generator_type);

    for (i, (frame, pixels)) in frames.into_iter().enumerate() {
        let new_color = &colors[i];

        let width = pixels.width();
        let height = pixels.height();

        let mut frame_pixels = vec![];
        for pixel in pixels.pixels() {
            // create LCHA pixel
            let original_pixel = Color::from_color(color::ColorType::new(
                (pixel.r as f64) / 255.,
                (pixel.g as f64) / 255.,
                (pixel.b as f64) / 255.,
                (pixel.a as f64) / 255.,
            ));

            // blend
            let blended_pixel = color::blend_colors(&original_pixel, new_color);

            // convert it to rgb
            let rgba_pixel =
                palette::rgb::Rgba::<palette::encoding::Srgb, f64>::from_color(blended_pixel);
            frame_pixels.push((rgba_pixel.color.red * 255.) as u8);
            frame_pixels.push((rgba_pixel.color.green * 255.) as u8);
            frame_pixels.push((rgba_pixel.color.blue * 255.) as u8);
            frame_pixels.push((rgba_pixel.alpha * 255.) as u8);
        }

        let mut new_frame = gif::Frame::from_rgba(
            width.try_into().unwrap(),
            height.try_into().unwrap(),
            &mut frame_pixels,
        );
        new_frame.delay = frame.delay;
        new_frame.dispose = frame.dispose;
        new_frame.interlaced = frame.interlaced;
        new_frame.needs_user_input = frame.needs_user_input;

        encoder.write_frame(&new_frame)?;
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
