use ::libc;
extern "C" {
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __time_t = libc::c_long;
pub type u_char = __u_char;
pub type time_t = __time_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_parameter_t {
    pub epoch: time_t,
    pub verbose_yn: u_char,
    pub transcript_yn: u_char,
    pub ipv6_yn: u_char,
    pub tcp_port: u_int16_t,
    pub udp_buffer: u_int32_t,
    pub hb_timeout: u_int16_t,
    pub secret: *const u_char,
    pub client: *const libc::c_char,
    pub finishhook: *const u_char,
    pub allhook: *const u_char,
    pub block_size: u_int32_t,
    pub file_size: u_int64_t,
    pub block_count: u_int32_t,
    pub target_rate: u_int32_t,
    pub error_rate: u_int32_t,
    pub ipd_time: u_int32_t,
    pub slower_num: u_int16_t,
    pub slower_den: u_int16_t,
    pub faster_num: u_int16_t,
    pub faster_den: u_int16_t,
    pub ringbuf: *mut libc::c_char,
    pub fileout: u_int16_t,
    pub slotnumber: libc::c_int,
    pub totalslots: libc::c_int,
    pub samplerate: libc::c_int,
    pub file_names: *mut *mut libc::c_char,
    pub file_sizes: *mut size_t,
    pub file_name_size: u_int16_t,
    pub total_files: u_int16_t,
    pub wait_u_sec: libc::c_long,
}
#[no_mangle]
pub static mut DEFAULT_BLOCK_SIZE: u_int32_t = 1024 as libc::c_int as u_int32_t;
#[no_mangle]
pub static mut DEFAULT_SECRET: *const u_char = b"kitten\0" as *const u8
    as *const libc::c_char as *mut u_char;
#[no_mangle]
pub static mut DEFAULT_TCP_PORT: u_int16_t = 51038 as libc::c_int as u_int16_t;
#[no_mangle]
pub static mut DEFAULT_UDP_BUFFER: u_int32_t = 20000000 as libc::c_int as u_int32_t;
#[no_mangle]
pub static mut DEFAULT_VERBOSE_YN: u_char = 1 as libc::c_int as u_char;
#[no_mangle]
pub static mut DEFAULT_TRANSCRIPT_YN: u_char = 0 as libc::c_int as u_char;
#[no_mangle]
pub static mut DEFAULT_IPV6_YN: u_char = 0 as libc::c_int as u_char;
#[no_mangle]
pub static mut DEFAULT_HEARTBEAT_TIMEOUT: u_int16_t = 15 as libc::c_int as u_int16_t;
#[no_mangle]
pub unsafe extern "C" fn reset_server(mut parameter: *mut ttp_parameter_t) {
    memset(
        parameter as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_parameter_t>() as libc::c_ulong,
    );
    (*parameter).block_size = DEFAULT_BLOCK_SIZE;
    (*parameter).secret = DEFAULT_SECRET;
    (*parameter).client = 0 as *const libc::c_char;
    (*parameter).tcp_port = DEFAULT_TCP_PORT;
    (*parameter).udp_buffer = DEFAULT_UDP_BUFFER;
    (*parameter).hb_timeout = DEFAULT_HEARTBEAT_TIMEOUT;
    (*parameter).verbose_yn = DEFAULT_VERBOSE_YN;
    (*parameter).transcript_yn = DEFAULT_TRANSCRIPT_YN;
    (*parameter).ipv6_yn = DEFAULT_IPV6_YN;
}
