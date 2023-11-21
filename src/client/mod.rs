pub mod command;
pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod ring;
pub mod transcript;

use crate::extc;

#[derive(Copy, Clone)]
pub struct command_t {
    pub count: u8,
    pub text: [*const libc::c_char; 10],
}
#[derive(Copy, Clone)]
pub struct retransmission_t {
    pub request_type: u16,
    pub block: u32,
    pub error_rate: u32,
}
#[derive(Copy, Clone)]
pub struct statistics_t {
    pub start_time: extc::timeval,
    pub stop_time: extc::timeval,
    pub this_time: extc::timeval,
    pub this_blocks: u32,
    pub this_retransmits: u32,
    pub total_blocks: u32,
    pub total_retransmits: u32,
    pub total_recvd_retransmits: u32,
    pub total_lost: u32,
    pub this_flow_originals: u32,
    pub this_flow_retransmitteds: u32,
    pub this_transmit_rate: f64,
    pub transmit_rate: f64,
    pub this_retransmit_rate: f64,
    pub error_rate: f64,
    pub start_udp_errors: u64,
    pub this_udp_errors: u64,
}
#[derive(Copy, Clone)]
pub struct retransmit_t {
    pub table: *mut u32,
    pub table_size: u32,
    pub index_max: u32,
}
#[derive(Copy, Clone)]
pub struct ttp_parameter_t {
    pub server_name: *mut libc::c_char,
    pub server_port: u16,
    pub client_port: u16,
    pub udp_buffer: u32,
    pub verbose_yn: u8,
    pub transcript_yn: u8,
    pub ipv6_yn: u8,
    pub output_mode: u8,
    pub block_size: u32,
    pub target_rate: u32,
    pub rate_adjust: u8,
    pub error_rate: u32,
    pub slower_num: u16,
    pub slower_den: u16,
    pub faster_num: u16,
    pub faster_den: u16,
    pub history: u16,
    pub lossless: u8,
    pub losswindow_ms: u32,
    pub blockdump: u8,
    pub passphrase: *mut libc::c_char,
    pub ringbuf: *mut libc::c_char,
}

pub struct ttp_transfer_t {
    pub epoch: i64,
    pub remote_filename: *const libc::c_char,
    pub local_filename: *const libc::c_char,
    pub file: *mut extc::FILE,
    pub vsib: *mut extc::FILE,
    pub transcript: *mut extc::FILE,
    pub udp_fd: libc::c_int,
    pub file_size: u64,
    pub block_count: u32,
    pub next_block: u32,
    pub gapless_to_block: u32,
    pub retransmit: retransmit_t,
    pub stats: statistics_t,
    pub ring_buffer: ring::RingBuffer,
    pub received: *mut u8,
    pub blocks_left: u32,
    pub restart_pending: u8,
    pub restart_lastidx: u32,
    pub restart_wireclearidx: u32,
    pub on_wire_estimate: u32,
}

pub struct ttp_session_t {
    pub parameter: *mut ttp_parameter_t,
    pub transfer: ttp_transfer_t,
    pub server: *mut extc::FILE,
    pub server_address: *mut extc::sockaddr,
    pub server_address_length: extc::socklen_t,
}
