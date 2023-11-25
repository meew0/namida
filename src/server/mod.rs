use std::{io::Write, net::TcpStream, path::PathBuf, time::Duration};

use crate::{
    extc,
    types::{BlockIndex, BlockSize, ErrorRate, FileMetadata, FileSize, Fraction, TargetRate},
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

    /// specifies the desired size for UDP socket send buffer (in bytes)
    #[arg(long = "buffer", short = 'b', default_value_t = config::DEFAULT_UDP_BUFFER)]
    pub udp_buffer: u32,

    /// specifies the timeout in seconds for disconnect after client heartbeat lost
    #[arg(long = "hbtimeout", default_value_t = config::DEFAULT_HEARTBEAT_TIMEOUT)]
    pub hb_timeout: u16,

    /// specifies the shared secret for the client and server
    #[arg(long = "secret", short = 's', default_value_t = config::DEFAULT_SECRET.to_owned())]
    pub secret: String,

    /// specifies an alternate client IP or host where to send data
    #[arg(long = "client", short = 'c')]
    pub client: Option<String>,

    /// run command on transfer completion, file name is appended automatically
    #[arg(long = "finishhook", short = 'f')]
    pub finishhook: Option<PathBuf>,

    /// list of files to share for downloaded via a client 'GET *'
    #[arg()]
    pub file_names: Vec<PathBuf>,

    /// Files with associated size
    #[arg(skip)]
    pub files: Vec<FileMetadata>,
}

pub struct Properties {
    pub epoch: Duration,
    pub block_size: BlockSize,
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
    pub wait_u_sec: i64,
}

impl Default for Properties {
    fn default() -> Self {
        Self {
            epoch: Duration::default(),
            block_size: config::DEFAULT_BLOCK_SIZE,
            file_size: Default::default(),
            block_count: Default::default(),
            target_rate: TargetRate(0),
            error_rate: ErrorRate(0),
            ipd_time: Default::default(),
            slower: Fraction {
                numerator: 0,
                denominator: 0,
            },
            faster: Fraction {
                numerator: 0,
                denominator: 0,
            },
            fileout: Default::default(),
            slotnumber: Default::default(),
            totalslots: Default::default(),
            samplerate: Default::default(),
            wait_u_sec: 0,
        }
    }
}

pub struct Transfer {
    pub filename: Option<PathBuf>,
    pub file: Option<std::fs::File>,
    pub transcript: Option<std::fs::File>,
    pub udp_fd: libc::c_int,
    pub udp_address: *mut extc::sockaddr,
    pub udp_length: extc::socklen_t,
    pub ipd_current: libc::c_double,
    pub block: BlockIndex,
}

impl Default for Transfer {
    fn default() -> Self {
        Self {
            filename: None,
            file: None,
            transcript: None,
            udp_fd: 0,
            udp_address: std::ptr::null_mut(),
            udp_length: 0,
            ipd_current: 0.0,
            block: BlockIndex(0),
        }
    }
}

pub struct Session {
    pub transfer: Transfer,
    pub properties: Properties,
    pub client: TcpStream,
    pub session_id: usize,
}

impl Session {
    pub fn read<T: bincode::Decode>(&mut self) -> anyhow::Result<T> {
        Ok(bincode::decode_from_std_read(
            &mut self.client,
            crate::common::BINCODE_CONFIG,
        )?)
    }

    pub fn write<T: bincode::Encode>(&mut self, value: T) -> anyhow::Result<usize> {
        Ok(bincode::encode_into_std_write(
            value,
            &mut self.client,
            crate::common::BINCODE_CONFIG,
        )?)
    }

    pub fn flush(&mut self) -> anyhow::Result<()> {
        self.client.flush()?;
        Ok(())
    }
}
