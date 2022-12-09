macro_rules! define_cli_enum {
    ($x:ident, { $($y:ident : $z:literal),* $(,)? }) => {
        #[derive(Clone, Copy)]
        pub enum $x {
            $(
                $y,
            )*
        }

        impl ValueEnum for $x {
            fn value_variants<'a>() -> &'a [Self] {
                return &[
                    $(
                        Self::$y,
                    )*
                ];
            }

            fn to_possible_value<'a>(&self) -> Option<PossibleValue> {
                return Some(match self {
                    $(
                        Self::$y => PossibleValue::new(stringify!($y)).help($z),
                    )*
                });
            }
        }

        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return self.to_possible_value()
                    .expect("no values are skipped")
                    .get_name()
                    .fmt(f);
            }
        }

        impl std::str::FromStr for $x {
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

    }
}

pub(crate) use define_cli_enum;
