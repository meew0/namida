use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use anyhow::bail;

use crate::{
    client::Statistics,
    datagram::{self, BlockType},
    message,
    types::{BlockIndex, ErrorRate, FileMetadata, FileSize, Fraction, TargetRate, UdpErrors},
};

use super::{ring, OutputMode, Transfer};

#[derive(Clone, clap::Args)]
#[allow(clippy::struct_excessive_bools)]
pub struct Parameter {
    /// The server to connect to. May be specified as IP address or hostname. A remote TCP port may
    /// also be specified using the `host:port` notation. If no port is specified, the default port
    /// will be used (51038).
    #[arg(long = "server", short = 's')]
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

    #[arg(long = "buffer", default_value_t = super::config::DEFAULT_UDP_BUFFER)]
    pub udp_buffer: u32,

    #[arg(long = "quiet", action = clap::ArgAction::SetFalse)]
    pub verbose_yn: bool,

    #[arg(long = "transcript")]
    pub transcript_yn: bool,

    #[arg(long = "ipv6")]
    pub ipv6_yn: bool,

    #[arg(long = "output", value_enum, default_value_t = OutputMode::Line)]
    pub output_mode: OutputMode,

    #[arg(long = "rate", value_parser = clap::builder::ValueParser::new(parse_rate), default_value_t = super::config::DEFAULT_TARGET_RATE)]
    pub target_rate: TargetRate,

    #[arg(long = "rateadjust")]
    pub rate_adjust: bool,

    #[arg(long = "error", default_value_t = super::config::DEFAULT_ERROR_RATE)]
    pub error_rate: ErrorRate,

    #[arg(long = "slower", value_parser = clap::builder::ValueParser::new(parse_fraction), default_value_t = super::config::DEFAULT_SLOWER)]
    pub slower: Fraction,

    #[arg(long = "faster", value_parser = clap::builder::ValueParser::new(parse_fraction), default_value_t = super::config::DEFAULT_FASTER)]
    pub faster: Fraction,

    #[arg(long = "history", default_value_t = super::config::DEFAULT_HISTORY)]
    pub history: u16,

    #[arg(long = "lossy", action = clap::ArgAction::SetFalse)]
    pub lossless: bool,

    #[arg(long = "losswindow", default_value_t = super::config::DEFAULT_LOSSWINDOW_MS)]
    pub losswindow_ms: u32,

    #[arg(long = "blockdump")]
    pub blockdump: bool,

    /// Specifies the path to a file from which the pre-shared key will be loaded. Only the first 32
    /// bytes of the file will be used as the PSK. If not specified, a hard-coded key will be used;
    /// this is not recommended.
    #[arg(long = "secret")]
    pub secret_file: Option<PathBuf>,

    /// The local filename under which the remote file should be saved. This will only work if
    /// requesting exactly one file, otherwise the command will fail!
    #[arg(long = "local")]
    pub local_filename: Option<PathBuf>,

    #[arg(skip = *crate::common::DEFAULT_SECRET)]
    pub secret: [u8; 32],

    /// The files to try to read from the server.
    #[arg()]
    pub files: Vec<PathBuf>,

    /// Download all files indexed on the server.
    #[arg(long = "all")]
    pub all: bool,
}

/// Parse a string in the form `123M` into an integer like `123000000`.
///
/// # Errors
/// Returns an error on parse failure.
///
/// # Panics
/// Panics on arithmetic overflow.
pub fn parse_rate(rate: &str) -> anyhow::Result<TargetRate> {
    let (main_part, last_char) = rate.split_at(
        rate.len()
            .checked_sub(1)
            .expect("tried to `parse_rate` an empty string"),
    );
    let parsed: u64 = main_part.parse()?;

    let value = match last_char {
        "k" | "K" => parsed.checked_mul(1000).expect("rate parsing overflow"),
        "m" | "M" => parsed
            .checked_mul(1_000_000)
            .expect("rate parsing overflow"),
        "g" | "G" => parsed
            .checked_mul(1_000_000_000)
            .expect("rate parsing overflow"),
        "t" | "T" => parsed
            .checked_mul(1_000_000_000_000)
            .expect("rate parsing overflow"),
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => rate.parse()?,
        _ => bail!("Invalid unit specifier"),
    };

    Ok(TargetRate(value))
}

