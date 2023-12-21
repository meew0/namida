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

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct FileChecksums {
    pub chunk_blocks: u64,
    pub last_chunk_blocks: u64,
    pub checksums: Vec<u64>,
}

impl FileChecksums {
    /// Find the chunks that match between the current object and the other given `FileChecksums`.
    /// Returns a bit vector of which chunks match, and the number of matching chunks.
    ///
    /// # Panics
    /// Panics if the count overflows.
    #[must_use]
    pub fn compare(&self, other: &FileChecksums) -> SkipChunks {
        let mut res = vec![false; self.checksums.len()];
        let mut count = 0_u64;

        // It makes no sense to compare two `FileChecksums` with different chunk sizes, so in that
        // case, we just assume that all chunks are different
        if self.chunk_blocks != other.chunk_blocks {
            // Return the all-`false` `Vec` we created before
            return SkipChunks {
                matches: res,
                chunk_blocks: self.chunk_blocks,
                last_chunk_blocks: self.last_chunk_blocks,
            };
        }

        // Iterate and compare all checksums
        for (index, checksum) in self.checksums.iter().enumerate() {
            // Ignore chunks that are not present at all in the other object
            if let Some(other_checksum) = other.checksums.get(index) {
                count = count.checked_add(1).expect("count overflow");
                res[index] = checksum == other_checksum;
            }
        }

        SkipChunks {
            matches: res,
            chunk_blocks: self.chunk_blocks,
            last_chunk_blocks: self.last_chunk_blocks,
        }
    }
}

#[derive(Debug, Clone, bincode::Encode, bincode::Decode)]
pub struct SkipChunks {
    pub chunk_blocks: u64,
    pub last_chunk_blocks: u64,
    pub matches: Vec<bool>,
}

impl SkipChunks {
    /// Counts the number of matching blocks.
    ///
    /// # Panics
    /// Panics on arithmetic overflow.
    #[must_use]
    pub fn count_blocks(&self) -> u64 {
        let Some(full_block_count) = self.matches.len().checked_sub(1) else {
            // `matches` has 0 length, so the return value will always be 0
            return 0;
        };

        let full_chunk_count = self
            .matches
            .iter()
            .take(full_block_count)
            .copied()
            .filter(|present| *present)
            .count() as u64;
        let last_chunk_blocks = if self.matches.last().is_some_and(|present| *present) {
            self.last_chunk_blocks
        } else {
            0
        };
        full_chunk_count
            .checked_mul(self.chunk_blocks)
            .and_then(|full_chunk_blocks| full_chunk_blocks.checked_add(last_chunk_blocks))
            .expect("count_blocks overflow")
    }

    /// Checks if the given block can be skipped.
    ///
    /// # Panics
    /// Panics on arithmetic overflow.
    #[must_use]
    pub fn has_block(&self, block_index: BlockIndex) -> bool {
        let chunk_index: usize = u64::from(block_index.0)
            .checked_div(self.chunk_blocks)
            .expect("chunk_blocks is 0")
            .try_into()
            .expect("chunk_index overflow");
        let maybe_chunk_present: Option<&bool> = self.matches.get(chunk_index);
        maybe_chunk_present.is_some_and(|present| *present)
    }
}

/// Compact bitset to store which blocks we already received
#[derive(Default)]
pub struct ReceivedMap {
    pub inner: Vec<u8>,
}

impl ReceivedMap {
    /// Create a new `ReceivedMap` able to store the given number of blocks.
    ///
    /// # Panics
    /// Panics on arithmetic overflow.
    #[must_use]
    pub fn new(block_count: BlockIndex) -> Self {
        Self {
            inner: vec![
                0;
                (block_count.0 / 8)
                    .checked_add(2)
                    .expect("`received` bitfield size overflow") as usize
            ],
        }
    }

    #[must_use]
    pub fn got_block(&self, blocknr: BlockIndex) -> bool {
        self.inner[(blocknr.0 / 8) as usize] & (1 << (blocknr.0 % 8)) != 0
    }

    pub fn set(&mut self, this_block: BlockIndex) {
        let fresh1 = &mut self.inner[(this_block.0 / 8) as usize];
        *fresh1 |= 1 << (this_block.0 % 8);
    }
}
