pub mod config;
pub mod dir;
pub mod get;
pub mod io;
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
    types::{BlockIndex, FileSize, ReceivedMap, UdpErrors},
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
    pub received: ReceivedMap,
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

impl Session {
    /// Returns true if the block has already been received
    #[must_use]
    pub fn got_block(&self, blocknr: BlockIndex) -> bool {
        if blocknr > self.transfer.block_count {
            return true;
        }

        self.transfer.received.got_block(blocknr)
    }
}

pub fn print_intro(encrypted: bool) {
    // show version / build information
    eprintln!(
        "namida client for protocol revision {} (block size = {}, magic = 0x{:x})\nVersion: {} (revision {})\nCompiled: {}\n",
        crate::version::NAMIDA_PROTOCOL_REVISION,
        crate::common::BLOCK_SIZE,
        crate::version::magic(encrypted),
        crate::version::NAMIDA_VERSION,
        &crate::version::GIT_HASH[0..7],
        crate::version::COMPILE_DATE_TIME,
    );
}
