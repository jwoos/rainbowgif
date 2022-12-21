use std::error;
use std::vec;

use crate::color;

macro_rules! define_cli_enum {
    ($enum_name:ident, { $($enum_val:ident : ($enum_val_name:literal, $enum_help:literal)),* $(,)? }) => {
        #[derive(Clone, Copy)]
        pub enum $enum_name {
            $(
                $enum_val,
            )*
        }

        impl ValueEnum for $enum_name {
            fn value_variants<'a>() -> &'a [Self] {
                return &[
                    $(
                        Self::$enum_val,
                    )*
                ];
            }

            fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
                return Some(match self {
                    $(
                        Self::$enum_val => PossibleValue::new($enum_val_name).help($enum_help),
                    )*
                });
            }
        }

        impl std::fmt::Display for $enum_name {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return self.to_possible_value()
                    .expect("no values are skipped")
                    .get_name()
                    .fmt(f);
            }
        }

        impl std::str::FromStr for $enum_name {
            type Err = String;

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                for variant in Self::value_variants() {
                    if variant.to_possible_value().unwrap().matches(s, false) {
                        return Ok(*variant);
                    }
                }

                return Err(format!("Invalid variant: {}", s));
            }
        }
    };
}

pub(crate) use define_cli_enum;

crate::error_utils::define_error!(
    CommandlineError, {
        IncompatibleValue: "The given value is incompatible with the configurations",
        NotImplemented: "The given option is not implemented",
        InvalidValue: "The given value is invalid",
    }
);

pub fn get_colors<C>(matches: &clap::ArgMatches) -> Result<vec::Vec<C>, Box<dyn error::Error>>
where
    C: color::Color,
{
    let color_strings = matches.get_many::<String>("colors").unwrap();
    let mut color_vec: vec::Vec<C> = vec::Vec::new();
    for color_string in color_strings {
        if color_string.len() != 6 {
            return Err(Box::new(CommandlineError::InvalidValue(
                None,
                format!("Invalid color format {}", &color_string),
            )));
        }

        match color::from_hex::<C>(&color_string) {
            Ok(c) => color_vec.push(c),
            Err(e) => {
                return Err(Box::new(CommandlineError::InvalidValue(
                    Some(Box::new(e)),
                    format!("Could not parse {} as color", color_string).into(),
                )))
            }
        }
    }

    return Ok(color_vec);
}

pub fn get_gradient<C>(
    matches: &clap::ArgMatches,
    colors: vec::Vec<C>,
    frames_len: usize,
    loop_count: usize,
) -> vec::Vec<C>
where
    C: color::Color + palette::Mix<Scalar = color::ScalarType> + Clone,
    palette::rgb::Rgb:
        palette::convert::FromColorUnclamped<<C as palette::WithAlpha<color::ScalarType>>::Color>,
{
    let gradient_desc = color::gradient::GradientDescriptor::new(colors);
    let generator_type = matches
        .get_one::<color::gradient::GradientGeneratorType>("generator")
        .unwrap()
        .to_owned();
    return gradient_desc.generate(frames_len * loop_count, generator_type);
}

pub fn get_gradient_2<C>(
    matches: &clap::ArgMatches,
    colors: vec::Vec<C>,
    frames_len: usize,
    loop_count: usize,
) -> vec::Vec<C>
where
    C: color::Color + palette::Mix<Scalar = color::ScalarType> + Clone,
{
    println!("HELLO");
    // hard coded gradient from the go version for fidget spinner
    return vec![
        C::from_color(color::ColorType::new(
            0.999999918662032,
            1.703028093156283e-06,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.9922556125011065,
            0.2849531521554511,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.9627980942031773,
            0.42782030813630506,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.9125083887754324,
            0.5468183165433307,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.841537229795374,
            0.6529456392653107,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.7488661564705226,
            0.749785992742367,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.6289970827379517,
            0.8391317023531399,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.46107468248846606,
            0.9222048643508922,
            0.,
            1.,
        )),
        C::from_color(color::ColorType::new(5.226930676345863e-07, 1., 0., 1.)),
        C::from_color(color::ColorType::new(
            0.,
            0.950319894289878,
            0.4129452019482706,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.,
            0.884209508605179,
            0.6780196090054769,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.,
            0.8095729216795732,
            0.9308077481287376,
            1.,
        )),
        C::from_color(color::ColorType::new(0., 0.7297272229698234, 1., 1.)),
        C::from_color(color::ColorType::new(0., 0.6407381119030577, 1., 1.)),
        C::from_color(color::ColorType::new(0., 0.5304552900441599, 1., 1.)),
        C::from_color(color::ColorType::new(0., 0.3754240144898441, 1., 1.)),
        C::from_color(color::ColorType::new(6.469852725499018e-07, 0., 1., 1.)),
        C::from_color(color::ColorType::new(
            0.5712265300199242,
            0.,
            0.8888992006071924,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.7730113881799562,
            0.,
            0.7635102796640795,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.9014103746953062,
            0.,
            0.632485491998479,
            1.,
        )),
        C::from_color(color::ColorType::new(
            0.9822822089152776,
            0.,
            0.5034234435986551,
            1.,
        )),
        C::from_color(color::ColorType::new(1., 0., 0.3818031186680415, 1.)),
        C::from_color(color::ColorType::new(1., 0., 0.26939468411722584, 1.)),
        C::from_color(color::ColorType::new(1., 0., 0.16015913371298365, 1.)),
    ];
}
