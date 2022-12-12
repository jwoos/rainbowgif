macro_rules! define_error {
    ($x:ident, { $($y:ident : $z:literal),* $(,)? }) => {
        #[derive(Debug)]
        pub enum $x {
            $(
                $y(Option<Box<dyn std::error::Error>>, String),
            )*
        }

        impl std::fmt::Display for $x {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                return match self {
                    $(
                        Self::$y(_, desc) => f.write_str(format!("{}: {desc}", $z).as_str()),
                    )*
                };
            }
        }

        impl std::error::Error for $x {
            fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
                return match self {
                    $(
                        Self::$y(src, _) => src.as_ref().map(|e| e.as_ref()),
                    )*
                };
            }
        }
    };
}

pub(crate) use define_error;
