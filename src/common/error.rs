use crate::extc;
use ::libc;

#[no_mangle]
pub static mut g_error: [libc::c_char; 512] = [0; 512];
pub unsafe fn error_handler(
    mut file: *const libc::c_char,
    mut line: libc::c_int,
    mut message: *const libc::c_char,
    mut fatal_yn: libc::c_int,
) -> libc::c_int {
    extc::fprintf(
        extc::stderr,
        b"%s: %s\n\0" as *const u8 as *const libc::c_char,
        if fatal_yn != 0 {
            b"Error\0" as *const u8 as *const libc::c_char
        } else {
            b"Warning\0" as *const u8 as *const libc::c_char
        },
        message,
    );
    if fatal_yn != 0 {
        extc::exit(1 as libc::c_int);
    }
    return -(1 as libc::c_int);
}
