use crate::types::{BlockSize, ErrorRate, Fraction, TargetRate};

pub const DEFAULT_BLOCK_SIZE: BlockSize = BlockSize(1024);
pub const DEFAULT_TABLE_SIZE: libc::c_int = 4096;
pub const DEFAULT_SERVER_NAME: &str = "localhost";
pub const DEFAULT_SERVER_PORT: u16 = 51038;
pub const DEFAULT_CLIENT_PORT: u16 = 51038;
pub const DEFAULT_UDP_BUFFER: u32 = 20_000_000;
pub const DEFAULT_VERBOSE_YN: u8 = 1;
pub const DEFAULT_TRANSCRIPT_YN: u8 = 0;
pub const DEFAULT_IPV6_YN: u8 = 0;
pub const DEFAULT_OUTPUT_MODE: u8 = 1;
pub const DEFAULT_RATE_ADJUST: u8 = 0;
pub const DEFAULT_TARGET_RATE: TargetRate = TargetRate(650_000_000);
pub const DEFAULT_ERROR_RATE: ErrorRate = ErrorRate(7500);
pub const DEFAULT_SLOWER: Fraction = Fraction {
    numerator: 25,
    denominator: 24,
};
pub const DEFAULT_FASTER: Fraction = Fraction {
    numerator: 5,
    denominator: 6,
};
pub const DEFAULT_HISTORY: u16 = 25;
pub const DEFAULT_NO_RETRANSMIT: u8 = 0;
pub const DEFAULT_LOSSLESS: u8 = 1;
pub const DEFAULT_LOSSWINDOW_MS: u32 = 1000;
pub const DEFAULT_BLOCKDUMP: u8 = 0;
pub const MAX_COMMAND_LENGTH: libc::c_int = 1024;
