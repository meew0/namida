use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn rand() -> libc::c_int;
    fn srand(__seed: libc::c_uint);
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
    fn feof(__stream: *mut FILE) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn usleep(__useconds: __useconds_t) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __useconds_t = libc::c_uint;
pub type __suseconds_t = libc::c_long;
pub type u_char = __u_char;
pub type off_t = __off64_t;
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
    let mut t_start: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut t_isopen: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut t_closed: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tdelta: libc::c_double = 0.;
    let mut tsleeps: libc::c_double = 0.;
    let mut randsleep: libc::c_ulong = 0;
    static mut block: *mut u_char = 0 as *const u_char as *mut u_char;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut file: *mut FILE = 0 as *mut FILE;
    let mut readbytes: u_int32_t = 0;
    let mut totalread: u_int64_t = 0;
    if argc <= 1 as libc::c_int {
        printf(
            b"Usage: ./fusereadtest filename [readsize in bytes]\n\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ::core::mem::size_of::<off_t>() as libc::c_ulong != 8 as libc::c_int as libc::c_ulong {
        printf(
            b"Warning: Not compiled with 64-bit Large File Support, results can be unreliable\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if argc > 2 as libc::c_int {
        readbytes = atoi(*argv.offset(2 as libc::c_int as isize)) as u_int32_t;
    } else {
        readbytes = ((1250 as libc::c_int * 512 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as u_int32_t;
    }
    block = malloc(readbytes as libc::c_ulong) as *mut u_char;
    gettimeofday(&mut t_start, 0 as *mut libc::c_void);
    srand(*(&mut t_start as *mut timeval as *mut libc::c_uint));
    file = fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    totalread = 0 as libc::c_int as u_int64_t;
    tsleeps = 0.0f64;
    gettimeofday(&mut t_isopen, 0 as *mut libc::c_void);
    loop {
        j = rand() / (2147483647 as libc::c_int / 4 as libc::c_int);
        i = 0 as libc::c_int;
        while i < j {
            totalread = (totalread as libc::c_ulong).wrapping_add(fread(
                block as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                readbytes as libc::c_ulong,
                file,
            )) as u_int64_t as u_int64_t;
            i += 1;
            i;
        }
        if feof(file) != 0 {
            break;
        }
        randsleep =
            (10000 as libc::c_longlong * 100 as libc::c_longlong * rand() as libc::c_longlong
                / 2147483647 as libc::c_int as libc::c_longlong) as libc::c_ulong;
        tsleeps += 1e-6f64 * randsleep as libc::c_double;
        usleep(randsleep as __useconds_t);
    }
    fclose(file);
    gettimeofday(&mut t_closed, 0 as *mut libc::c_void);
    tdelta = (t_closed.tv_sec - t_start.tv_sec) as libc::c_double
        + 1e-6f64 * (t_closed.tv_usec - t_start.tv_usec) as libc::c_double;
    printf(
        b"Start    = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        t_start.tv_sec as libc::c_ulong,
        t_start.tv_usec as libc::c_ulong,
    );
    printf(
        b"Opened   = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        t_isopen.tv_sec as libc::c_ulong,
        t_isopen.tv_usec as libc::c_ulong,
    );
    printf(
        b"Finished = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        t_closed.tv_sec as libc::c_ulong,
        t_closed.tv_usec as libc::c_ulong,
    );
    printf(
        b"Delta    = %0.3lf sec\n\0" as *const u8 as *const libc::c_char,
        tdelta,
    );
    printf(
        b"Sleeps   = %0.3lf sec\n\0" as *const u8 as *const libc::c_char,
        tsleeps,
    );
    printf(
        b"Speed    = %0.3lf Mbps\n\0" as *const u8 as *const libc::c_char,
        totalread as libc::c_double * 8.0f64
            / (tdelta
                * 1024 as libc::c_int as libc::c_double
                * 1024 as libc::c_int as libc::c_double),
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
