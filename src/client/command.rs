use std::{
    io::Write,
    path::{Path, PathBuf},
    sync::Arc,
    time::Instant,
};

use anyhow::bail;

use super::{ring, OutputMode, Parameter, Session, Statistics, Transfer};
use crate::{
    common::SocketWrapper,
    datagram::{self, BlockType},
    message::{ClientToServer, DirListStatus, ServerToClient},
    types::{
        BlockIndex, BlockSize, ErrorRate, FileMetadata, FileSize, Fraction, TargetRate, UdpErrors,
    },
};

/// “Closes the connection” by virtue of dropping the session object.
///
/// # Errors
/// Will never error, only returns a `Result` for compatibility with other `command` functions.
pub fn close(parameter: &Parameter, session: Option<Session>) -> anyhow::Result<()> {
    if parameter.verbose_yn {
        if session.is_some() {
            drop(session);
            println!("Connection closed.");
        } else {
            println!("No connection currently active.");
        }
        println!();
    }

    Ok(())
}

/// Opens a new control session to the server specified in the command, or in the given set of
/// default parameters. On success, we return the created session object.
///
/// Note that the default host and port stored in the parameter object are updated if they were
/// specified in the command itself.
///
/// # Errors
/// Returns an error on I/O failure.
pub fn connect(command: &[&str], parameter: &mut Parameter) -> anyhow::Result<Session> {
    // if we were given a new host, store that information
    if command.len() > 1 {
        parameter.server = command[1].to_owned();
    }

    // obtain our client socket, and create a new session object with it
    let mut session = Session {
        transfer: Transfer::default(),
        server: SocketWrapper {
            socket: super::network::create_tcp_socket(parameter)?,
        },
    };

    // negotiate the connection parameters
    if let Err(err) = super::protocol::negotiate(&mut session) {
        bail!("Protocol negotiation failed: {:?}", err);
    }

    // get the shared secret, and authenticate to the server
    let secret = match &parameter.passphrase {
        Some(passphrase) => passphrase,
        None => "kitten",
    };
    if let Err(err) = super::protocol::authenticate(&mut session, secret) {
        bail!("Authentication failure: {:?}", err);
    }

    // we succeeded
    if parameter.verbose_yn {
        println!("Connected.");
        println!();
    }

    Ok(session)
}

/// Tries to request a list of server shared files and their sizes.
///
/// # Errors
/// Returns an error on I/O failure.
pub fn dir(_command: &[&str], session: &mut Session) -> anyhow::Result<()> {
    // send request and parse the resulting response
    session.server.write(ClientToServer::DirList)?;
    let ServerToClient::DirListHeader { status, num_files } = session.server.read()? else {
        bail!("Expected dir list status");
    };
    if !matches!(status, DirListStatus::Ok) {
        bail!(
            "Server does not support listing of shared files: {:?}",
            status
        );
    }

    eprintln!("Remote file list:");
    for i in 0..num_files {
        let ServerToClient::DirListFile(file_metadata) = session.server.read()? else {
            bail!("Expected dir list file");
        };

        eprintln!(
            " {:2}) {:<64} {:10}",
            i,
            file_metadata.path.display(),
            file_metadata.size.0
        );
    }
    eprintln!();

    session.server.write(ClientToServer::DirListEnd)?;

    Ok(())
}

