use crate::types::BlockSize;

pub const DEFAULT_BLOCK_SIZE: BlockSize = BlockSize(1024);
pub const DEFAULT_SECRET: &str = "kitten";
pub const DEFAULT_BIND: &str = "0.0.0.0:51038";
pub const DEFAULT_UDP_BUFFER: u32 = 20000000;
pub const DEFAULT_VERBOSE_YN: u8 = 1;
pub const DEFAULT_TRANSCRIPT_YN: u8 = 0;
pub const DEFAULT_IPV6_YN: u8 = 0;
pub const DEFAULT_HEARTBEAT_TIMEOUT: u16 = 15;
