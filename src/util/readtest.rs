use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(__stream: *mut FILE, __off: __off64_t, __whence: libc::c_int) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type off_t = __off64_t;
pub type int64_t = __int64_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut start: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut stop: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut block_size: u_int32_t = 0;
    static mut block: *mut u_char = 0 as *const u_char as *mut u_char;
    let mut file_size: u_int64_t = 0;
    if ::core::mem::size_of::<off_t>() as libc::c_ulong != 8 as libc::c_int as libc::c_ulong {
        printf(
            b"Warning: Not compiled with 64-bit Large File Support, results can be unreliable\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    block_size = (if argc > 2 as libc::c_int {
        atoi(*argv.offset(2 as libc::c_int as isize))
    } else {
        32678 as libc::c_int
    }) as u_int32_t;
    block = malloc(block_size as libc::c_ulong) as *mut u_char;
    gettimeofday(&mut start, 0 as *mut libc::c_void);
    file = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    fseeko(file, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
    file_size = ftello(file) as u_int64_t;
    fseeko(file, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    while fread(
        block as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        block_size as libc::c_ulong,
        file,
    ) > 0 as libc::c_int as libc::c_ulong
    {}
    fclose(file);
    gettimeofday(&mut stop, 0 as *mut libc::c_void);
    let mut usec: int64_t =
        (1000000 as libc::c_longlong * (stop.tv_sec - start.tv_sec) as libc::c_longlong
            + (stop.tv_usec - start.tv_usec) as libc::c_longlong) as int64_t;
    let mut bits: int64_t = (file_size * 8 as libc::c_int as u_int64_t) as int64_t;
    printf(
        b"Start time = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        start.tv_sec as libc::c_ulong,
        start.tv_usec as libc::c_ulong,
    );
    printf(
        b"Stop time  = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        stop.tv_sec as libc::c_ulong,
        stop.tv_usec as libc::c_ulong,
    );
    printf(
        b"Interval   = %0.3lf sec\n\0" as *const u8 as *const libc::c_char,
        usec as libc::c_double / 1000000.0f64,
    );
    printf(
        b"Read speed = %0.3lf Mbps\n\0" as *const u8 as *const libc::c_char,
        bits as libc::c_double * 1.0f64 / usec as libc::c_double,
    );
    return 0 as libc::c_int;
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
