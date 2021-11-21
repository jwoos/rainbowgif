use std::fs::File;
use std::vec;

use clap::{App, Arg};
use gif;

mod color;

fn main() -> Result<(), String> {
    let matches = App::new("rainbowgif")
        .version("0.1.0")
        .about("Rainbow GIF")
        .arg(
            Arg::with_name("INPUT_FILE")
                .required(true)
                .help("The path to the input file"),
        )
        .arg(
            Arg::with_name("OUTPUT_FILE")
                .required(true)
                .help("The path to the output file"),
        )
        .arg(
            Arg::with_name("colors")
                .short("c")
                .long("colors")
                .value_name("COLOR")
                .takes_value(true)
                .use_delimiter(true)
                .default_value("FF0000,007F00,FFFF00,00FF00,0000FF,8B00FF")
                .help("The colors to use in the gradient"),
        )
        .get_matches();

    let color_strings = matches.values_of("colors").unwrap();
    let mut color_vec = vec::Vec::new();
    for color_string in color_strings {
        if color_string.len() != 6 {
            return Err(format!("Invalid color format {}", &color_string));
        }

        match color::hex_to_color(&color_string) {
            Ok(c) => color_vec.push(c),
            Err(e) => return Err(format!("{}: {}", e.to_string(), color_string)),
        }
    }

    let src_image_path = matches.value_of("INPUT_FILE").unwrap();
    let src_image = match File::open(src_image_path) {
        Ok(f) => f,
        Err(e) => return Err(format!("{}: {}", e.to_string(), src_image_path)),
    };

    let mut decoder_options = gif::DecodeOptions::new();
    decoder_options.set_color_output(gif::ColorOutput::RGBA);
    let mut decoder = decoder_options.read_info(src_image).map_err(
        |e| format!("Unable to open decoder: {}", e.to_string())
        )?;

    let dest_image_path = matches.value_of("OUTPUT_FILE").unwrap();
    let mut dest_image = File::create(dest_image_path).map_err(|e| e.to_string())?;
    let mut encoder = gif::Encoder::new(
        &mut dest_image,
        decoder.width(),
        decoder.height(),
        &decoder.global_palette().unwrap_or(&[])
    ).map_err(|e| format!("Unable to open encoder: {}", e.to_string()))?;
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    while let Some(frame) = decoder.read_next_frame().map_err(|e| format!("Unable to decode: {}", e.to_string()))? {
        encoder.write_frame(&frame).map_err(|e| format!("Unable to encode: {}", e.to_string()))?;
    }

    return Ok(());
}
