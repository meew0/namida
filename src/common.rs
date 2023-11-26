use std::{
    net::{IpAddr, Ipv4Addr, Ipv6Addr},
    time::{Duration, Instant},
};

use crate::extc;
use ::libc;
use anyhow::bail;

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

pub unsafe fn htonll(mut value: u64) -> u64 {
    static mut necessary: libc::c_int = -(1 as libc::c_int);
    if necessary == -(1 as libc::c_int) {
        necessary = (5 as libc::c_int != extc::__bswap_16(5 as libc::c_int as u16) as libc::c_int)
            as libc::c_int;
    }
    if necessary != 0 {
        (extc::__bswap_32(
            (value as libc::c_ulonglong & 0xffffffff as libc::c_longlong as libc::c_ulonglong)
                as u32,
        ) as u64)
            << 32 as libc::c_int
            | extc::__bswap_32((value >> 32 as libc::c_int) as u32) as u64
    } else {
        value
    }
}
pub fn make_transcript_filename(mut extension: &str) -> String {
    let seconds = crate::common::epoch().as_secs();
    format!("{}.{}", seconds, extension)
}

pub unsafe fn ntohll(mut value: u64) -> u64 {
    htonll(value)
}

pub fn prepare_proof(mut buffer: &mut [u8], mut secret: &[u8]) -> md5::Digest {
    for (offset, fresh0) in buffer.iter_mut().enumerate() {
        *fresh0 ^= secret[offset % secret.len()];
    }
    md5::compute(buffer)
}

pub unsafe fn read_line(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut buffer_length: usize,
) -> anyhow::Result<()> {
    let mut buffer_offset: libc::c_int = 0 as libc::c_int;
    loop {
        if extc::read(
            fd,
            buffer.offset(buffer_offset as isize) as *mut libc::c_void,
            1 as libc::c_int as u64,
        ) <= 0 as libc::c_int as i64
        {
            bail!("Could not read complete line of input");
        }
        buffer_offset += 1;
        if !(*buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
            != '\0' as i32
            && *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32
            && (buffer_offset as usize) < buffer_length)
        {
            break;
        }
    }
    *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    Ok(())
}
pub unsafe fn fread_line(
    mut f: *mut extc::FILE,
    mut buffer: *mut libc::c_char,
    mut buffer_length: u64,
) -> anyhow::Result<()> {
    let mut buffer_offset: libc::c_int = 0 as libc::c_int;
    loop {
        if extc::fread(
            buffer.offset(buffer_offset as isize) as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) <= 0 as libc::c_int as libc::c_ulong
        {
            bail!("Could not read complete line of input");
        }
        buffer_offset += 1;
        if !(*buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
            != '\0' as i32
            && *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32
            && (buffer_offset as u64) < buffer_length)
        {
            break;
        }
    }
    *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    Ok(())
}

pub fn usleep_that_works(usec: u64) {
    std::thread::sleep(Duration::from_micros(usec));
}

pub unsafe fn get_udp_in_errors() -> u64 {
    let mut f: *mut extc::FILE = std::ptr::null_mut::<extc::FILE>();
    let mut errs: u64 = 0 as libc::c_int as u64;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut p: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    f = extc::fopen(
        b"/proc/net/snmp\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        return 0 as libc::c_int as u64;
    }
    while extc::feof(f) == 0 {
        if (extc::fgets(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            f,
        ))
        .is_null()
        {
            break;
        }
        if !(!(extc::strstr(
            buf.as_mut_ptr(),
            b"Udp:\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
            && !(extc::strstr(
                buf.as_mut_ptr(),
                b"InErrors\0" as *const u8 as *const libc::c_char,
            ))
            .is_null()
            && extc::feof(f) == 0)
        {
            continue;
        }
        if (extc::fgets(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            f,
        ))
        .is_null()
        {
            break;
        }
        len = extc::strlen(buf.as_mut_ptr()) as libc::c_int;
        p = buf.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int
            && !p.is_null()
            && p < buf
                .as_mut_ptr()
                .offset(len as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            p = extc::strchr(p, ' ' as i32);
            i += 1;
            p = p.offset(1);
        }
        if !p.is_null()
            && p < buf
                .as_mut_ptr()
                .offset(len as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            let fresh1 = p;
            p = p.offset(-1);
            errs = extc::atol(fresh1) as u64;
        } else {
            errs = 0 as libc::c_int as u64;
        }
        break;
    }
    extc::fclose(f);
    errs
}
pub unsafe fn full_write(mut fd: libc::c_int, mut buf: *const libc::c_void, mut count: u64) -> i64 {
    let mut written: i64 = 0 as libc::c_int as i64;
    while (written as u64) < count {
        let mut nwr: i64 = extc::write(
            fd,
            buf.offset(written as isize),
            count.wrapping_sub(written as u64),
        );
        if nwr < 0 as libc::c_int as i64 {
            extc::fprintf(
                extc::stderr,
                b"full_write(): %s\n\0" as *const u8 as *const libc::c_char,
                extc::strerror(*extc::__errno_location()),
            );
            return written;
        }
        written += nwr;
    }
    written
}
pub unsafe fn full_read(mut fd: libc::c_int, mut buf: *mut libc::c_void, mut count: u64) -> i64 {
    let mut nread: i64 = 0 as libc::c_int as i64;
    while (nread as u64) < count {
        let mut nrd: i64 = crate::extc::read(
            fd,
            buf.offset(nread as isize),
            count.wrapping_sub(nread as u64),
        );
        if nrd < 0 as libc::c_int as i64 {
            extc::fprintf(
                extc::stderr,
                b"full_read(): %s\n\0" as *const u8 as *const libc::c_char,
                extc::strerror(*extc::__errno_location()),
            );
            return nread;
        }
        nread += nrd;
    }
    nread
}
