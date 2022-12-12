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

pub fn get_colors<Color>(
    matches: &clap::ArgMatches,
) -> Result<vec::Vec<Color>, Box<dyn error::Error>>
where
    Color: color::Color,
{
    let color_strings = matches.get_many::<String>("colors").unwrap();
    let mut color_vec: vec::Vec<Color> = vec::Vec::new();
    for color_string in color_strings {
        if color_string.len() != 6 {
            return Err(Box::new(CommandlineError::InvalidValue(
                None,
                format!("Invalid color format {}", &color_string),
            )));
        }

        match color::from_hex(&color_string) {
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

pub fn get_gradient<Color>(
    matches: &clap::ArgMatches,
    colors: vec::Vec<Color>,
    frames_len: usize,
    loop_count: usize,
) -> vec::Vec<Color>
where
    Color: color::Color + palette::Mix<Scalar = color::ScalarType> + Clone,
{
    let gradient_desc = color::GradientDescriptor::new(colors);
    let generator_type = matches
        .get_one::<color::GradientGeneratorType>("generator")
        .unwrap()
        .to_owned();
    return gradient_desc.generate(frames_len * loop_count, generator_type);
}
