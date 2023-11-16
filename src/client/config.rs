use ::libc;
extern "C" {
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type u_char = __u_char;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_parameter_t {
    pub server_name: *mut libc::c_char,
    pub server_port: u_int16_t,
    pub client_port: u_int16_t,
    pub udp_buffer: u_int32_t,
    pub verbose_yn: u_char,
    pub transcript_yn: u_char,
    pub ipv6_yn: u_char,
    pub output_mode: u_char,
    pub block_size: u_int32_t,
    pub target_rate: u_int32_t,
    pub rate_adjust: u_char,
    pub error_rate: u_int32_t,
    pub slower_num: u_int16_t,
    pub slower_den: u_int16_t,
    pub faster_num: u_int16_t,
    pub faster_den: u_int16_t,
    pub history: u_int16_t,
    pub lossless: u_char,
    pub losswindow_ms: u_int32_t,
    pub blockdump: u_char,
    pub passphrase: *mut libc::c_char,
    pub ringbuf: *mut libc::c_char,
}
pub static mut DEFAULT_BLOCK_SIZE: u_int32_t = 1024 as libc::c_int as u_int32_t;
pub static mut DEFAULT_TABLE_SIZE: libc::c_int = 4096 as libc::c_int;
pub static mut DEFAULT_SERVER_NAME: *const libc::c_char = b"localhost\0" as *const u8
    as *const libc::c_char;
pub static mut DEFAULT_SERVER_PORT: u_int16_t = 51038 as libc::c_int as u_int16_t;
pub static mut DEFAULT_CLIENT_PORT: u_int16_t = 51038 as libc::c_int as u_int16_t;
pub static mut DEFAULT_UDP_BUFFER: u_int32_t = 20000000 as libc::c_int as u_int32_t;
pub static mut DEFAULT_VERBOSE_YN: u_char = 1 as libc::c_int as u_char;
pub static mut DEFAULT_TRANSCRIPT_YN: u_char = 0 as libc::c_int as u_char;
pub static mut DEFAULT_IPV6_YN: u_char = 0 as libc::c_int as u_char;
pub static mut DEFAULT_OUTPUT_MODE: u_char = 1 as libc::c_int as u_char;
pub static mut DEFAULT_RATE_ADJUST: u_char = 0 as libc::c_int as u_char;
pub static mut DEFAULT_TARGET_RATE: u_int32_t = 650000000 as libc::c_int as u_int32_t;
pub static mut DEFAULT_ERROR_RATE: u_int32_t = 7500 as libc::c_int as u_int32_t;
pub static mut DEFAULT_SLOWER_NUM: u_int16_t = 25 as libc::c_int as u_int16_t;
pub static mut DEFAULT_SLOWER_DEN: u_int16_t = 24 as libc::c_int as u_int16_t;
pub static mut DEFAULT_FASTER_NUM: u_int16_t = 5 as libc::c_int as u_int16_t;
pub static mut DEFAULT_FASTER_DEN: u_int16_t = 6 as libc::c_int as u_int16_t;
pub static mut DEFAULT_HISTORY: u_int16_t = 25 as libc::c_int as u_int16_t;
pub static mut DEFAULT_NO_RETRANSMIT: u_char = 0 as libc::c_int as u_char;
pub static mut DEFAULT_LOSSLESS: u_char = 1 as libc::c_int as u_char;
pub static mut DEFAULT_LOSSWINDOW_MS: u_int32_t = 1000 as libc::c_int as u_int32_t;
pub static mut DEFAULT_BLOCKDUMP: u_char = 0 as libc::c_int as u_char;
pub static mut MAX_COMMAND_LENGTH: libc::c_int = 1024 as libc::c_int;
#[no_mangle]
pub unsafe extern "C" fn reset_client(mut parameter: *mut ttp_parameter_t) {
    if !((*parameter).server_name).is_null() {
        free((*parameter).server_name as *mut libc::c_void);
    }
    memset(
        parameter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_parameter_t>() as libc::c_ulong,
    );
    (*parameter).block_size = DEFAULT_BLOCK_SIZE;
    (*parameter).server_name = strdup(DEFAULT_SERVER_NAME);
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
        error_handler(
            b"config.c\0" as *const u8 as *const libc::c_char,
            138 as libc::c_int,
            b"Could not reset default server name\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
}
