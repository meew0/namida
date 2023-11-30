use std::{fmt::Display, path::PathBuf};

// Clap value parser and display implementations
macro_rules! clapify {
    ($new_type:ident, $old_type:ty, $parser_name:ident) => {
        impl ::clap::builder::ValueParserFactory for $new_type {
            type Parser = $parser_name;
            fn value_parser() -> Self::Parser {
                $parser_name
            }
        }

        #[derive(Clone, Debug)]
        pub struct $parser_name;
        impl ::clap::builder::TypedValueParser for $parser_name {
            type Value = $new_type;

            fn parse_ref(
                &self,
                cmd: &::clap::Command,
                arg: Option<&::clap::Arg>,
                value: &::std::ffi::OsStr,
            ) -> Result<Self::Value, clap::Error> {
                let inner = clap::value_parser!($old_type);
                let val = inner.parse_ref(cmd, arg, value)?;
                Ok($new_type(val))
            }
        }

        impl ::std::fmt::Display for $new_type {
            fn fmt(&self, formatter: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(formatter, "{}", self.0)
            }
        }
    };
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, bincode::Encode, bincode::Decode,
)]
pub struct BlockIndex(pub u32);

impl BlockIndex {
    #[must_use]
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }

    /// Adds another `BlockIndex` to this one.
    ///
    /// # Panics
    /// Panics on overflow.
    #[must_use]
    pub fn safe_add(self, rhs: Self) -> Self {
        Self(self.0.checked_add(rhs.0).expect("block index overflow"))
    }

    /// Subtracts another `BlockIndex` from this one.
    ///
    /// # Panics
    /// Panics on underflow.
    #[must_use]
    pub fn safe_sub(self, rhs: Self) -> Self {
        Self(self.0.checked_sub(rhs.0).expect("block index underflow"))
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct BlockSize(pub u32);
clapify!(BlockSize, u32, BlockSizeValueParser);

#[derive(Debug, Clone, Copy, bincode::Encode, bincode::Decode)]
pub struct TargetRate(pub u64);
clapify!(TargetRate, u64, TargetRateValueParser);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, bincode::Encode, bincode::Decode)]
pub struct ErrorRate(pub u32);
clapify!(ErrorRate, u32, ErrorRateValueParser);

#[derive(Debug, Clone, Copy, Default, bincode::Encode, bincode::Decode)]
pub struct FileSize(pub u64);

#[derive(Debug, Clone, Copy, bincode::Encode, bincode::Decode)]
pub struct Fraction {
    pub numerator: u16,
    pub denominator: u16,
}

impl Display for Fraction {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(formatter, "{}/{}", self.numerator, self.denominator)
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: FileSize,
}

#[derive(Debug, Clone)]
pub enum UdpErrors {
    Available { initial: u64, current: u64 },
    Unavailable,
}

impl Display for UdpErrors {
    fn fmt(&self, formatter: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Available { initial, current } => {
                write!(
                    formatter,
                    "{}",
                    current
                        .checked_sub(*initial)
                        .expect("UDP error count should not decrease over time")
                )
            }
            Self::Unavailable => write!(formatter, "N/A"),
        }
    }
}

impl Default for UdpErrors {
    fn default() -> Self {
        Self::Unavailable
    }
}

impl UdpErrors {
    #[must_use]
    pub fn new() -> Self {
        match crate::common::get_udp_in_errors() {
            Ok(value) => Self::Available {
                initial: value,
                current: value,
            },
            Err(err) => {
                println!("Note: OS-level UDP error count is unavailable for reason: {err}");
                Self::Unavailable
            }
        }
    }

    pub fn update(&mut self) {
        let Self::Available { current, .. } = self else {
            return;
        };

        match crate::common::get_udp_in_errors() {
            Ok(value) => *current = value,
            Err(err) => {
                println!("WARNING: OS-level UDP error count was previously available, but is now unavailable for reason: {err}");
                *self = Self::Unavailable;
            }
        }
    }
}
