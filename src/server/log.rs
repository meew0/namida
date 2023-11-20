use crate::extc;
use ::libc;

#[no_mangle]
pub unsafe extern "C" fn _log(
    mut log_file: *mut extc::FILE,
    mut format: *const libc::c_char,
    mut args: ...
) {
    let mut epoch: extc::time_t = 0;
    let mut now: *mut extc::tm = 0 as *mut extc::tm;
    let mut arg_list: ::core::ffi::VaListImpl;
    epoch = extc::time(0 as *mut extc::time_t);
    now = extc::localtime(&mut epoch);
    extc::fprintf(
        log_file,
        b"%04d-%02d-%02d %02d:%02d:%02d: \0" as *const u8 as *const libc::c_char,
        (*now).tm_year + 1900 as libc::c_int,
        (*now).tm_mon + 1 as libc::c_int,
        (*now).tm_mday,
        (*now).tm_hour,
        (*now).tm_min,
        (*now).tm_sec,
    );
    arg_list = args.clone();
    extc::vfprintf(log_file, format, arg_list.as_va_list());
    extc::fprintf(log_file, b"\n\0" as *const u8 as *const libc::c_char);
    extc::fflush(log_file);
}
