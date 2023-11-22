use crate::extc;

pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod transcript;

#[derive(Copy, Clone)]
pub struct Retransmission {
    pub request_type: u16,
    pub block: u32,
    pub error_rate: u32,
}

#[derive(Clone)]
pub struct Parameter {
    pub epoch: extc::time_t,
    pub verbose_yn: u8,
    pub transcript_yn: u8,
    pub ipv6_yn: u8,
    pub tcp_port: u16,
    pub udp_buffer: u32,
    pub hb_timeout: u16,
    pub secret: String,
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

impl Default for Parameter {
    fn default() -> Self {
        Self {
            epoch: 0,
            verbose_yn: config::DEFAULT_VERBOSE_YN,
            transcript_yn: config::DEFAULT_TRANSCRIPT_YN,
            ipv6_yn: config::DEFAULT_IPV6_YN,
            tcp_port: config::DEFAULT_TCP_PORT,
            udp_buffer: config::DEFAULT_UDP_BUFFER,
            hb_timeout: config::DEFAULT_HEARTBEAT_TIMEOUT,
            secret: config::DEFAULT_SECRET.to_owned(),
            client: std::ptr::null::<libc::c_char>(),
            finishhook: std::ptr::null::<u8>(),
            allhook: std::ptr::null::<u8>(),
            block_size: config::DEFAULT_BLOCK_SIZE,
            file_size: 0,
            block_count: 0,
            target_rate: 0,
            error_rate: 0,
            ipd_time: 0,
            slower_num: 0,
            slower_den: 0,
            faster_num: 0,
            faster_den: 0,
            fileout: 0,
            slotnumber: 0,
            totalslots: 0,
            samplerate: 0,
            file_names: std::ptr::null_mut::<*mut libc::c_char>(),
            file_sizes: std::ptr::null_mut::<u64>(),
            file_name_size: 0,
            total_files: 0,
            wait_u_sec: 0,
        }
    }
}

#[derive(Copy, Clone)]
pub struct Transfer {
    pub parameter: *mut Parameter,
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
pub struct Session {
    pub transfer: Transfer,
    pub client_fd: libc::c_int,
    pub session_id: libc::c_int,
}
