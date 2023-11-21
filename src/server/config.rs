use crate::extc;
use ::libc;

pub static mut DEFAULT_BLOCK_SIZE: u32 = 1024 as libc::c_int as u32;
pub static mut DEFAULT_SECRET: *const u8 =
    b"kitten\0" as *const u8 as *const libc::c_char as *mut u8;
pub static mut DEFAULT_TCP_PORT: u16 = 51038 as libc::c_int as u16;
pub static mut DEFAULT_UDP_BUFFER: u32 = 20000000 as libc::c_int as u32;
pub static mut DEFAULT_VERBOSE_YN: u8 = 1 as libc::c_int as u8;
pub static mut DEFAULT_TRANSCRIPT_YN: u8 = 0 as libc::c_int as u8;
pub static mut DEFAULT_IPV6_YN: u8 = 0 as libc::c_int as u8;
pub static mut DEFAULT_HEARTBEAT_TIMEOUT: u16 = 15 as libc::c_int as u16;
pub unsafe fn reset_server(mut parameter: *mut super::ttp_parameter_t) {
    extc::memset(
        parameter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<super::ttp_parameter_t>() as libc::c_ulong,
    );
    (*parameter).block_size = DEFAULT_BLOCK_SIZE;
    (*parameter).secret = DEFAULT_SECRET;
    (*parameter).client = std::ptr::null::<libc::c_char>();
    (*parameter).tcp_port = DEFAULT_TCP_PORT;
    (*parameter).udp_buffer = DEFAULT_UDP_BUFFER;
    (*parameter).hb_timeout = DEFAULT_HEARTBEAT_TIMEOUT;
    (*parameter).verbose_yn = DEFAULT_VERBOSE_YN;
    (*parameter).transcript_yn = DEFAULT_TRANSCRIPT_YN;
    (*parameter).ipv6_yn = DEFAULT_IPV6_YN;
}