/// Parse a string in the form `aaa/bbb` into a `Fraction` object.
///
/// # Errors
/// Returns an error on parse failure.
pub fn parse_fraction(fraction: &str) -> anyhow::Result<Fraction> {
    if let Some((num_str, den_str)) = fraction.split_once('/') {
        let numerator: u16 = num_str.parse()?;
        let denominator: u16 = den_str.parse()?;
        Ok(Fraction {
            numerator,
            denominator,
        })
    } else {
        bail!("No slash found")
    }
}

#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn run(mut parameter: Parameter) -> anyhow::Result<()> {
    crate::common::load_secret(&parameter.secret_file, &mut parameter.secret);
    super::print_intro(parameter.encrypted);

    // Connect to the server
    let mut session =
        super::protocol::connect(&parameter.server, parameter.encrypted, &parameter.secret)?;

    // These variables are only used when requesting multiple files.
    let mut file_names: Vec<PathBuf> = vec![];

    if parameter.all {
        println!("Requesting all indexed files");

        session
            .server
            .write(message::ClientToServer::FileListRequest)?;
        let message::ServerToClient::FileCount(count) = session.server.read()? else {
            bail!("Expected file count");
        };
        if count == 0 {
            bail!("Server advertised no files to get");
        }

        println!();
        println!("Server is sharing {count} files");
        println!("Multi-GET of {count} files:");

        for _i in 0..count {
            let message::ServerToClient::FileListEntry(file_metadata) = session.server.read()?
            else {
                bail!("Expected file");
            };
            let FileMetadata { path, size } = file_metadata;

            println!(" {} ({} bytes)", path.display(), size.0);
            file_names.push(path);
        }

        session.server.flush()?;
    } else {
        if parameter.files.is_empty() {
            bail!("No files are specified. Either specify a list of files to be downloaded, or use the `--all` option to download all indexed files.");
        }

        file_names.extend_from_slice(&parameter.files);
    }

    if file_names.is_empty() {
        bail!("No files are to be downloaded.");
    }

    if file_names.len() > 1 && parameter.local_filename.is_some() {
        bail!("A local filename can only be specified if only one file is to be downloaded.");
    }

    let mut stats_iteration = 0;
    let mut successful = true;

    'outer: for remote_filename in file_names {
        // Get a suitable local filename for the remote one
        let local_filename = create_local_filename(&remote_filename, &parameter.local_filename);

        // negotiate the file request with the server
        let remote_udp_port = super::protocol::open_transfer(
            &mut session,
            &parameter,
            remote_filename,
            local_filename,
        )?;

        // create the UDP data socket
        super::protocol::open_port(&mut session, &parameter, remote_udp_port)?;

        // allocate the retransmission table and received bitfield
        session.transfer.retransmit.previous_table = vec![];
        session.transfer.received = vec![
            0;
            (session.transfer.block_count.0 / 8)
                .checked_add(2)
                .expect("`received` bitfield size overflow")
                as usize
        ];

        // allocate the ring buffer
        // We want to avoid unwrapping the buffer every time we use it. But, on the other hand, we
        // cannot borrow it for the entire length of the function, because it will prevent us from
        // borrowing the session as a whole. So we make a local clone of the `Arc` which will be
        // dropped at the end of the function.
        let ring_buffer_ref = session
            .transfer
            .ring_buffer
            .insert(Arc::new(super::ring::Buffer::create()));
        let ring_buffer = Arc::clone(ring_buffer_ref);

        // allocate the faster local buffer
        let local_datagram_buffer_size = (crate::common::BLOCK_SIZE as usize)
            .checked_add(6)
            .expect("datagram buffer size overflow");
        let mut local_datagram_buffer =
            ring::allocate_zeroed_boxed_slice(local_datagram_buffer_size);

        // allocate the buffer for the ciphertext, if necessary
        let mut encrypted_buffer = if parameter.encrypted {
            let size = (crate::common::BLOCK_SIZE as usize)
                .checked_add(30) // 8 for nonce + 16 for noise auth data + 6 for block header
                .expect("datagram buffer size overflow");
            vec![0_u8; size]
        } else {
            vec![]
        };

        // This other clone of the ring buffer will be moved into the disk thread.
        let cloned_ring_buffer = Arc::clone(&ring_buffer);

        // start up the disk I/O thread
        let block_count = session.transfer.block_count;
        let file_size = session.transfer.file_size;
        let file = session
            .transfer
            .file
            .take()
            .expect("file should have been opened");
        let disk_thread_handle = std::thread::spawn(move || {
            disk_thread(cloned_ring_buffer, block_count, file_size, file)
        });

        // we start by expecting block #1
        session.transfer.next_block = BlockIndex(1);
        session.transfer.gapless_to_block = BlockIndex(0);

        // Start timing
        session.transfer.stats = Statistics::default();
        session.transfer.stats.udp_errors = UdpErrors::new();
        session.transfer.stats.start_time = Some(Instant::now());
        session.transfer.stats.this_time = Some(Instant::now());
        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::data_start(&mut session));
        }

        let mut dumpcount = 0_u32;

        // until we break out of the transfer
        loop {
            // try to receive a datagram
            let receive_buffer = if parameter.encrypted {
                encrypted_buffer.as_mut_slice()
            } else {
                &mut local_datagram_buffer
            };

            let udp_result = session
                .transfer
                .udp_socket
                .as_ref()
                .expect("UDP socket should be present")
                .recv_from(receive_buffer);

            match udp_result {
                Ok((len, _)) => {
                    if len != receive_buffer.len() {
                        println!(
                            "Ignoring datagram with incorrect length: {len} != {}",
                            receive_buffer.len()
                        );
                    }
                }
                Err(err) => {
                    println!("WARNING: UDP data transmission error: {err}");
                    println!("Apparently frozen transfer, trying to do retransmit request");
                    if let Err(err) = super::protocol::repeat_retransmit(&mut session) {
                        println!("WARNING: Repeat of retransmission requests failed: {err:?}");
                        successful = false;
                        break 'outer;
                    }
                }
            }

            let local_datagram_view: datagram::View = if parameter.encrypted {
                const U64_SIZE: usize = std::mem::size_of::<u64>();
                let (nonce, _) = bincode::decode_from_slice(
                    &encrypted_buffer[..U64_SIZE],
                    crate::common::BINCODE_CONFIG,
                )?;
                let payload = &encrypted_buffer[U64_SIZE..];
                session
                    .server
                    .decrypt_borrow_decode(nonce, payload, &mut local_datagram_buffer)?
            } else {
                let (datagram_view, _) = bincode::borrow_decode_from_slice(
                    &local_datagram_buffer,
                    crate::common::BINCODE_CONFIG,
                )?;
                datagram_view
            };

            let this_block = local_datagram_view.header.block_index; // 1-based
            let this_type = local_datagram_view.header.block_type;

            // keep statistics on received blocks
            session.transfer.stats.total_blocks =
                session.transfer.stats.total_blocks.safe_add(BlockIndex(1));
            if matches!(this_type, BlockType::Retransmission) {
                session.transfer.stats.this_flow_retransmitteds = session
                    .transfer
                    .stats
                    .this_flow_retransmitteds
                    .safe_add(BlockIndex(1));
                session.transfer.stats.total_recvd_retransmits = session
                    .transfer
                    .stats
                    .total_recvd_retransmits
                    .safe_add(BlockIndex(1));
            } else {
                session.transfer.stats.this_flow_originals = session
                    .transfer
                    .stats
                    .this_flow_originals
                    .safe_add(BlockIndex(1));
            }

            // main transfer control logic
            if !ring_buffer.is_full() // don't let disk-I/O freeze stop feedback of stats to server
                && (!session.got_block(this_block)
                    || matches!(this_type, BlockType::Final)
                    || session.transfer.restart_pending)
            {
                // insert new blocks into disk write ringbuffer
                if !session.got_block(this_block) {
                    // reserve ring space, copy the data in, confirm the reservation
                    ring_buffer.reserve(local_datagram_view);
                    ring_buffer.confirm();

                    // mark the block as received
                    let fresh1 = &mut session.transfer.received[(this_block.0 / 8) as usize];
                    *fresh1 |= 1 << (this_block.0 % 8);

                    if session.transfer.blocks_left.is_zero() {
                        println!("Oops! Negative-going blocks_left count at block: type={:?} this={} final={} left={}",
                                this_type,
                                this_block.0,
                                session.transfer.block_count.0,
                                session.transfer.blocks_left.0,
                            );
                    } else {
                        session.transfer.blocks_left =
                            session.transfer.blocks_left.safe_sub(BlockIndex(1));
                    }
                }

                // transmit restart: avoid re-triggering on blocks still down the wire before
                // server reacts
                if !session.transfer.restart_pending
                    || matches!(this_type, BlockType::Final)
                    || this_block <= session.transfer.restart_lastidx
                    || this_block > session.transfer.restart_wireclearidx
                {
                    // queue any retransmits we need
                    if this_block > session.transfer.next_block {
                        if parameter.lossless {
                            // lossless transfer mode, request all missing data to be resent
                            let mut block = session.transfer.next_block;
                            while block < this_block {
                                super::protocol::request_retransmit(&mut session, block);
                                block = block.safe_add(BlockIndex(1));
                            }
                        } else {
                            // lossy transfer mode
                            if parameter.losswindow_ms == 0 {
                                // lossy transfer, no retransmits
                                session.transfer.gapless_to_block = this_block;
                            } else {
                                // semi-lossy transfer, purge data past specified approximate time
                                // window
                                let mut path_capability: f64 = 0.8_f64
                                    * (session.transfer.stats.this_transmit_rate
                                        + session.transfer.stats.this_retransmit_rate);
                                path_capability *= 0.001_f64 * f64::from(parameter.losswindow_ms);

                                let first = 1_000_000.0 * path_capability
                                    / (8.0 * f64::from(crate::common::BLOCK_SIZE));
                                let second = f64::from(
                                    (this_block.safe_sub(session.transfer.gapless_to_block)).0,
                                );
                                let block_diff = f64::min(first, second);

                                // TODO: potentially rewrite this part using more precise non-FP
                                // arithmetic. It will not match what tsunami does but might be
                                // more desirable
                                #[allow(clippy::cast_possible_truncation)]
                                #[allow(clippy::cast_sign_loss)]
                                let earliest_block =
                                    BlockIndex((f64::from(this_block.0) - block_diff) as u32);
                                let mut block = earliest_block;
                                while block < this_block {
                                    super::protocol::request_retransmit(&mut session, block);
                                    block = block.safe_add(BlockIndex(1));
                                }

                                // hop over the missing section
                                session.transfer.next_block = earliest_block;
                                session.transfer.gapless_to_block = earliest_block;
                            }
                        }
                    }

                    // advance the index of the gapless section going from start block to highest
                    // block
                    while session
                        .got_block(session.transfer.gapless_to_block.safe_add(BlockIndex(1)))
                        && session.transfer.gapless_to_block < session.transfer.block_count
                    {
                        session.transfer.gapless_to_block =
                            session.transfer.gapless_to_block.safe_add(BlockIndex(1));
                    }

                    // if this is an orignal, we expect to receive the successor to this block next
                    // transmit restart note: these resent blocks are labeled original as well
                    if matches!(this_type, BlockType::Original) {
                        session.transfer.next_block = this_block.safe_add(BlockIndex(1));
                    }

                    // transmit restart: already got out of the missing blocks range?
                    if session.transfer.restart_pending
                        && session.transfer.next_block >= session.transfer.restart_lastidx
                    {
                        session.transfer.restart_pending = false;
                    }

                    // are we at the end of the transmission?
                    if matches!(this_type, BlockType::Final) {
                        // got all blocks by now
                        if session.transfer.blocks_left == BlockIndex(0) {
                            break;
                        }
                        if !parameter.lossless
                            && session.transfer.retransmit.previous_table.is_empty()
                            && !session.transfer.restart_pending
                        {
                            break;
                        }

                        // add possible still missing blocks to retransmit list
                        let mut block = session.transfer.gapless_to_block.safe_add(BlockIndex(1));
                        while block < session.transfer.block_count {
                            super::protocol::request_retransmit(&mut session, block);
                            block = block.safe_add(BlockIndex(1));
                        }

                        // send the retransmit request list again
                        super::protocol::repeat_retransmit(&mut session)?;
                    }
                }
            }

            // repeat our server feedback and requests if it's time
            if session.transfer.stats.total_blocks.0 % 50 != 0 {
                continue;
            }

            // if it's been at least 350ms
            if crate::common::get_µs_since(
                session
                    .transfer
                    .stats
                    .this_time
                    .expect("this_time should be set"),
            ) <= 350_000
            {
                continue;
            }

            // repeat our retransmission requests
            super::protocol::repeat_retransmit(&mut session)?;

            // send and show our current statistics
            super::protocol::update_stats(&mut session, &parameter, &mut stats_iteration)?;

            // progress blockmap (DEBUG)
            if parameter.blockdump {
                let postfix = format!(".bmap{dumpcount}");
                if let Err(err) = dump_blockmap(&postfix, &session.transfer) {
                    eprintln!("Failed to write blockmap dump: {err:?}");
                }
                dumpcount = dumpcount.wrapping_add(1);
            }
        }

        println!("Transfer complete. Flushing to disk and signaling server to stop...");
        session.transfer.udp_socket.take();

        // tell the server to quit transmitting
        if let Err(err) = super::protocol::request_stop(&mut session) {
            println!("WARNING: Could not request end of transfer: {err:?}");
            successful = false;
            break;
        }

        // add a stop block to the ring buffer
        ring_buffer.reserve_zero();
        ring_buffer.confirm();

        // wait for the disk thread to die
        if let Err(err) = disk_thread_handle.join() {
            println!("Error in disk thread: {err:?}");
        }

        // get finishing time
        session.transfer.stats.stop_time = Some(Instant::now());
        let delta = crate::common::get_µs_since(
            session
                .transfer
                .stats
                .start_time
                .expect("start_time should have been set"),
        );

        // count the truly lost blocks from the `received` bitmap table
        session.transfer.stats.total_lost = BlockIndex(0);
        let mut block = BlockIndex(1);
        while block <= session.transfer.block_count {
            if !session.got_block(block) {
                session.transfer.stats.total_lost =
                    session.transfer.stats.total_lost.safe_add(BlockIndex(1));
            }
            block = block.safe_add(BlockIndex(1));
        }

        // calculate and display the final results
        let bit_thru = 8.0_f64
            * f64::from(session.transfer.stats.total_blocks.0)
            * f64::from(crate::common::BLOCK_SIZE);
        let bit_good = (8.0_f64 * f64::from(session.transfer.stats.total_recvd_retransmits.0))
            .mul_add(-f64::from(crate::common::BLOCK_SIZE), bit_thru);
        #[allow(clippy::cast_precision_loss)]
        let bit_file = 8.0_f64 * session.transfer.file_size.0 as f64;

        let megabit_thru = bit_thru / 1_000_000.0;
        let megabit_good = bit_good / 1_000_000.0;
        let megabit_file = bit_file / 1_000_000.0;

        #[allow(clippy::cast_precision_loss)]
        let time_secs = delta as f64 / 1e6_f64;

        println!("PC performance figure : {} packets dropped (if high this indicates receiving PC overload)",
                session.transfer.stats.udp_errors,
            );
        println!("Transfer duration     : {time_secs:0>.2} seconds");
        println!("Total packet data     : {megabit_thru:0>.2} Mbit");
        println!("Goodput data          : {megabit_good:0>.2} Mbit");
        println!("File data             : {megabit_file:0>.2} Mbit");
        println!(
            "Throughput            : {:0>.2} Mbps",
            megabit_thru / time_secs
        );
        println!(
            "Goodput w/ restarts   : {:0>.2} Mbps",
            megabit_good / time_secs
        );
        println!(
            "Final file rate       : {:0>.2} Mbps",
            megabit_file / time_secs
        );
        print!("Transfer mode         : ");
        if parameter.lossless {
            if session.transfer.stats.total_lost == BlockIndex(0) {
                println!("lossless");
            } else {
                println!(
                    "lossless mode - but lost count={} > 0, please file a bug report!!",
                    session.transfer.stats.total_lost.0,
                );
            }
        } else {
            if parameter.losswindow_ms == 0 {
                println!("lossy");
            } else {
                println!("semi-lossy, time window {} ms", parameter.losswindow_ms);
            }
            println!(
                    "Data blocks lost      : {} ({:.2}% of data) per user-specified time window constraint",
                    session.transfer.stats.total_lost.0,
                    100.0_f64 * f64::from(session.transfer.stats.total_lost.0)
                        / f64::from(session.transfer.block_count.0),
                );
        }
        println!();

        // update the transcript
        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::data_stop(&mut session));
            crate::common::transcript_warn_error(super::transcript::close(&mut session, delta));
        }

        // dump the received packet bitfield to a file, with added filename prefix `.blockmap`
        if parameter.blockdump {
            if let Err(err) = dump_blockmap(".blockmap", &session.transfer) {
                eprintln!("Failed to write blockmap: {err}");
            }
        }

        session.transfer.retransmit.previous_table = vec![];

        // update the target rate
        if parameter.rate_adjust {
            #[allow(clippy::cast_sign_loss)]
            #[allow(clippy::cast_possible_truncation)]
            let new_target_rate =
                TargetRate((1.15_f64 * 1e6_f64 * (megabit_file / time_secs)) as u64);
            parameter.target_rate = new_target_rate;

            #[allow(clippy::cast_precision_loss)]
            let new_target_rate_megabits = parameter.target_rate.0 as f64 / 1e6_f64;
            println!(
                "Adjusting target rate to {new_target_rate_megabits:.3} Mbps for next transfer.",
            );
        }

        // continue with the next file, if it exists
    }

    if successful {
        eprintln!("All transfers were successful!");
        session.server.write(message::ClientToServer::Close)?;
    } else {
        eprintln!("Transfer not successful.");
        eprintln!();

        session.transfer.udp_socket.take();
        session.transfer.retransmit.previous_table.clear();

        bail!("Transfer unsuccessful");
    }

    Ok(())
}

