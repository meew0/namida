pub mod command;
pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod ring;
pub mod transcript;

use std::{
    net::UdpSocket,
    path::PathBuf,
    sync::Arc,
    time::{Duration, Instant},
};

use crate::{
    common::SocketWrapper,
    types::{BlockIndex, BlockSize, ErrorRate, FileSize, Fraction, TargetRate, UdpErrors},
};

#[derive(Clone, Default)]
pub struct Statistics {
    pub start_time: Option<Instant>,
    pub stop_time: Option<Instant>,
    pub this_time: Option<Instant>,
    pub this_blocks: BlockIndex,
    pub this_retransmits: BlockIndex,
    pub total_blocks: BlockIndex,
    pub total_retransmits: BlockIndex,
    pub total_recvd_retransmits: BlockIndex,
    pub total_lost: BlockIndex,
    pub this_flow_originals: BlockIndex,
    pub this_flow_retransmitteds: BlockIndex,
    pub this_transmit_rate: f64,
    pub transmit_rate: f64,
    pub this_retransmit_rate: f64,
    pub error_rate: f64,
    pub udp_errors: UdpErrors,
}

#[derive(Clone)]
pub struct Retransmit {
    pub previous_table: Vec<BlockIndex>,
    pub next_table: Vec<BlockIndex>,
}

impl Retransmit {
    pub const MAX_RETRANSMISSION_BUFFER: u32 = 2048;

    pub fn swap_tables(&mut self) {
        std::mem::swap(&mut self.previous_table, &mut self.next_table);
    }
}

impl Default for Retransmit {
    fn default() -> Self {
        Self {
            previous_table: vec![BlockIndex(0); Self::MAX_RETRANSMISSION_BUFFER as usize],
            next_table: vec![BlockIndex(0); Self::MAX_RETRANSMISSION_BUFFER as usize],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum OutputMode {
    Line,
    Screen,
}

#[derive(Clone, clap::Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct Parameter {
    #[arg(long = "server", default_value_t = config::DEFAULT_SERVER_NAME.to_owned())]
    pub server: String,

    /// Specify a static UDP port to receive data on. If not specified, a random port will be used.
    #[arg(long = "udpport")]
    pub client_port: Option<u16>,

    /// By default, the client will have the server discover its public UDP address by sending some
    /// data to it. If this option is set, this behaviour will be disabled and data will always be
    /// sent to the client's TCP address combined with the port to which the UDP socket is bound.
    /// This will make the file initialisation process simpler and more deterministic, but it will
    /// cause problems if the client is behind NAT.
    #[arg(long = "no-discovery", action = clap::ArgAction::SetFalse)]
    pub discovery: bool,

    /// If this flag is present, the client will not encrypt the connection. The same flag must also
    /// be specified on the server.
    #[arg(long = "unencrypted", action = clap::ArgAction::SetFalse)]
    pub encrypted: bool,

    #[arg(long = "buffer", default_value_t = config::DEFAULT_UDP_BUFFER)]
    pub udp_buffer: u32,

    #[arg(long = "quiet", action = clap::ArgAction::SetFalse)]
    pub verbose_yn: bool,

    #[arg(long = "transcript")]
    pub transcript_yn: bool,

    #[arg(long = "ipv6")]
    pub ipv6_yn: bool,

    #[arg(long = "output", value_enum, default_value_t = OutputMode::Line)]
    pub output_mode: OutputMode,

    #[arg(long = "blocksize", default_value_t = config::DEFAULT_BLOCK_SIZE)]
    pub block_size: BlockSize,

    #[arg(long = "rate", value_parser = clap::builder::ValueParser::new(command::parse_rate), default_value_t = config::DEFAULT_TARGET_RATE)]
    pub target_rate: TargetRate,

    #[arg(long = "rateadjust")]
    pub rate_adjust: bool,

    #[arg(long = "error", default_value_t = config::DEFAULT_ERROR_RATE)]
    pub error_rate: ErrorRate,

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

    /// Specifies the path to a file from which the pre-shared key will be loaded. Only the first 32
    /// bytes of the file will be used as the PSK. If not specified, a hard-coded key will be used;
    /// this is not recommended.
    #[arg(long = "secret")]
    pub secret_file: Option<PathBuf>,

    #[arg(skip = *crate::common::DEFAULT_SECRET)]
    pub secret: [u8; 32],
}

#[derive(Default)]
pub struct Transfer {
    pub epoch: Duration,
    pub remote_filename: Option<PathBuf>,
    pub local_filename: Option<PathBuf>,
    pub file: Option<std::fs::File>,
    pub transcript: Option<std::fs::File>,
    pub udp_socket: Option<UdpSocket>,
    pub file_size: FileSize,
    pub block_count: BlockIndex,
    pub next_block: BlockIndex,
    pub gapless_to_block: BlockIndex,
    pub retransmit: Retransmit,
    pub stats: Statistics,
    pub ring_buffer: Option<Arc<ring::Buffer>>,
    pub received: Vec<u8>,
    pub blocks_left: BlockIndex,
    pub restart_pending: bool,
    pub restart_lastidx: BlockIndex,
    pub restart_wireclearidx: BlockIndex,
    pub on_wire_estimate: BlockIndex,
}

pub struct Session {
    pub transfer: Transfer,
    pub server: SocketWrapper,
}
