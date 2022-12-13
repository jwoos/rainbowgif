use std::error;

use clap::{arg, command};
use palette::{self, FromColor};

fn main() -> Result<(), Box<dyn error::Error>> {
    let matches = command!()
        .arg(arg!(color: <COLOR> "Color to view"))
        .get_matches();

    let color = matches.get_one::<String>("color").unwrap();

    {
        let rgba_color: palette::rgb::Rgba = rainbowgif::color::from_hex(color)?;
        println!("RGBA: {:?}", rgba_color);

        let lcha_color = palette::Lcha::from_color(rgba_color);
        println!("LCHA: {:?}", lcha_color);

        let laba_color = palette::Laba::from_color(rgba_color);
        println!("LABA: {:?}", laba_color);

        let hsla_color = palette::Hsla::from_color(rgba_color);
        println!("HSLA: {:?}", hsla_color);

        let hsva_color = palette::Hsva::from_color(rgba_color);
        println!("HSVA: {:?}", hsva_color);
    }

    println!("--------------");

    {
        let lin_rgba_color: palette::rgb::LinSrgba = rainbowgif::color::from_hex(color)?;
        println!("RGBA: {:?}", lin_rgba_color);

        let lcha_color = palette::Lcha::from_color(lin_rgba_color);
        println!("LCHA: {:?}", lcha_color);

        let laba_color = palette::Laba::from_color(lin_rgba_color);
        println!("LABA: {:?}", laba_color);

        let hsla_color = palette::Hsla::from_color(lin_rgba_color);
        println!("HSLA: {:?}", hsla_color);

        let hsva_color = palette::Hsva::from_color(lin_rgba_color);
        println!("HSVA: {:?}", hsva_color);
    }

    println!("--------------");

    {
        let gamma_rgba_color: palette::rgb::GammaSrgba = rainbowgif::color::from_hex(color)?;
        println!("RGBA: {:?}", gamma_rgba_color);

        let lcha_color = palette::Lcha::from_color(gamma_rgba_color);
        println!("LCHA: {:?}", lcha_color);

        let laba_color = palette::Laba::from_color(gamma_rgba_color);
        println!("LABA: {:?}", laba_color);

        let hsla_color = palette::Hsla::from_color(gamma_rgba_color);
        println!("HSLA: {:?}", hsla_color);

        let hsva_color = palette::Hsva::from_color(gamma_rgba_color);
        println!("HSVA: {:?}", hsva_color);
    }

    return Ok(());
}
