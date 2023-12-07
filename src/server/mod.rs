use std::{
    net::{SocketAddr, UdpSocket},
    path::PathBuf,
    time::Duration,
};

use crate::{
    common::SocketWrapper,
    types::{BlockIndex, ErrorRate, FileSize, Fraction, TargetRate},
};

pub mod config;
pub mod io;
pub mod main;
pub mod network;
pub mod protocol;
pub mod transcript;

#[derive(Clone, clap::Args)]
pub struct Parameter {
    /// turns on verbose output mode
    #[arg(long = "verbose", short = 'v')]
    pub verbose_yn: bool,

    /// turns on transcript mode for statistics recording
    #[arg(long = "transcript", short = 't')]
    pub transcript_yn: bool,

    /// Address at which to listen for incoming TCP connections. Determines port, bind host, and
    /// IPv6 usage.
    #[arg(long = "bind", short = 'B', default_value_t = config::DEFAULT_BIND.to_owned())]
    pub bind: String,

    /// If this flag is present, the server will only accept unencrypted connections. By default, it
    /// will only accept encrypted connections.
    #[arg(long = "unencrypted", action = clap::ArgAction::SetFalse)]
    pub encrypted: bool,

    /// Defines the indexing mode — whether input files and directories are never indexed (which
    /// means file listing will be unsupported), only indexed at startup, or reindexed whenever the
    /// client requests a file list.
    #[arg(long = "index", default_value_t, value_enum)]
    pub index: IndexMode,

    /// specifies the desired size for UDP socket send buffer (in bytes)
    #[arg(long = "buffer", short = 'b', default_value_t = config::DEFAULT_UDP_BUFFER)]
    pub udp_buffer: u32,

    /// specifies the timeout in seconds for disconnect after client heartbeat lost
    #[arg(long = "hbtimeout", default_value_t = config::DEFAULT_HEARTBEAT_TIMEOUT)]
    pub hb_timeout: u16,

    /// specifies the file from which the pre-shared key will be read. If not specified, a
    /// hardcoded key will be used (not recommended)
    #[arg(long = "secret", short = 's')]
    pub secret_file: Option<PathBuf>,

    /// specifies an alternate client IP or host where to send data
    #[arg(long = "client", short = 'c')]
    pub client: Option<String>,

    /// run command on transfer completion, file name is appended automatically
    #[arg(long = "finishhook", short = 'f')]
    pub finishhook: Option<PathBuf>,

    /// list of files to share for downloaded via a client 'GET *'
    #[arg()]
    pub file_names: Vec<PathBuf>,

    #[arg(skip = *crate::common::DEFAULT_SECRET)]
    pub secret: [u8; 32],
}

#[derive(Clone, Default, clap::ValueEnum)]
pub enum IndexMode {
    /// Indexing will never be performed. Directory listing and getting all files using `*` will be
    /// unsupported.
    Never,

    /// Indexing will only be performed at startup. If files are added in the meantime, this will
    /// not be reflected in directory listings, but the file can still be downloaded if the client
    /// knows the path regardless.
    #[default]
    Startup,

    /// Every time the client requests a list of files, the input folder(s) will be reindexed.
    Always,
}

pub struct Properties {
    pub epoch: Duration,
    pub file_size: FileSize,
    pub block_count: BlockIndex,
    pub target_rate: TargetRate,
    pub error_rate: ErrorRate,
    pub ipd_time: u32,
    pub slower: Fraction,
    pub faster: Fraction,
    pub fileout: u16,
    pub slotnumber: i32,
    pub totalslots: i32,
    pub samplerate: i32,
    pub wait_µs: i64,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            epoch: Duration::default(),
            file_size: FileSize::default(),
            block_count: BlockIndex::default(),
            target_rate: TargetRate(0),
            error_rate: ErrorRate(0),
            ipd_time: 0,
            slower: Fraction {
                numerator: 0,
                denominator: 0,
            },
            faster: Fraction {
                numerator: 0,
                denominator: 0,
            },
            fileout: 0,
            slotnumber: 0,
            totalslots: 0,
            samplerate: 0,
            wait_µs: 0,
        }
    }
}

pub struct Transfer {
    pub filename: Option<PathBuf>,
    pub file: Option<std::fs::File>,
    pub transcript: Option<std::fs::File>,
    pub udp_socket: Option<UdpSocket>,
    pub udp_address: Option<SocketAddr>,
    pub ipd_current: libc::c_double,
    pub block: BlockIndex,
}

impl Default for Transfer {
    fn default() -> Self {
        Self {
            filename: None,
            file: None,
            transcript: None,
            udp_socket: None,
            udp_address: None,
            ipd_current: 0.0,
            block: BlockIndex(0),
        }
    }
}

pub struct Session {
    pub transfer: Transfer,
    pub properties: Properties,
    pub client: SocketWrapper,
    pub session_id: usize,
}
