pub mod command;
pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod ring;
pub mod transcript;

use std::sync::Arc;

use crate::extc;

#[derive(Copy, Clone)]
pub struct Command {
    pub count: u8,
    pub text: [*const libc::c_char; 10],
}
#[derive(Copy, Clone)]
pub struct Retransmission {
    pub request_type: u16,
    pub block: u32,
    pub error_rate: u32,
}

#[derive(Copy, Clone, Default)]
pub struct Statistics {
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
pub struct Retransmit {
    pub table: *mut u32,
    pub table_size: u32,
    pub index_max: u32,
}

impl Default for Retransmit {
    fn default() -> Self {
        Self {
            table: std::ptr::null_mut(),
            table_size: Default::default(),
            index_max: Default::default(),
        }
    }
}

#[derive(Clone)]
pub struct Parameter {
    pub server_name: String,
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
    pub passphrase: Option<String>,
}

impl Default for Parameter {
    fn default() -> Self {
        Self {
            block_size: config::DEFAULT_BLOCK_SIZE,
            server_name: config::DEFAULT_SERVER_NAME.to_owned(),
            server_port: config::DEFAULT_SERVER_PORT,
            client_port: config::DEFAULT_CLIENT_PORT,
            udp_buffer: config::DEFAULT_UDP_BUFFER,
            verbose_yn: config::DEFAULT_VERBOSE_YN,
            transcript_yn: config::DEFAULT_TRANSCRIPT_YN,
            ipv6_yn: config::DEFAULT_IPV6_YN,
            output_mode: config::DEFAULT_OUTPUT_MODE,
            target_rate: config::DEFAULT_TARGET_RATE,
            rate_adjust: config::DEFAULT_RATE_ADJUST,
            error_rate: config::DEFAULT_ERROR_RATE,
            slower_num: config::DEFAULT_SLOWER_NUM,
            slower_den: config::DEFAULT_SLOWER_DEN,
            faster_num: config::DEFAULT_FASTER_NUM,
            faster_den: config::DEFAULT_FASTER_DEN,
            history: config::DEFAULT_HISTORY,
            lossless: config::DEFAULT_LOSSLESS,
            losswindow_ms: config::DEFAULT_LOSSWINDOW_MS,
            blockdump: config::DEFAULT_BLOCKDUMP,
            passphrase: None,
        }
    }
}

pub struct Transfer {
    pub epoch: i64,
    pub remote_filename: Option<String>,
    pub local_filename: Option<String>,
    pub file: Option<std::fs::File>,
    pub transcript: *mut extc::FILE,
    pub udp_fd: libc::c_int,
    pub file_size: u64,
    pub block_count: u32,
    pub next_block: u32,
    pub gapless_to_block: u32,
    pub retransmit: Retransmit,
    pub stats: Statistics,
    pub ring_buffer: Option<Arc<ring::RingBuffer>>,
    pub received: *mut u8,
    pub blocks_left: u32,
    pub restart_pending: u8,
    pub restart_lastidx: u32,
    pub restart_wireclearidx: u32,
    pub on_wire_estimate: u32,
}

impl Default for Transfer {
    fn default() -> Self {
        Self {
            epoch: Default::default(),
            remote_filename: None,
            local_filename: None,
            file: None,
            transcript: std::ptr::null_mut(),
            udp_fd: Default::default(),
            file_size: Default::default(),
            block_count: Default::default(),
            next_block: Default::default(),
            gapless_to_block: Default::default(),
            retransmit: Default::default(),
            stats: Default::default(),
            ring_buffer: Default::default(),
            received: std::ptr::null_mut(),
            blocks_left: Default::default(),
            restart_pending: Default::default(),
            restart_lastidx: Default::default(),
            restart_wireclearidx: Default::default(),
            on_wire_estimate: Default::default(),
        }
    }
}

pub struct Session {
    pub transfer: Transfer,
    pub server: *mut extc::FILE,
    pub server_address: *mut extc::sockaddr,
    pub server_address_length: extc::socklen_t,
}
