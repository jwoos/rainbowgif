use clap::{App, Arg};

mod color;

fn main() {
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
        .get_matches();
}
