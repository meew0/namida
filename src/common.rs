use std::{
    fs::File,
    io::Read,
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::{Duration, Instant},
};

use anyhow::anyhow;

pub const NAMIDA_VERSION: &str = "devel";
pub const PROTOCOL_REVISION: u32 = 0x20061025;

pub static BINCODE_CONFIG: bincode::config::Configuration<
    bincode::config::BigEndian,
    bincode::config::Fixint,
> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub fn transcript_warn_error(result: anyhow::Result<()>) {
    if let Err(err) = result {
        println!("Unable to perform transcript: {}", err);
    }
}

pub fn get_usec_since(old_time: Instant) -> u64 {
    let now = Instant::now();
    (now - old_time)
        .as_micros()
        .try_into()
        .expect("microseconds 64 bit overflow")
}

pub fn epoch() -> Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
}

pub fn catch_all_host(ipv6: bool) -> IpAddr {
    if ipv6 {
        IpAddr::V6(Ipv6Addr::UNSPECIFIED)
    } else {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    }
}

pub fn make_transcript_filename(mut extension: &str) -> String {
    let seconds = crate::common::epoch().as_secs();
    format!("{}.{}", seconds, extension)
}

pub fn prepare_proof(mut buffer: &mut [u8], mut secret: &[u8]) -> md5::Digest {
    for (offset, fresh0) in buffer.iter_mut().enumerate() {
        *fresh0 ^= secret[offset % secret.len()];
    }
    md5::compute(buffer)
}

pub fn usleep_that_works(usec: u64) {
    std::thread::sleep(Duration::from_micros(usec));
}

pub fn get_udp_in_errors() -> anyhow::Result<u64> {
    let mut snmp_file = File::open("/proc/net/snmp")?;
    let mut snmp_string = String::new();
    snmp_file.read_to_string(&mut snmp_string)?;

    let mut lines = snmp_string.lines().filter(|line| line.starts_with("Udp: "));

    let first_udp_line = lines.next().ok_or(anyhow!("Could not find UDP line"))?;
    let second_udp_line = lines
        .next()
        .ok_or(anyhow!("Could not find second UDP line"))?;

    let in_errors_pos = first_udp_line
        .split(' ')
        .position(|element| element == "InErrors")
        .ok_or(anyhow!("Could not find InErrors in first UDP line"))?;
    let in_errors_value_str = second_udp_line
        .split(' ')
        .nth(in_errors_pos)
        .ok_or(anyhow!("Second UDP line does not have enough values"))?;
    let in_errors_value: u64 = in_errors_value_str.parse()?;

    Ok(in_errors_value)
}
