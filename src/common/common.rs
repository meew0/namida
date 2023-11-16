use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn open(__file: *const libc::c_char, __oflag: libc::c_int, _: ...) -> libc::c_int;
    fn select(
        __nfds: libc::c_int,
        __readfds: *mut fd_set,
        __writefds: *mut fd_set,
        __exceptfds: *mut fd_set,
        __timeout: *mut timeval,
    ) -> libc::c_int;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strstr(_: *const libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strerror(_: libc::c_int) -> *mut libc::c_char;
    fn gmtime_r(__timer: *const time_t, __tp: *mut tm) -> *mut tm;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn write(__fd: libc::c_int, __buf: *const libc::c_void, __n: size_t) -> ssize_t;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn __errno_location() -> *mut libc::c_int;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn md5_digest(buffer: *mut u_char, size: size_t, digest: *mut u_char);
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
pub type u_char = __u_char;
pub type ssize_t = __ssize_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
pub type __fd_mask = libc::c_long;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct fd_set {
    pub fds_bits: [__fd_mask; 16],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
}
#[no_mangle]
pub static mut PROTOCOL_REVISION: u_int32_t = 0x20061025 as libc::c_int as u_int32_t;
#[no_mangle]
pub static mut REQUEST_RETRANSMIT: u_int16_t = 0 as libc::c_int as u_int16_t;
#[no_mangle]
pub static mut REQUEST_RESTART: u_int16_t = 1 as libc::c_int as u_int16_t;
#[no_mangle]
pub static mut REQUEST_STOP: u_int16_t = 2 as libc::c_int as u_int16_t;
#[no_mangle]
pub static mut REQUEST_ERROR_RATE: u_int16_t = 3 as libc::c_int as u_int16_t;
#[no_mangle]
pub unsafe extern "C" fn get_random_data(
    mut buffer: *mut u_char,
    mut bytes: size_t,
) -> libc::c_int {
    let mut random_fd: libc::c_int = 0;
    random_fd = open(
        b"/dev/urandom\0" as *const u8 as *const libc::c_char,
        0 as libc::c_int,
    );
    if random_fd < 0 as libc::c_int {
        return -(1 as libc::c_int);
    }
    if read(random_fd, buffer as *mut libc::c_void, bytes) < 0 as libc::c_int as ssize_t {
        return -(1 as libc::c_int);
    }
    return if close(random_fd) < 0 as libc::c_int {
        -(1 as libc::c_int)
    } else {
        0 as libc::c_int
    };
}
#[no_mangle]
pub unsafe extern "C" fn get_usec_since(mut old_time: *mut timeval) -> u_int64_t {
    let mut now: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut result: u_int64_t = 0 as libc::c_int as u_int64_t;
    gettimeofday(&mut now, 0 as *mut libc::c_void);
    while now.tv_sec > (*old_time).tv_sec {
        result = result.wrapping_add(1000000 as libc::c_int as u_int64_t);
        now.tv_sec -= 1;
        now.tv_sec;
    }
    return result.wrapping_add((now.tv_usec - (*old_time).tv_usec) as u_int64_t);
}
#[no_mangle]
pub unsafe extern "C" fn htonll(mut value: u_int64_t) -> u_int64_t {
    static mut necessary: libc::c_int = -(1 as libc::c_int);
    if necessary == -(1 as libc::c_int) {
        necessary = (5 as libc::c_int != __bswap_16(5 as libc::c_int as __uint16_t) as libc::c_int)
            as libc::c_int;
    }
    if necessary != 0 {
        return (__bswap_32(
            (value as libc::c_ulonglong & 0xffffffff as libc::c_longlong as libc::c_ulonglong)
                as __uint32_t,
        ) as u_int64_t)
            << 32 as libc::c_int
            | __bswap_32((value >> 32 as libc::c_int) as __uint32_t) as u_int64_t;
    } else {
        return value;
    };
}
#[no_mangle]
pub unsafe extern "C" fn make_transcript_filename(
    mut buffer: *mut libc::c_char,
    mut epoch: time_t,
    mut extension: *const libc::c_char,
) -> *mut libc::c_char {
    let mut gmt: tm = tm {
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
        tm_zone: 0 as *const libc::c_char,
    };
    gmtime_r(&mut epoch, &mut gmt);
    sprintf(
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
    return buffer;
}
#[no_mangle]
pub unsafe extern "C" fn ntohll(mut value: u_int64_t) -> u_int64_t {
    return htonll(value);
}
#[no_mangle]
pub unsafe extern "C" fn prepare_proof(
    mut buffer: *mut u_char,
    mut bytes: size_t,
    mut secret: *const u_char,
    mut digest: *mut u_char,
) -> *mut u_char {
    let mut secret_length: u_int32_t = 0;
    let mut offset: u_int32_t = 0;
    secret_length = strlen(secret as *mut libc::c_char) as u_int32_t;
    offset = 0 as libc::c_int as u_int32_t;
    while (offset as size_t) < bytes {
        let ref mut fresh0 = *buffer.offset(offset as isize);
        *fresh0 = (*fresh0 as libc::c_int
            ^ *secret.offset((offset % secret_length) as isize) as libc::c_int)
            as u_char;
        offset = offset.wrapping_add(1);
        offset;
    }
    md5_digest(buffer, bytes, digest);
    return digest;
}
#[no_mangle]
pub unsafe extern "C" fn read_line(
    mut fd: libc::c_int,
    mut buffer: *mut libc::c_char,
    mut buffer_length: size_t,
) -> libc::c_int {
    let mut buffer_offset: libc::c_int = 0 as libc::c_int;
    loop {
        if read(
            fd,
            buffer.offset(buffer_offset as isize) as *mut libc::c_void,
            1 as libc::c_int as size_t,
        ) <= 0 as libc::c_int as ssize_t
        {
            return error_handler(
                b"common.c\0" as *const u8 as *const libc::c_char,
                242 as libc::c_int,
                b"Could not read complete line of input\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        buffer_offset += 1;
        buffer_offset;
        if !(*buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
            != '\0' as i32
            && *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32
            && (buffer_offset as size_t) < buffer_length)
        {
            break;
        }
    }
    *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn fread_line(
    mut f: *mut FILE,
    mut buffer: *mut libc::c_char,
    mut buffer_length: size_t,
) -> libc::c_int {
    let mut buffer_offset: libc::c_int = 0 as libc::c_int;
    loop {
        if fread(
            buffer.offset(buffer_offset as isize) as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            f,
        ) <= 0 as libc::c_int as libc::c_ulong
        {
            return error_handler(
                b"common.c\0" as *const u8 as *const libc::c_char,
                266 as libc::c_int,
                b"Could not read complete line of input\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        buffer_offset += 1;
        buffer_offset;
        if !(*buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
            != '\0' as i32
            && *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) as libc::c_int
                != '\n' as i32
            && (buffer_offset as size_t) < buffer_length)
        {
            break;
        }
    }
    *buffer.offset((buffer_offset - 1 as libc::c_int) as isize) = '\0' as i32 as libc::c_char;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn usleep_that_works(mut usec: u_int64_t) {
    let mut sleep_time: u_int64_t =
        usec / 10000 as libc::c_int as u_int64_t * 10000 as libc::c_int as u_int64_t;
    let mut delay: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut now: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    gettimeofday(&mut now, 0 as *mut libc::c_void);
    if sleep_time >= 10000 as libc::c_int as u_int64_t {
        delay.tv_sec = (sleep_time / 1000000 as libc::c_int as u_int64_t) as __time_t;
        delay.tv_usec = (sleep_time % 1000000 as libc::c_int as u_int64_t) as __suseconds_t;
        select(
            0 as libc::c_int,
            0 as *mut fd_set,
            0 as *mut fd_set,
            0 as *mut fd_set,
            &mut delay,
        );
    }
    while get_usec_since(&mut now) < usec {}
}
#[no_mangle]
pub unsafe extern "C" fn get_udp_in_errors() -> u_int64_t {
    let mut f: *mut FILE = 0 as *mut FILE;
    let mut errs: u_int64_t = 0 as libc::c_int as u_int64_t;
    let mut buf: [libc::c_char; 512] = [0; 512];
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut len: libc::c_int = 0;
    let mut i: libc::c_int = 0;
    f = fopen(
        b"/proc/net/snmp\0" as *const u8 as *const libc::c_char,
        b"r\0" as *const u8 as *const libc::c_char,
    );
    if f.is_null() {
        return 0 as libc::c_int as u_int64_t;
    }
    while feof(f) == 0 {
        if (fgets(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            f,
        ))
        .is_null()
        {
            break;
        }
        if !(!(strstr(
            buf.as_mut_ptr(),
            b"Udp:\0" as *const u8 as *const libc::c_char,
        ))
        .is_null()
            && !(strstr(
                buf.as_mut_ptr(),
                b"InErrors\0" as *const u8 as *const libc::c_char,
            ))
            .is_null()
            && feof(f) == 0)
        {
            continue;
        }
        if (fgets(
            buf.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 512]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong) as libc::c_int,
            f,
        ))
        .is_null()
        {
            break;
        }
        len = strlen(buf.as_mut_ptr()) as libc::c_int;
        p = buf.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 3 as libc::c_int
            && !p.is_null()
            && p < buf
                .as_mut_ptr()
                .offset(len as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            p = strchr(p, ' ' as i32);
            i += 1;
            i;
            p = p.offset(1);
            p;
        }
        if !p.is_null()
            && p < buf
                .as_mut_ptr()
                .offset(len as isize)
                .offset(-(1 as libc::c_int as isize))
        {
            let fresh1 = p;
            p = p.offset(-1);
            errs = atol(fresh1) as u_int64_t;
        } else {
            errs = 0 as libc::c_int as u_int64_t;
        }
        break;
    }
    fclose(f);
    return errs;
}
#[no_mangle]
pub unsafe extern "C" fn full_write(
    mut fd: libc::c_int,
    mut buf: *const libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut written: ssize_t = 0 as libc::c_int as ssize_t;
    while (written as size_t) < count {
        let mut nwr: ssize_t = write(
            fd,
            buf.offset(written as isize),
            count.wrapping_sub(written as size_t),
        );
        if nwr < 0 as libc::c_int as ssize_t {
            fprintf(
                stderr,
                b"full_write(): %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return written;
        }
        written += nwr;
    }
    return written;
}
#[no_mangle]
pub unsafe extern "C" fn full_read(
    mut fd: libc::c_int,
    mut buf: *mut libc::c_void,
    mut count: size_t,
) -> ssize_t {
    let mut nread: ssize_t = 0 as libc::c_int as ssize_t;
    while (nread as size_t) < count {
        let mut nrd: ssize_t = read(
            fd,
            buf.offset(nread as isize),
            count.wrapping_sub(nread as size_t),
        );
        if nrd < 0 as libc::c_int as ssize_t {
            fprintf(
                stderr,
                b"full_read(): %s\n\0" as *const u8 as *const libc::c_char,
                strerror(*__errno_location()),
            );
            return nread;
        }
        nread += nrd;
    }
    return nread;
}