/// Tries to initiate a file transfer for the remote file given in the command. If the user did not
/// supply a local filename, we derive it from the remote filename.
///
/// # Errors
/// Returns an error on I/O failure, or when the command syntax is invalid.
///
/// # Panics
/// Panics if the ring buffer becomes unset for some reason.
pub fn get(
    command: &[&str],
    parameter: &mut Parameter,
    session: &mut Session,
) -> anyhow::Result<()> {
    // make sure that we have a remote file name
    if command.len() < 2 {
        bail!("Invalid command syntax (use 'help get' for details)");
    }
    assert!(command.len() >= 2); // to ensure bounds check elision

    // reinitialize the transfer data
    session.transfer = Transfer::default();

    // These variables are only used when requesting multiple files.
    let mut multimode = false;
    let mut file_names: Vec<PathBuf> = vec![];

    if command[1] == "*" {
        // if the client is asking for multiple files to be transferred
        multimode = true;
        println!("Requesting all available files");

        session.server.write(ClientToServer::MultiRequest)?;
        let ServerToClient::MultiFileCount(count) = session.server.read()? else {
            bail!("Expected file count");
        };

        session
            .server
            .write(ClientToServer::MultiAcknowledgeCount)?;

        if count == 0 {
            bail!("Server advertised no files to get");
        }

        println!();
        println!("Server is sharing {count} files");
        println!("Multi-GET of {count} files:");

        for _i in 0..count {
            let ServerToClient::MultiFile(file_metadata) = session.server.read()? else {
                bail!("Expected file");
            };
            let FileMetadata { path, size } = file_metadata;

            println!(" {} ({} bytes)", path.display(), size.0);
            file_names.push(path);
        }

        session.server.write(ClientToServer::MultiEnd)?;
        session.server.flush()?;
    } else {
        file_names.push(PathBuf::from(command[1].to_owned()));
    }

    let mut stats_iteration = 0;
    let mut successful = true;

    'outer: for remote_filename in file_names {
        // Get a suitable local filename for the remote one
        let local_filename = create_local_filename(multimode, &remote_filename, command);

        // negotiate the file request with the server
        super::protocol::open_transfer(session, parameter, remote_filename, local_filename)?;

        // create the UDP data socket
        super::protocol::open_port(session, parameter)?;

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
            .insert(Arc::new(super::ring::Buffer::create(parameter.block_size)));
        let ring_buffer = Arc::clone(ring_buffer_ref);

        // allocate the faster local buffer
        let mut local_datagram_buffer = ring::allocate_zeroed_boxed_slice(
            parameter
                .block_size
                .0
                .checked_add(6)
                .expect("datagram buffer size overflow") as usize,
        );

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
            crate::common::transcript_warn_error(super::transcript::data_start(session));
        }

        let mut dumpcount = 0_u32;

        // until we break out of the transfer
        loop {
            // try to receive a datagram
            let udp_result = session
                .transfer
                .udp_socket
                .as_ref()
                .expect("UDP socket should be present")
                .recv_from(local_datagram_buffer.as_mut());

            match udp_result {
                Ok(_) => {}
                Err(err) => {
                    println!("WARNING: UDP data transmission error: {err}");
                    println!("Apparently frozen transfer, trying to do retransmit request");
                    if let Err(err) = super::protocol::repeat_retransmit(session) {
                        println!("WARNING: Repeat of retransmission requests failed: {err:?}");
                        successful = false;
                        break 'outer;
                    }
                }
            }

            // retrieve the block number and block type
            let Some(local_datagram_view) = datagram::View::parse(&local_datagram_buffer) else {
                println!("WARNING: received datagram with invalid block type, ignoring");
                continue;
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
                && (!got_block(session, this_block)
                    || matches!(this_type, BlockType::Final)
                    || session.transfer.restart_pending)
            {
                // insert new blocks into disk write ringbuffer
                if !got_block(session, this_block) {
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
                                super::protocol::request_retransmit(session, block);
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
                                    / (8.0 * f64::from(parameter.block_size.0));
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
                                    super::protocol::request_retransmit(session, block);
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
                    while got_block(
                        session,
                        session.transfer.gapless_to_block.safe_add(BlockIndex(1)),
                    ) && session.transfer.gapless_to_block < session.transfer.block_count
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
                            super::protocol::request_retransmit(session, block);
                            block = block.safe_add(BlockIndex(1));
                        }

                        // send the retransmit request list again
                        super::protocol::repeat_retransmit(session)?;
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
            super::protocol::repeat_retransmit(session)?;

            // send and show our current statistics
            super::protocol::update_stats(session, parameter, &mut stats_iteration)?;

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
        if let Err(err) = super::protocol::request_stop(session) {
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
            if !got_block(session, block) {
                session.transfer.stats.total_lost =
                    session.transfer.stats.total_lost.safe_add(BlockIndex(1));
            }
            block = block.safe_add(BlockIndex(1));
        }

        // calculate and display the final results
        let bit_thru = 8.0_f64
            * f64::from(session.transfer.stats.total_blocks.0)
            * f64::from(parameter.block_size.0);
        let bit_good = (8.0_f64 * f64::from(session.transfer.stats.total_recvd_retransmits.0))
            .mul_add(-f64::from(parameter.block_size.0), bit_thru);
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
            crate::common::transcript_warn_error(super::transcript::data_stop(session));
            crate::common::transcript_warn_error(super::transcript::close(
                session, parameter, delta,
            ));
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

    if !successful {
        eprintln!("Transfer not successful.  (WARNING: You may need to reconnect.)");
        eprintln!();

        session.transfer.udp_socket.take();
        session.transfer.retransmit.previous_table.clear();

        bail!("Transfer unsuccessful");
    }

    Ok(())
}

fn create_local_filename(multimode: bool, remote_filename: &Path, command: &[&str]) -> PathBuf {
    let local_filename = if multimode {
        println!("GET *: now requesting file '{}'", remote_filename.display());
        remote_filename.to_path_buf()
    } else if command.len() >= 3 {
        // Local filename was specified
        PathBuf::from(command[2])
    } else if let Some(file_name_part) = remote_filename.file_name() {
        // Remote filename contains slash, use only the last part as the local filename
        PathBuf::from(file_name_part)
    } else {
        // Remote filename does not contain slash, use it as the local filename in its
        // entirety
        remote_filename.to_path_buf()
    };
    local_filename
}

/// Offers help on either the list of available commands or a particular command.
pub fn help(command: &[&str]) {
    if command.len() < 2 {
        println!("Help is available for the following commands:");
        println!();
        println!("    close    connect    get    dir    help    quit    set");
        println!();
        println!("Use 'help <command>' for help on an individual command.");
        println!();
        return;
    }

    let query = command[1];

    if query.eq_ignore_ascii_case("close") {
        println!("Usage: close");
        println!();
        println!("Closes the current connection to a remote namida server.");
    } else if query.eq_ignore_ascii_case("connect") {
        println!("Usage: connect");
        println!("       connect <remote-host>");
        println!("       connect <remote-host> <remote-port>");
        println!();
        println!("Opens a connection to a remote namida server.  If the host and port");
        println!("are not specified, default values are used.  (Use the 'set' command to");
        println!("modify these values.)");
        println!();
        println!("After connecting, you will be prompted to enter a shared secret for");
        println!("authentication.");
    } else if query.eq_ignore_ascii_case("get") {
        println!("Usage: get <remote-file>");
        println!("       get <remote-file> <local-file>");
        println!();
        println!("Attempts to retrieve the remote file with the given name.  If the");
        println!("local filename is not specified, the final part of the remote filename");
        println!("(after the last path separator) will be used.");
    } else if query.eq_ignore_ascii_case("dir") {
        println!("Usage: dir");
        println!();
        println!("Attempts to list the available remote files.");
    } else if query.eq_ignore_ascii_case("help") {
        println!("Come on.  You know what that command does.");
    } else if query.eq_ignore_ascii_case("quit") {
        println!("Usage: quit");
        println!();
        println!("Closes any open connection to a remote namida server and exits the");
        println!("namida client.");
    } else if query.eq_ignore_ascii_case("set") {
        println!("Usage: set");
        println!("       set <field>");
        println!("       set <field> <value>");
        println!();
        println!("Sets one of the defaults to the given value.  If the value is omitted,");
        println!("the current value of the field is returned.  If the field is also");
        println!("omitted, the current values of all defaults are returned.");
    } else {
        println!("'{query}' is not a recognized command.");
        println!("Use 'help' for a list of commands.");
    }

    println!();
}

/// No cleanup is necessary in Rust, so this method only prints a message and exits the process.
pub fn quit() {
    println!("Thank you for using namida.");
    println!();
    println!("The repository can be found at: https://github.com/meew0/namida/");
    println!();

    std::process::exit(0);
}

/// Sets a particular parameter to the given value, or simply reports on the current value of one
/// or more fields.
///
/// # Errors
/// Returns an error if a value fails to be parsed.
///
/// # Panics
/// Panics on arithmetic overflow.
#[allow(clippy::missing_asserts_for_indexing)]
pub fn set(command: &[&str], parameter: &mut Parameter) -> anyhow::Result<()> {
    let do_all = command.len() == 1;

    // handle actual set operations first
    if command.len() == 3 {
        let property = command[1];
        let value_str = command[2];

        if property.eq_ignore_ascii_case("server") {
            parameter.server = value_str.to_owned();
        } else if property.eq_ignore_ascii_case("udpport") {
            parameter.client_port = value_str.parse()?;
        } else if property.eq_ignore_ascii_case("buffer") {
            parameter.udp_buffer = value_str.parse()?;
        } else if property.eq_ignore_ascii_case("blocksize") {
            parameter.block_size = BlockSize(value_str.parse()?);
        } else if property.eq_ignore_ascii_case("verbose") {
            parameter.verbose_yn = value_str == "yes";
        } else if property.eq_ignore_ascii_case("transcript") {
            parameter.transcript_yn = value_str == "yes";
        } else if property.eq_ignore_ascii_case("ip") {
            parameter.ipv6_yn = value_str == "v6";
        } else if property.eq_ignore_ascii_case("output") {
            parameter.output_mode = if value_str == "screen" {
                OutputMode::Screen
            } else {
                OutputMode::Line
            };
        } else if property.eq_ignore_ascii_case("rateadjust") {
            parameter.rate_adjust = value_str == "yes";
        } else if property.eq_ignore_ascii_case("rate") {
            parameter.target_rate = parse_rate(value_str)?;
        } else if property.eq_ignore_ascii_case("error") {
            parameter.error_rate = ErrorRate(
                value_str
                    .parse::<u32>()?
                    .checked_mul(1000)
                    .expect("error rate overflow"),
            );
        } else if property.eq_ignore_ascii_case("slowdown") {
            parameter.slower = parse_fraction(value_str)?;
        } else if property.eq_ignore_ascii_case("speedup") {
            parameter.faster = parse_fraction(value_str)?;
        } else if property.eq_ignore_ascii_case("history") {
            parameter.history = value_str.parse()?;
        } else if property.eq_ignore_ascii_case("lossless") {
            parameter.lossless = value_str == "yes";
        } else if property.eq_ignore_ascii_case("losswindow") {
            parameter.losswindow_ms = value_str.parse()?;
        } else if property.eq_ignore_ascii_case("blockdump") {
            parameter.blockdump = value_str == "yes";
        } else if property.eq_ignore_ascii_case("passphrase") {
            parameter.passphrase = Some(value_str.to_owned());
        }
    }

    let property = if command.len() > 1 { command[1] } else { "" };

    // report on current values
    if do_all || property.eq_ignore_ascii_case("server") {
        println!("server = {}", parameter.server);
    }
    if do_all || property.eq_ignore_ascii_case("udpport") {
        println!("udpport = {}", i32::from(parameter.client_port));
    }
    if do_all || property.eq_ignore_ascii_case("buffer") {
        println!("buffer = {}", parameter.udp_buffer);
    }
    if do_all || property.eq_ignore_ascii_case("blocksize") {
        println!("blocksize = {}", parameter.block_size);
    }
    if do_all || property.eq_ignore_ascii_case("verbose") {
        println!(
            "verbose = {}",
            if parameter.verbose_yn { "yes" } else { "no" },
        );
    }
    if do_all || property.eq_ignore_ascii_case("transcript") {
        println!(
            "transcript = {}",
            if parameter.transcript_yn { "yes" } else { "no" },
        );
    }
    if do_all || property.eq_ignore_ascii_case("ip") {
        println!("ip = {}", if parameter.ipv6_yn { "v6" } else { "v4" });
    }
    if do_all || property.eq_ignore_ascii_case("output") {
        println!(
            "output = {}",
            match parameter.output_mode {
                OutputMode::Screen => "screen",
                OutputMode::Line => "line",
            },
        );
    }
    if do_all || property.eq_ignore_ascii_case("rate") {
        println!("rate = {}", parameter.target_rate);
    }
    if do_all || property.eq_ignore_ascii_case("rateadjust") {
        println!(
            "rateadjust = {}",
            if parameter.rate_adjust { "yes" } else { "no" },
        );
    }
    if do_all || property.eq_ignore_ascii_case("error") {
        println!(
            "error = {:0>.2}%",
            f64::from(parameter.error_rate.0) / 1000.0_f64
        );
    }
    if do_all || property.eq_ignore_ascii_case("slowdown") {
        println!(
            "slowdown = {}/{}",
            parameter.slower.numerator, parameter.slower.denominator,
        );
    }
    if do_all || property.eq_ignore_ascii_case("speedup") {
        println!(
            "speedup = {}/{}",
            parameter.faster.numerator, parameter.faster.denominator,
        );
    }
    if do_all || property.eq_ignore_ascii_case("history") {
        println!("history = {}%", parameter.history);
    }
    if do_all || property.eq_ignore_ascii_case("lossless") {
        println!(
            "lossless = {}",
            if parameter.lossless { "yes" } else { "no" },
        );
    }
    if do_all || property.eq_ignore_ascii_case("losswindow") {
        println!("losswindow = {} msec", parameter.losswindow_ms);
    }
    if do_all || property.eq_ignore_ascii_case("blockdump") {
        println!(
            "blockdump = {}",
            if parameter.blockdump { "yes" } else { "no" },
        );
    }
    if do_all || property.eq_ignore_ascii_case("passphrase") {
        println!(
            "passphrase = {}",
            if (parameter.passphrase).is_none() {
                "default"
            } else {
                "<user-specified>"
            },
        );
    }
    println!();
    Ok(())
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

/// Returns true if the block has already been received
#[must_use]
pub fn got_block(session: &Session, blocknr: BlockIndex) -> bool {
    if blocknr > session.transfer.block_count {
        return true;
    }

    session.transfer.received[(blocknr.0 / 8) as usize] & (1 << (blocknr.0 % 8)) != 0
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
