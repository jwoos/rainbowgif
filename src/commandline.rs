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
    }
);
