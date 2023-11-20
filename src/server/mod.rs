use crate::extc;

pub mod config;
pub mod io;
pub mod log;
pub mod main;
pub mod network;
pub mod protocol;
pub mod transcript;

#[derive(Copy, Clone)]
#[repr(C)]
pub struct retransmission_t {
    pub request_type: u16,
    pub block: u32,
    pub error_rate: u32,
}

#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_parameter_t {
    pub epoch: extc::time_t,
    pub verbose_yn: u8,
    pub transcript_yn: u8,
    pub ipv6_yn: u8,
    pub tcp_port: u16,
    pub udp_buffer: u32,
    pub hb_timeout: u16,
    pub secret: *const u8,
    pub client: *const libc::c_char,
    pub finishhook: *const u8,
    pub allhook: *const u8,
    pub block_size: u32,
    pub file_size: u64,
    pub block_count: u32,
    pub target_rate: u32,
    pub error_rate: u32,
    pub ipd_time: u32,
    pub slower_num: u16,
    pub slower_den: u16,
    pub faster_num: u16,
    pub faster_den: u16,
    pub ringbuf: *mut libc::c_char,
    pub fileout: u16,
    pub slotnumber: libc::c_int,
    pub totalslots: libc::c_int,
    pub samplerate: libc::c_int,
    pub file_names: *mut *mut libc::c_char,
    pub file_sizes: *mut u64,
    pub file_name_size: u16,
    pub total_files: u16,
    pub wait_u_sec: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_transfer_t {
    pub parameter: *mut ttp_parameter_t,
    pub filename: *mut libc::c_char,
    pub file: *mut extc::FILE,
    pub vsib: *mut extc::FILE,
    pub transcript: *mut extc::FILE,
    pub udp_fd: libc::c_int,
    pub udp_address: *mut extc::sockaddr,
    pub udp_length: extc::socklen_t,
    pub ipd_current: libc::c_double,
    pub block: u32,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_session_t {
    pub parameter: *mut ttp_parameter_t,
    pub transfer: ttp_transfer_t,
    pub client_fd: libc::c_int,
    pub session_id: libc::c_int,
}
