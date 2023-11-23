use std::path::PathBuf;

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

#[derive(Clone, clap::Args)]
pub struct Parameter {
    #[arg(skip)]
    pub epoch: extc::time_t,

    /// turns on verbose output mode
    #[arg(long = "verbose", short = 'v')]
    pub verbose_yn: bool,

    /// turns on transcript mode for statistics recording
    #[arg(long = "transcript", short = 't')]
    pub transcript_yn: bool,

    /// operates using IPv6 instead of (not in addition to!) IPv4
    #[arg(long = "v6", short = '6')]
    pub ipv6_yn: bool,

    /// specifies which TCP port on which to listen to incoming connections
    #[arg(long = "port", short = 'p', default_value_t = config::DEFAULT_TCP_PORT)]
    pub tcp_port: u16,

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
    pub finishhook: Option<String>,

    /// run command on 'get *' to produce a custom file list for client downloads
    #[arg(long = "allhook", short = 'a')]
    pub allhook: Option<String>,

    #[arg(skip = config::DEFAULT_BLOCK_SIZE)]
    pub block_size: u32,

    #[arg(skip)]
    pub file_size: u64,

    #[arg(skip)]
    pub block_count: u32,

    #[arg(skip)]
    pub target_rate: u32,

    #[arg(skip)]
    pub error_rate: u32,

    #[arg(skip)]
    pub ipd_time: u32,

    #[arg(skip)]
    pub slower_num: u16,

    #[arg(skip)]
    pub slower_den: u16,

    #[arg(skip)]
    pub faster_num: u16,

    #[arg(skip)]
    pub faster_den: u16,

    #[arg(skip)]
    pub fileout: u16,

    #[arg(skip)]
    pub slotnumber: libc::c_int,

    #[arg(skip)]
    pub totalslots: libc::c_int,

    #[arg(skip)]
    pub samplerate: libc::c_int,

    /// list of files to share for downloaded via a client 'GET *'
    #[arg()]
    pub file_names: Vec<PathBuf>,

    /// Files with associated size
    #[arg(skip)]
    pub files: Vec<(PathBuf, u64)>,

    #[arg(skip)]
    pub file_name_size: usize,

    #[arg(skip)]
    pub wait_u_sec: libc::c_long,
}

pub struct Transfer {
    pub filename: Option<String>,
    pub file: Option<std::fs::File>,
    pub transcript: Option<std::fs::File>,
    pub udp_fd: libc::c_int,
    pub udp_address: *mut extc::sockaddr,
    pub udp_length: extc::socklen_t,
    pub ipd_current: libc::c_double,
    pub block: u32,
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
            block: 0,
        }
    }
}

pub struct Session {
    pub transfer: Transfer,
    pub client_fd: libc::c_int,
    pub session_id: libc::c_int,
}
