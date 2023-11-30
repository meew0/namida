use std::{
    io::Write,
    net::{IpAddr, Ipv4Addr, Ipv6Addr, TcpStream},
    time::{Duration, Instant},
};

use anyhow::anyhow;

pub static BINCODE_CONFIG: bincode::config::Configuration<
    bincode::config::BigEndian,
    bincode::config::Fixint,
> = bincode::config::standard()
    .with_big_endian()
    .with_fixed_int_encoding();

pub fn transcript_warn_error(result: anyhow::Result<()>) {
    if let Err(err) = result {
        println!("Unable to perform transcript: {err}");
    }
}

/// Returns the number of microseconds that have passed since the given `Instant`.
///
/// # Panics
/// Panics if the number of microseconds would not fit into a `u64`.
#[must_use]
pub fn get_µs_since(old_time: Instant) -> u64 {
    let now = Instant::now();
    now.duration_since(old_time)
        .as_micros()
        .try_into()
        .expect("microseconds 64 bit overflow")
}

/// Returns the `Duration` since the Unix epoch.
#[must_use]
pub fn epoch() -> Duration {
    std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap_or_default()
}

/// Returns the IPv6 or IPv4 universal bind host (e.g. 0.0.0.0 for IPv4) depending on the given
/// parameter.
#[must_use]
pub fn catch_all_host(ipv6: bool) -> IpAddr {
    if ipv6 {
        IpAddr::V6(Ipv6Addr::UNSPECIFIED)
    } else {
        IpAddr::V4(Ipv4Addr::UNSPECIFIED)
    }
}

#[must_use]
pub fn make_transcript_filename(extension: &str) -> String {
    let seconds = epoch().as_secs();
    format!("{seconds}.{extension}")
}

#[must_use]
pub fn prepare_proof(buffer: &mut [u8], secret: &[u8]) -> md5::Digest {
    for (offset, fresh0) in buffer.iter_mut().enumerate() {
        *fresh0 ^= secret[offset.rem_euclid(secret.len())];
    }
    md5::compute(buffer)
}

/// Sleeps for the given number of microseconds.
pub fn µsleep_that_works(µs: u64) {
    std::thread::sleep(Duration::from_micros(µs));
}

/// Returns the UDP `InErrors` value from `/proc/net/snmp` on Linux, which quantifies the number of
/// UDP packets that were lost at OS level.
///
/// # Errors
/// Returns an error if the value could not be obtained for whatever reason, such as being on an
/// operating system that does not support this method of obtaining the UDP input error count.
pub fn get_udp_in_errors() -> anyhow::Result<u64> {
    let snmp_string = std::fs::read_to_string("/proc/net/snmp")?;

    let mut lines = snmp_string.lines().filter(|line| line.starts_with("Udp: "));

    let first_udp_line = lines
        .next()
        .ok_or_else(|| anyhow!("Could not find UDP line"))?;
    let second_udp_line = lines
        .next()
        .ok_or_else(|| anyhow!("Could not find second UDP line"))?;

    let in_errors_pos = first_udp_line
        .split(' ')
        .position(|element| element == "InErrors")
        .ok_or_else(|| anyhow!("Could not find InErrors in first UDP line"))?;
    let in_errors_value_str = second_udp_line
        .split(' ')
        .nth(in_errors_pos)
        .ok_or_else(|| anyhow!("Second UDP line does not have enough values"))?;
    let in_errors_value: u64 = in_errors_value_str.parse()?;

    Ok(in_errors_value)
}

/// Wraps a `TcpStream` to be able to conveniently read `bincode` de-/encodable objects.
pub struct SocketWrapper {
    pub socket: TcpStream,
}

impl SocketWrapper {
    /// Try to read one instance of the given type from the TCP stream. Blocks until one complete
    /// instance is read.
    ///
    /// # Errors
    /// Returns an error if the reading process terminated prematurely (e.g. due to EOF)
    pub fn read<T: bincode::Decode>(&mut self) -> anyhow::Result<T> {
        Ok(bincode::decode_from_std_read(
            &mut self.socket,
            BINCODE_CONFIG,
        )?)
    }

    /// Write the given object into the TCP stream.
    ///
    /// # Errors
    /// Returns an error if writing the bytes was unsuccessful.
    pub fn write<T: bincode::Encode>(&mut self, value: T) -> anyhow::Result<usize> {
        Ok(bincode::encode_into_std_write(
            value,
            &mut self.socket,
            BINCODE_CONFIG,
        )?)
    }

    /// Flushes the TCP stream.
    ///
    /// # Errors
    /// Returns an error on I/O failure.
    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.socket.flush()?;
        Ok(())
    }
}
