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
            fn fmt(&self, f: &mut ::std::fmt::Formatter<'_>) -> ::std::fmt::Result {
                write!(f, "{}", self.0)
            }
        }
    };
}

#[derive(
    Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Default, bincode::Encode, bincode::Decode,
)]
pub struct BlockIndex(pub u32);

impl std::ops::Add for BlockIndex {
    type Output = BlockIndex;

    fn add(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_add(rhs.0).expect("block index overflow"))
    }
}

impl std::ops::Sub for BlockIndex {
    type Output = BlockIndex;

    fn sub(self, rhs: Self) -> Self::Output {
        Self(self.0.checked_sub(rhs.0).expect("block index overflow"))
    }
}

impl BlockIndex {
    pub fn is_zero(&self) -> bool {
        self.0 == 0
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, bincode::Encode, bincode::Decode)]
pub struct BlockSize(pub u32);
clapify!(BlockSize, u32, BlockSizeValueParser);

#[derive(Debug, Clone, Copy, bincode::Encode, bincode::Decode)]
pub struct TargetRate(pub u64);
clapify!(TargetRate, u64, TargetRateValueParser);

#[derive(Debug, Clone, Copy, bincode::Encode, bincode::Decode)]
pub struct ErrorRate(pub u32);
clapify!(ErrorRate, u32, ErrorRateValueParser);

#[derive(Debug, Clone, Copy, Default, bincode::Encode, bincode::Decode)]
pub struct FileSize(pub u64);

#[derive(Debug, Clone, Copy, Default, bincode::Encode, bincode::Decode)]
pub struct Epoch(pub i64);

#[derive(Debug, Clone, Copy, bincode::Encode, bincode::Decode)]
pub struct Fraction {
    pub numerator: u16,
    pub denominator: u16,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct FileMetadata {
    pub path: PathBuf,
    pub size: FileSize,
}
