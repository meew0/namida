use super::ttp_parameter_t;
use crate::extc;
use ::libc;

pub static mut DEFAULT_BLOCK_SIZE: u32 = 1024 as libc::c_int as u32;
pub static mut DEFAULT_TABLE_SIZE: libc::c_int = 4096 as libc::c_int;
pub static mut DEFAULT_SERVER_NAME: *const libc::c_char =
    b"localhost\0" as *const u8 as *const libc::c_char;
pub static mut DEFAULT_SERVER_PORT: u16 = 51038 as libc::c_int as u16;
pub static mut DEFAULT_CLIENT_PORT: u16 = 51038 as libc::c_int as u16;
pub static mut DEFAULT_UDP_BUFFER: u32 = 20000000 as libc::c_int as u32;
pub static mut DEFAULT_VERBOSE_YN: u8 = 1 as libc::c_int as u8;
pub static mut DEFAULT_TRANSCRIPT_YN: u8 = 0 as libc::c_int as u8;
pub static mut DEFAULT_IPV6_YN: u8 = 0 as libc::c_int as u8;
pub static mut DEFAULT_OUTPUT_MODE: u8 = 1 as libc::c_int as u8;
pub static mut DEFAULT_RATE_ADJUST: u8 = 0 as libc::c_int as u8;
pub static mut DEFAULT_TARGET_RATE: u32 = 650000000 as libc::c_int as u32;
pub static mut DEFAULT_ERROR_RATE: u32 = 7500 as libc::c_int as u32;
pub static mut DEFAULT_SLOWER_NUM: u16 = 25 as libc::c_int as u16;
pub static mut DEFAULT_SLOWER_DEN: u16 = 24 as libc::c_int as u16;
pub static mut DEFAULT_FASTER_NUM: u16 = 5 as libc::c_int as u16;
pub static mut DEFAULT_FASTER_DEN: u16 = 6 as libc::c_int as u16;
pub static mut DEFAULT_HISTORY: u16 = 25 as libc::c_int as u16;
pub static mut DEFAULT_NO_RETRANSMIT: u8 = 0 as libc::c_int as u8;
pub static mut DEFAULT_LOSSLESS: u8 = 1 as libc::c_int as u8;
pub static mut DEFAULT_LOSSWINDOW_MS: u32 = 1000 as libc::c_int as u32;
pub static mut DEFAULT_BLOCKDUMP: u8 = 0 as libc::c_int as u8;
pub static mut MAX_COMMAND_LENGTH: libc::c_int = 1024 as libc::c_int;
pub unsafe fn reset_client(mut parameter: *mut ttp_parameter_t) {
    if !((*parameter).server_name).is_null() {
        extc::free((*parameter).server_name as *mut libc::c_void);
    }
    extc::memset(
        parameter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_parameter_t>() as libc::c_ulong,
    );
    (*parameter).block_size = DEFAULT_BLOCK_SIZE;
    (*parameter).server_name = extc::strdup(DEFAULT_SERVER_NAME);
    (*parameter).server_port = DEFAULT_SERVER_PORT;
    (*parameter).client_port = DEFAULT_CLIENT_PORT;
    (*parameter).udp_buffer = DEFAULT_UDP_BUFFER;
    (*parameter).verbose_yn = DEFAULT_VERBOSE_YN;
    (*parameter).transcript_yn = DEFAULT_TRANSCRIPT_YN;
    (*parameter).ipv6_yn = DEFAULT_IPV6_YN;
    (*parameter).output_mode = DEFAULT_OUTPUT_MODE;
    (*parameter).target_rate = DEFAULT_TARGET_RATE;
    (*parameter).rate_adjust = DEFAULT_RATE_ADJUST;
    (*parameter).error_rate = DEFAULT_ERROR_RATE;
    (*parameter).slower_num = DEFAULT_SLOWER_NUM;
    (*parameter).slower_den = DEFAULT_SLOWER_DEN;
    (*parameter).faster_num = DEFAULT_FASTER_NUM;
    (*parameter).faster_den = DEFAULT_FASTER_DEN;
    (*parameter).history = DEFAULT_HISTORY;
    (*parameter).lossless = DEFAULT_LOSSLESS;
    (*parameter).losswindow_ms = DEFAULT_LOSSWINDOW_MS;
    (*parameter).blockdump = DEFAULT_BLOCKDUMP;
    if ((*parameter).server_name).is_null() {
        crate::common::error::error_handler(
            b"config.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            b"Could not reset default server name\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
}
