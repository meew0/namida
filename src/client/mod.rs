pub mod command;
pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod ring;
pub mod transcript;

use std::{fmt::Display, io::Write, net::TcpStream, sync::Arc};

use anyhow::bail;

use crate::{
    extc,
    message::ServerToClient,
    types::{BlockIndex, BlockSize, Epoch, ErrorRate, FileSize, Fraction, TargetRate},
};

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

#[derive(Clone)]
pub struct Retransmit {
    pub previous_table: Vec<BlockIndex>,
    pub next_table: Vec<BlockIndex>,
}

impl Retransmit {
    pub fn swap_tables(&mut self) {
        std::mem::swap(&mut self.previous_table, &mut self.next_table);
    }
}

impl Default for Retransmit {
    fn default() -> Self {
        Self {
            previous_table: vec![BlockIndex(0); 2048],
            next_table: vec![BlockIndex(0); 2048],
        }
    }
}

#[derive(Copy, Clone, PartialEq, Eq, PartialOrd, Ord, clap::ValueEnum)]
pub enum OutputMode {
    Default,
    Screen,
}

#[derive(Clone, clap::Args)]
pub struct Parameter {
    #[arg(long = "server", default_value_t = config::DEFAULT_SERVER_NAME.to_owned())]
    pub server: String,

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
    pub block_size: BlockSize,

    #[arg(long = "rate", default_value_t = config::DEFAULT_TARGET_RATE)]
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

    #[arg(long = "passphrase")]
    pub passphrase: Option<String>,
}

#[derive(Default)]
pub struct Transfer {
    pub epoch: Epoch,
    pub remote_filename: Option<String>,
    pub local_filename: Option<String>,
    pub file: Option<std::fs::File>,
    pub transcript: Option<std::fs::File>,
    pub udp_fd: libc::c_int,
    pub file_size: FileSize,
    pub block_count: BlockIndex,
    pub next_block: BlockIndex,
    pub gapless_to_block: BlockIndex,
    pub retransmit: Retransmit,
    pub stats: Statistics,
    pub ring_buffer: Option<Arc<ring::RingBuffer>>,
    pub received: Vec<u8>,
    pub blocks_left: BlockIndex,
    pub restart_pending: u8,
    pub restart_lastidx: BlockIndex,
    pub restart_wireclearidx: BlockIndex,
    pub on_wire_estimate: BlockIndex,
}

pub struct Session {
    pub transfer: Transfer,
    pub server: Option<TcpStream>,
}

impl Session {
    pub fn read<T: bincode::Decode>(&mut self) -> anyhow::Result<T> {
        let Some(server) = &mut self.server else {
            bail!("Connection not open")
        };
        Ok(bincode::decode_from_std_read(
            server,
            crate::common::BINCODE_CONFIG,
        )?)
    }

    pub fn write<T: bincode::Encode>(&mut self, value: T) -> anyhow::Result<usize> {
        let Some(server) = &mut self.server else {
            bail!("Connection not open")
        };
        Ok(bincode::encode_into_std_write(
            value,
            server,
            crate::common::BINCODE_CONFIG,
        )?)
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        let Some(server) = &mut self.server else {
            bail!("Connection not open")
        };
        server.flush()?;
        Ok(())
    }
}
