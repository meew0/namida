/// Our own protocol revision counter. On every protocol update, this is incremented by 1.
pub const NAMIDA_PROTOCOL_REVISION: u16 = 0;

/// The protocol revision as a 32 bit integer, compatible with Tsunami's negotiation mechanism.
/// Tsunami simply used a date interpreted as hexadecimal digits, e.g. 0x20061025.
/// We use a different format that should always be incompatible with hypothetical other versions
/// of Tsunami.
pub const PROTOCOL_REVISION: u32 = 0xff23_0000 | NAMIDA_PROTOCOL_REVISION as u32;

/// The version as a string. The semver “minor” part should be the same as the protocol revision
/// counter.
pub const NAMIDA_VERSION: &str = env!("CARGO_PKG_VERSION");

/// Git revision at compile time
pub const GIT_HASH: &str = env!("GIT_HASH");

/// Formatted date & time of when the namida executable was compiled.
pub const COMPILE_DATE_TIME: &str = env!("NAMIDA_COMPILE_DT");