fn create_local_filename(remote_filename: &Path, local_filename: &Option<PathBuf>) -> PathBuf {
    if let Some(local_filename) = local_filename.as_ref() {
        // Local filename was specified
        PathBuf::from(local_filename)
    } else if let Some(file_name_part) = remote_filename.file_name() {
        // Remote filename contains slash, use only the last part as the local filename
        PathBuf::from(file_name_part)
    } else {
        // Remote filename does not contain slash, use it as the local filename in its
        // entirety
        remote_filename.to_path_buf()
    }
}

/// This is the thread that takes care of saved received blocks to disk. It runs until the network
/// thread sends it a datagram with a block number of 0.
#[allow(clippy::needless_pass_by_value)]
fn disk_thread(
    ring_buffer: Arc<super::ring::Buffer>,
    block_count: BlockIndex,
    file_size: FileSize,
    mut file: std::fs::File,
) -> anyhow::Result<()> {
    // while the world is turning
    loop {
        // get another block
        ring_buffer.peek(|datagram_view| {
            // quit if we got the mythical 0 block
            if datagram_view.header.block_index == BlockIndex(0) {
                bail!("!!!!");
            }

            // save it to disk
            super::io::accept_block(datagram_view, block_count, file_size, &mut file)?;
            Ok(())
        })?;

        // pop the block
        ring_buffer.pop();
    }
}

/// Writes the current bitmap of received block accounting into a file named like the transferred
/// file but with an extra postfix.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if there is no local filename set.
pub fn dump_blockmap(postfix: &str, xfer: &Transfer) -> anyhow::Result<()> {
    // Get filename and append postfix
    let mut fname = xfer
        .local_filename
        .as_ref()
        .expect("local_filename should be present")
        .clone();
    let mut file_name_part = fname
        .file_name()
        .expect("local_filename should have a file name part")
        .to_owned();
    file_name_part.push(postfix);
    fname.set_file_name(file_name_part);

    // write: [4 bytes block_count] [map byte 0] [map byte 1] ... [map N (partial final byte)]
    let mut fbits = std::fs::File::options()
        .write(true)
        .create(true)
        .open(fname)?;
    fbits.write_all(&xfer.block_count.0.to_le_bytes())?;
    let block_data = &xfer.received[0..((xfer.block_count.0 / 8).wrapping_add(1) as usize)];
    fbits.write_all(block_data)?;

    Ok(())
}
