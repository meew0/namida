pub mod command;
pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod ring;
pub mod transcript;

use std::{fmt::Display, sync::Arc};

use crate::extc;

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

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum OutputMode {
    Default,
    Screen,
}

#[derive(Clone)]
pub struct Fraction {
    numerator: u16,
    denominator: u16,
}

impl Display for Fraction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.numerator, self.denominator)
    }
}

#[derive(Clone, clap::Args)]
pub struct Parameter {
    #[arg(long = "server", default_value_t = config::DEFAULT_SERVER_NAME.to_owned())]
    pub server_name: String,

    #[arg(long = "port", default_value_t = config::DEFAULT_SERVER_PORT)]
    pub server_port: u16,

    #[arg(long = "udpport", default_value_t = config::DEFAULT_CLIENT_PORT)]
    pub client_port: u16,

    #[arg(long = "buffer", default_value_t = config::DEFAULT_UDP_BUFFER)]
    pub udp_buffer: u32,

    #[arg(long = "quiet", action = clap::ArgAction::SetFalse)]
    pub verbose_yn: bool,

    #[arg(long = "transcript")]
    pub transcript_yn: bool,

    #[arg(long = "ipv6")]
    pub ipv6_yn: bool,

    #[arg(long = "output", value_enum, default_value_t = OutputMode::Default)]
    pub output_mode: OutputMode,

    #[arg(long = "blocksize", default_value_t = config::DEFAULT_BLOCK_SIZE)]
    pub block_size: u32,

    #[arg(long = "rate", default_value_t = config::DEFAULT_TARGET_RATE)]
    pub target_rate: u64,

    #[arg(long = "rateadjust")]
    pub rate_adjust: bool,

    #[arg(long = "error", default_value_t = config::DEFAULT_ERROR_RATE)]
    pub error_rate: u32,

    #[arg(long = "slower", value_parser = clap::builder::ValueParser::new(command::parse_fraction), default_value_t = config::DEFAULT_SLOWER)]
    pub slower: Fraction,

    #[arg(long = "faster", value_parser = clap::builder::ValueParser::new(command::parse_fraction), default_value_t = config::DEFAULT_FASTER)]
    pub faster: Fraction,

    #[arg(long = "history", default_value_t = config::DEFAULT_HISTORY)]
    pub history: u16,

    #[arg(long = "lossy", action = clap::ArgAction::SetFalse)]
    pub lossless: bool,

    #[arg(long = "losswindow", default_value_t = config::DEFAULT_LOSSWINDOW_MS)]
    pub losswindow_ms: u32,

    #[arg(long = "blockdump")]
    pub blockdump: bool,

    #[arg(long = "passphrase")]
    pub passphrase: Option<String>,
}

pub struct Transfer {
    pub epoch: i64,
    pub remote_filename: Option<String>,
    pub local_filename: Option<String>,
    pub file: Option<std::fs::File>,
    pub transcript: Option<std::fs::File>,
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
            transcript: None,
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
