/// Our own protocol revision counter. On every protocol update, this is incremented by 1. There are
/// 11 bits available for this value, so it should not exceed 2048.
pub const NAMIDA_PROTOCOL_REVISION: u16 = 4;

/// The protocol revision as a 32 bit integer, compatible with Tsunami's negotiation mechanism.
/// Tsunami simply used a date interpreted as hexadecimal digits, e.g. 0x20061025.
/// We use a different format that should always be incompatible with hypothetical other versions
/// of Tsunami.
const VERSION_IDENTIFIER_BASE: u32 = 0xf000_0000 | NAMIDA_PROTOCOL_REVISION as u32;

/// This value is bitwise or-ed with `PROTOCOL_REVISION` if the given party desires an encrypted
/// connection.
const ENCRYPTED_PROTOCOL_FLAG: u32 = 0x0000_0800;

/// We also make the block size part of the protocol identifier, to make sure a block size
/// disagreement causes a failure earlier than later.
const BLOCK_SIZE_SHIFT: u32 = 12;

/// The magic value is constructed as follows (numbers refer to bit indices, 31 being most
/// significant):
///
/// ```
/// 31 30 29 28 27 26 25 24 23 22 21 20 19 18 17 16 15 14 13 12 11 10  9  8  7  6  5  4  3  2  1  0
///  1  1  1  1 [                 block size                  ]  E [      protocol revision       ]
/// ```
///
/// where `E` is `1` if an encrypted connection should take place, and `0` otherwise.
#[must_use]
pub const fn magic(encrypted: bool) -> u32 {
    let shifted_block_size = (crate::common::BLOCK_SIZE as u32) << BLOCK_SIZE_SHIFT;
    if encrypted {
        VERSION_IDENTIFIER_BASE | ENCRYPTED_PROTOCOL_FLAG | shifted_block_size
    } else {
        VERSION_IDENTIFIER_BASE | shifted_block_size
    }
}

/// The version as a string. The semver “minor” part should be the same as the protocol revision
/// counter.
pub const NAMIDA_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Git revision at compile time
pub const GIT_HASH: &str = env!("GIT_HASH");

/// Formatted date & time of when the namida executable was compiled.
pub const COMPILE_DATE_TIME: &str = env!("NAMIDA_COMPILE_DT");
