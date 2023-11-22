use crate::extc;
use ::libc;
use anyhow::bail;

pub const PROTOCOL_REVISION: u32 = 0x20061025 as libc::c_int as u32;
pub const REQUEST_RETRANSMIT: u16 = 0 as libc::c_int as u16;
pub const REQUEST_RESTART: u16 = 1 as libc::c_int as u16;
pub const REQUEST_STOP: u16 = 2 as libc::c_int as u16;
pub const REQUEST_ERROR_RATE: u16 = 3 as libc::c_int as u16;

pub unsafe fn get_usec_since(mut old_time: *mut extc::timeval) -> u64 {
    let mut now: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut result: u64 = 0 as libc::c_int as u64;
    extc::gettimeofday(&mut now, std::ptr::null_mut::<libc::c_void>());
    while now.tv_sec > (*old_time).tv_sec {
        result = result.wrapping_add(1000000 as libc::c_int as u64);
        now.tv_sec -= 1;
    }
    result.wrapping_add((now.tv_usec - (*old_time).tv_usec) as u64)
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
pub unsafe fn make_transcript_filename(
    mut buffer: *mut libc::c_char,
    mut epoch: extc::time_t,
    mut extension: *const libc::c_char,
) -> *mut libc::c_char {
    let mut gmt: extc::tm = extc::tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: std::ptr::null::<libc::c_char>(),
    };
    extc::gmtime_r(&epoch, &mut gmt);
    extc::sprintf(
        buffer,
        b"%04d-%02d-%02d-%02d-%02d-%02d.%s\0" as *const u8 as *const libc::c_char,
        gmt.tm_year + 1900 as libc::c_int,
        gmt.tm_mon + 1 as libc::c_int,
        gmt.tm_mday,
        gmt.tm_hour,
        gmt.tm_min,
        gmt.tm_sec,
        extension,
    );
    buffer
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
pub unsafe fn usleep_that_works(mut usec: u64) {
    let mut sleep_time: u64 = usec / 10000 as libc::c_int as u64 * 10000 as libc::c_int as u64;
    let mut delay: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut now: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    extc::gettimeofday(&mut now, std::ptr::null_mut::<libc::c_void>());
    if sleep_time >= 10000 as libc::c_int as u64 {
        delay.tv_sec = (sleep_time / 1000000 as libc::c_int as u64) as extc::__time_t;
        delay.tv_usec = (sleep_time % 1000000 as libc::c_int as u64) as extc::__suseconds_t;
        extc::select(
            0 as libc::c_int,
            std::ptr::null_mut::<extc::fd_set>(),
            std::ptr::null_mut::<extc::fd_set>(),
            std::ptr::null_mut::<extc::fd_set>(),
            &mut delay,
        );
    }
    while get_usec_since(&mut now) < usec {}
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
