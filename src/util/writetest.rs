use crate::extc;
use ::libc;

unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
    let mut start: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut stop: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut file: *mut extc::FILE = std::ptr::null_mut::<extc::FILE>();
    let mut block_size: u32 = 0;
    static mut block: *mut u8 = 0 as *const u8 as *mut u8;
    let mut file_size: u64 = 5000000000 as libc::c_longlong as u64;
    let mut sofar: u64 = 0 as libc::c_int as u64;
    if ::core::mem::size_of::<extc::off_t>() as libc::c_ulong != 8 as libc::c_int as libc::c_ulong {
        extc::printf(
            b"Warning: Not compiled with 64-bit Large File Support, results can be unreliable\n\0"
                as *const u8 as *const libc::c_char,
        );
    }
    block_size = (if argc > 2 as libc::c_int {
        extc::atoi(*argv.offset(2 as libc::c_int as isize))
    } else {
        32678 as libc::c_int
    }) as u32;
    block = extc::malloc(block_size as libc::c_ulong) as *mut u8;
    extc::gettimeofday(&mut start, std::ptr::null_mut::<libc::c_void>());
    file = extc::fopen(
        *argv.offset(1 as libc::c_int as isize),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    while sofar < file_size {
        extc::fwrite(
            block as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            block_size as libc::c_ulong,
            file,
        );
        sofar = sofar.wrapping_add(block_size as u64);
    }
    extc::fclose(file);
    extc::gettimeofday(&mut stop, std::ptr::null_mut::<libc::c_void>());
    let mut usec: i64 = 1000000 as libc::c_longlong
        * (stop.tv_sec - start.tv_sec) as libc::c_longlong
        + (stop.tv_usec - start.tv_usec) as libc::c_longlong;
    let mut bits: i64 = (file_size * 8 as libc::c_int as u64) as i64;
    extc::printf(
        b"Start time  = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        start.tv_sec as libc::c_ulong,
        start.tv_usec as libc::c_ulong,
    );
    extc::printf(
        b"Stop time   = %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        stop.tv_sec as libc::c_ulong,
        stop.tv_usec as libc::c_ulong,
    );
    extc::printf(
        b"Interval    = %0.3lf sec\n\0" as *const u8 as *const libc::c_char,
        usec as libc::c_double / 1000000.0f64,
    );
    extc::printf(
        b"Write speed = %0.3lf Mbps\n\0" as *const u8 as *const libc::c_char,
        bits as libc::c_double * 1.0f64 / usec as libc::c_double,
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
