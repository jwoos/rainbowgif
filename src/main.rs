use std::fs::File;
use std::vec;

use clap::{arg, command};
use gif;
use gif_dispose;
use palette;
use palette::FromColor;

mod color;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let matches = command!()
        .arg(arg!(<INPUT_FILE> "The path to the input file"))
        .arg(arg!(<OUTPUT_FILE> "The path to the output file"))
        .arg(
            arg!(-c --colors <COLORS> "The colors to use in the gradient")
                .value_delimiter(',')
                .default_value("FF0000,00FF00,0000FF")
                .help("The colors to use in the gradient"),
        )
        .get_matches();

    let color_strings = matches.get_many::<String>("colors").unwrap();
    let mut color_vec = vec::Vec::new();
    for color_string in color_strings {
        if color_string.len() != 6 {
            return Err(format!("Invalid color format {}", &color_string).into());
        }

        match color::hex_to_color(&color_string) {
            Ok(c) => color_vec.push(c),
            Err(e) => return Err(format!("{}: {}", e.to_string(), color_string).into()),
        }
    }

    let src_image_path = matches.get_one::<String>("INPUT_FILE").unwrap();
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

    let dest_image_path = matches.get_one::<String>("OUTPUT_FILE").unwrap();
    let mut dest_image = File::create(dest_image_path)?;
    let mut encoder = gif::Encoder::new(&mut dest_image, decoder.width(), decoder.height(), &[])?;
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    let gradient_desc = color::GradientDescriptor::new(color_vec);
    let colors = gradient_desc.generate(frames.len(), color::GradientGeneratorType::Continuous);

    for (i, (frame, pixels)) in frames.into_iter().enumerate() {
        let new_color = colors[i];

        let width = pixels.width();
        let height = pixels.height();

        let mut frame_pixels = vec![];
        for pixel in pixels.pixels() {
            let original_alpha = pixel.a;

            // create LCH pixel
            let lch_pixel =
                palette::Lch::<palette::white_point::D65, f64>::from_color(color::ColorType::new(
                    (pixel.r as f64) / 255.,
                    (pixel.g as f64) / 255.,
                    (pixel.b as f64) / 255.,
                    (pixel.a as f64) / 255.,
                ));

            // blend
            let new_lch_pixel = color::blend_color(lch_pixel, new_color);

            // convert it to rgb
            let rgba_pixel = color::ColorType::from_color(new_lch_pixel);
            frame_pixels.push((rgba_pixel.color.red * 255.) as u8);
            frame_pixels.push((rgba_pixel.color.green * 255.) as u8);
            frame_pixels.push((rgba_pixel.color.blue * 255.) as u8);
            frame_pixels.push(original_alpha);
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
