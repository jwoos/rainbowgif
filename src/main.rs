use clap::{App, Arg};
use std::vec;

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
            Arg::with_name("OUPUT_FILE")
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
                .help("The colors to use in the gradient")
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
            Err(e) => return Err(format!("{}: {}", e.to_string(), color_string))
        }
    }

    return Ok(());
}
