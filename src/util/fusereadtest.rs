use crate::extc;
use ::libc;

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut t_start: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut t_isopen: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut t_closed: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut tdelta: libc::c_double = 0.;
    let mut tsleeps: libc::c_double = 0.;
    let mut randsleep: libc::c_ulong = 0;
    static mut block: *mut u8 = 0 as *const u8 as *mut u8;
    let mut i: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    let mut file: *mut extc::FILE = std::ptr::null_mut::<extc::FILE>();
    let mut readbytes: u32 = 0;
    let mut totalread: u64 = 0;
    if argc <= 1 as libc::c_int {
        extc::printf(
            b"Usage: ./fusereadtest filename [readsize in bytes]\n\n\0" as *const u8
                as *const libc::c_char,
        );
        return 0 as libc::c_int;
    }
    if ::core::mem::size_of::<extc::off_t>() as libc::c_ulong != 8 as libc::c_int as libc::c_ulong {
        extc::printf(
            b"Warning: Not compiled with 64-bit Large File Support, results can be unreliable\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    if argc > 2 as libc::c_int {
        readbytes = extc::atoi(*argv.offset(2 as libc::c_int as isize)) as u32;
    } else {
        readbytes = ((1250 as libc::c_int * 512 as libc::c_int * 2 as libc::c_int) as libc::c_ulong)
            .wrapping_mul(::core::mem::size_of::<libc::c_float>() as libc::c_ulong)
            as u32;
    }
    block = extc::malloc(readbytes as libc::c_ulong) as *mut u8;
    extc::gettimeofday(&mut t_start, std::ptr::null_mut::<libc::c_void>());
    extc::srand(*(&mut t_start as *mut extc::timeval as *mut libc::c_uint));
    file = extc::fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"r\0" as *const u8 as *const libc::c_char,
    );
    totalread = 0 as libc::c_int as u64;
    tsleeps = 0.0f64;
    extc::gettimeofday(&mut t_isopen, std::ptr::null_mut::<libc::c_void>());
    loop {
        j = extc::rand() / (2147483647 as libc::c_int / 4 as libc::c_int);
        i = 0 as libc::c_int;
        while i < j {
            totalread = (totalread as libc::c_ulong).wrapping_add(extc::fread(
                block as *mut libc::c_void,
                1 as libc::c_int as libc::c_ulong,
                readbytes as libc::c_ulong,
                file,
            )) as u64 as u64;
            i += 1;
            i;
        }
        if extc::feof(file) != 0 {
            break;
        }
        randsleep =
            (10000 as libc::c_longlong * 100 as libc::c_longlong * extc::rand() as libc::c_longlong
                / 2147483647 as libc::c_int as libc::c_longlong) as libc::c_ulong;
        tsleeps += 1e-6f64 * randsleep as libc::c_double;
        extc::usleep(randsleep as extc::__useconds_t);
    }
    extc::fclose(file);
    extc::gettimeofday(&mut t_closed, std::ptr::null_mut::<libc::c_void>());
    tdelta = (t_closed.tv_sec - t_start.tv_sec) as libc::c_double
        + 1e-6f64 * (t_closed.tv_usec - t_start.tv_usec) as libc::c_double;
    extc::printf(
        b"Start    = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        t_start.tv_sec as libc::c_ulong,
        t_start.tv_usec as libc::c_ulong,
    );
    extc::printf(
        b"Opened   = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        t_isopen.tv_sec as libc::c_ulong,
        t_isopen.tv_usec as libc::c_ulong,
    );
    extc::printf(
        b"Finished = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        t_closed.tv_sec as libc::c_ulong,
        t_closed.tv_usec as libc::c_ulong,
    );
    extc::printf(
        b"Delta    = %0.3lf sec\n\0" as *const u8 as *const libc::c_char,
        tdelta,
    );
    extc::printf(
        b"Sleeps   = %0.3lf sec\n\0" as *const u8 as *const libc::c_char,
        tsleeps,
    );
    extc::printf(
        b"Speed    = %0.3lf Mbps\n\0" as *const u8 as *const libc::c_char,
        totalread as libc::c_double * 8.0f64
            / (tdelta
                * 1024 as libc::c_int as libc::c_double
                * 1024 as libc::c_int as libc::c_double),
    );
    0 as libc::c_int
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
