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
{
    let gradient_desc = color::GradientDescriptor::new(colors);
    let generator_type = matches
        .get_one::<color::GradientGeneratorType>("generator")
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
    // hard coded gradient from the go version for fidget spinner
    return vec![
        C::from_color(color::ColorType::new(
            0.999999918662032,
            1.703028093156283e-06,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.9914169995249467,
            0.29205712531435135,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.9592459324271599,
            0.43885289190869065,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.9044291173911987,
            0.5612814779045389,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.8270405808052534,
            0.6703931033776005,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.7254743050328099,
            0.7697987501213789,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.5914385047550472,
            0.8613648090339407,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.3898006526650924,
            0.9463982300208429,
            0.,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.,
            0.9851655134211822,
            0.2141394257294868,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.,
            0.9259073776005535,
            0.5182056423223595,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.,
            0.8524698406884128,
            0.7902052069268879,
            255.,
        )),
        C::from_color(color::ColorType::new(0., 0.7720877689601598, 1., 255.)),
        C::from_color(color::ColorType::new(0., 0.6850147194000623, 1., 255.)),
        C::from_color(color::ColorType::new(0., 0.5821499112361646, 1., 255.)),
        C::from_color(color::ColorType::new(0., 0.44391355573146596, 1., 255.)),
        C::from_color(color::ColorType::new(0., 0.2147536235971932, 1., 255.)),
        C::from_color(color::ColorType::new(
            0.4831433551802441,
            0.,
            0.924635012741116,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.7293094160250635,
            0.,
            0.7971033150961222,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.8780512060016573,
            0.,
            0.6610399714577333,
            255.,
        )),
        C::from_color(color::ColorType::new(
            0.9710907845696191,
            0.,
            0.5254520894378957,
            255.,
        )),
        C::from_color(color::ColorType::new(1., 0., 0.3971317542547891, 255.)),
        C::from_color(color::ColorType::new(1., 0., 0.2788680698144303, 255.)),
        C::from_color(color::ColorType::new(1., 0., 0.16511199952017694, 255.)),
        C::from_color(color::ColorType::new(1., 0., 0., 255.)),
    ];
}
