use std::{io::Write, path::PathBuf, sync::Arc, time::Instant};

use ::libc;
use anyhow::bail;

use super::{ring, OutputMode, Parameter, Session, Statistics, Transfer};
use crate::{
    datagram::{self, BlockType},
    extc,
    message::{ClientToServer, DirListStatus, ServerToClient},
    types::{BlockIndex, BlockSize, ErrorRate, FileMetadata, FileSize, Fraction, TargetRate},
};

pub fn command_close(parameter: &Parameter, session: Option<Session>) -> anyhow::Result<()> {
    if parameter.verbose_yn {
        if session.is_some() {
            println!("Connection closed.");
        } else {
            println!("No connection currently active.")
        }
        println!();
    }

    Ok(())
}

pub fn command_connect(command: &[&str], parameter: &mut Parameter) -> anyhow::Result<Session> {
    if command.len() > 1 {
        parameter.server = command[1].to_owned();
    }

    let mut session = Session {
        transfer: Default::default(),
        server: super::network::create_tcp_socket_client(parameter)?,
    };

    if let Err(err) = super::protocol::ttp_negotiate_client(&mut session) {
        bail!("Protocol negotiation failed: {:?}", err);
    }

    let secret = match &parameter.passphrase {
        Some(passphrase) => passphrase.clone(),
        None => "kitten".to_owned(),
    };
    if let Err(err) = super::protocol::ttp_authenticate_client(&mut session, secret) {
        bail!("Authentication failure: {:?}", err);
    }

    if parameter.verbose_yn {
        println!("Connected.\n");
    }

    Ok(session)
}

pub fn command_dir(_command: &[&str], session: &mut Session) -> anyhow::Result<()> {
    session.write(ClientToServer::DirList)?;

    let ServerToClient::DirListHeader { status, num_files } = session.read()? else {
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
        let ServerToClient::DirListFile(file_metadata) = session.read()? else {
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

    session.write(ClientToServer::DirListEnd)?;

    Ok(())
}

pub fn command_get(
    command: &[&str],
    parameter: &mut Parameter,
    session: &mut Session,
) -> anyhow::Result<()> {
    if (command.len() as libc::c_int) < 2 as libc::c_int {
        bail!("Invalid command syntax (use 'help get' for details)");
    }

    session.transfer = Transfer::default();
    let mut multimode = false;
    let mut file_names: Vec<PathBuf> = vec![];

    if command[1] == "*" {
        multimode = true;
        println!("Requesting all available files");

        session.write(ClientToServer::MultiRequest)?;
        let ServerToClient::MultiFileCount(count) = session.read()? else {
            bail!("Expected file count");
        };

        session.write(ClientToServer::MultiAcknowledgeCount)?;

        if count == 0 {
            bail!("Server advertised no files to get");
        } else {
            println!();
            println!("Server is sharing {} files", count);
            println!("Multi-GET of {} files:", count);

            for _i in 0..count {
                let ServerToClient::MultiFile(file_metadata) = session.read()? else {
                    bail!("Expected file");
                };
                let FileMetadata { path, size } = file_metadata;

                println!(" {} ({} bytes)", path.display(), size.0);
                file_names.push(path);
            }

            session.write(ClientToServer::MultiEnd)?;
            session.flush()?;
        }
    } else {
        file_names.push(PathBuf::from(command[1].to_owned()));
    }

    let mut stats_iteration = 0;
    let mut successful = true;

    'outer: for remote_filename in file_names {
        let local_filename = if multimode {
            println!("GET *: now requesting file '{}'", remote_filename.display());
            remote_filename.clone()
        } else if command.len() >= 3 {
            // Local filename was specified
            PathBuf::from(command[2])
        } else if let Some(file_name_part) = remote_filename.file_name() {
            // Remote filename contains slash, use only the last part as the local filename
            PathBuf::from(file_name_part)
        } else {
            // Remote filename does not contain slash, use it as the local filename in its
            // entirety
            remote_filename.clone()
        };

        super::protocol::ttp_open_transfer_client(
            session,
            parameter,
            remote_filename,
            local_filename,
        )?;

        unsafe {
            super::protocol::ttp_open_port_client(session, parameter)?;
        }

        session.transfer.retransmit.previous_table = vec![];
        session.transfer.received = vec![0; (session.transfer.block_count.0 / 8 + 2) as usize];
        session.transfer.ring_buffer = Some(Arc::new(super::ring::RingBuffer::create(
            parameter.block_size,
        )));

        let mut local_datagram_buffer =
            ring::allocate_zeroed_boxed_slice(6 + parameter.block_size.0 as usize);

        let cloned_ring_buffer = Arc::clone(session.transfer.ring_buffer.as_mut().unwrap());
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

        session.transfer.next_block = BlockIndex(1);
        session.transfer.gapless_to_block = BlockIndex(0);

        session.transfer.stats = Statistics::default();

        session.transfer.stats.start_udp_errors = unsafe { crate::common::get_udp_in_errors() };
        session.transfer.stats.this_udp_errors = session.transfer.stats.start_udp_errors;

        session.transfer.stats.start_time = Some(Instant::now());
        session.transfer.stats.this_time = Some(Instant::now());
        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::xscript_data_start_client(
                session,
            ));
        }

        let mut dumpcount = 0_u32;

        loop {
            let status;

            unsafe {
                status = extc::recvfrom(
                    session.transfer.udp_fd,
                    local_datagram_buffer.as_mut_ptr() as *mut libc::c_void,
                    (6 as libc::c_int as u32).wrapping_add(parameter.block_size.0) as usize,
                    0 as libc::c_int,
                    extc::__SOCKADDR_ARG {
                        __sockaddr__: std::ptr::null_mut::<libc::c_void>() as *mut extc::sockaddr,
                    },
                    std::ptr::null_mut::<extc::socklen_t>(),
                ) as libc::c_int;
                if status < 0 as libc::c_int {
                    println!("WARNING: UDP data transmission error");
                    extc::printf(
                        b"Apparently frozen transfer, trying to do retransmit request\n\0"
                            as *const u8 as *const libc::c_char,
                    );
                    if let Err(err) = super::protocol::ttp_repeat_retransmit(session) {
                        println!(
                            "WARNING: Repeat of retransmission requests failed: {:?}",
                            err
                        );
                        successful = false;
                        break 'outer;
                    }
                }
            }

            // retrieve the block number and block type
            let local_datagram_view = datagram::View::parse(&local_datagram_buffer);
            let this_block = local_datagram_view.header.block_index; // 1-based
            let this_type = local_datagram_view.header.block_type;

            // keep statistics on received blocks
            session.transfer.stats.total_blocks =
                session.transfer.stats.total_blocks + BlockIndex(1);
            if !matches!(this_type, BlockType::Retransmission) {
                session.transfer.stats.this_flow_originals =
                    session.transfer.stats.this_flow_originals + BlockIndex(1);
            } else {
                session.transfer.stats.this_flow_retransmitteds =
                    session.transfer.stats.this_flow_retransmitteds + BlockIndex(1);
                session.transfer.stats.total_recvd_retransmits =
                    session.transfer.stats.total_recvd_retransmits + BlockIndex(1);
            }

            // main transfer control logic
            if !session.transfer.ring_buffer.as_mut().unwrap().is_full() // don't let disk-I/O freeze stop feedback of stats to server
                && (!got_block(session, this_block)
                    || matches!(this_type, BlockType::Final)
                    || session.transfer.restart_pending)
            {
                // insert new blocks into disk write ringbuffer
                if !got_block(session, this_block) {
                    // reserve ring space, copy the data in, confirm the reservation
                    session
                        .transfer
                        .ring_buffer
                        .as_mut()
                        .unwrap()
                        .reserve(local_datagram_view);
                    session.transfer.ring_buffer.as_mut().unwrap().confirm();

                    // mark the block as received
                    let fresh1 = &mut session.transfer.received[(this_block.0 / 8) as usize];
                    *fresh1 = (*fresh1 | 1 << (this_block.0 % 8)) as u8;

                    if session.transfer.blocks_left.is_zero() {
                        println!("Oops! Negative-going blocks_left count at block: type={:?} this={} final={} left={}",
                                this_type,
                                this_block.0,
                                session.transfer.block_count.0,
                                session.transfer.blocks_left.0,
                            );
                    } else {
                        session.transfer.blocks_left = session.transfer.blocks_left - BlockIndex(1);
                    }
                }

                // transmit restart: avoid re-triggering on blocks still down the wire before
                // server reacts
                let mut should_continue = true;
                if session.transfer.restart_pending
                    && !matches!(this_type, BlockType::Final)
                    && this_block > session.transfer.restart_lastidx
                    && this_block <= session.transfer.restart_wireclearidx
                {
                    should_continue = false;
                }

                if should_continue {
                    // queue any retransmits we need
                    if this_block > session.transfer.next_block {
                        if parameter.lossless {
                            // lossless transfer mode, request all missing data to be resent
                            let mut block = session.transfer.next_block;
                            while block < this_block {
                                super::protocol::ttp_request_retransmit(session, block);
                                block = block + BlockIndex(1);
                            }
                        } else {
                            // lossy transfer mode
                            if parameter.losswindow_ms == 0 {
                                session.transfer.gapless_to_block = this_block;
                            } else {
                                let mut path_capability: f64 = 0.8f64
                                    * (session.transfer.stats.this_transmit_rate
                                        + session.transfer.stats.this_retransmit_rate);
                                path_capability *= 0.001f64 * parameter.losswindow_ms as f64;

                                let first = 1_000_000.0 * path_capability
                                    / (8 * parameter.block_size.0) as f64;
                                let second =
                                    (this_block - session.transfer.gapless_to_block).0 as f64;
                                let block_diff = if first < second { first } else { second };

                                let earliest_block =
                                    BlockIndex((this_block.0 as f64 - block_diff) as u32);
                                let mut block = earliest_block;
                                while block < this_block {
                                    super::protocol::ttp_request_retransmit(session, block);
                                    block = block + BlockIndex(1);
                                }

                                session.transfer.next_block = earliest_block;
                                session.transfer.gapless_to_block = earliest_block;
                            }
                        }
                    }

                    while got_block(session, session.transfer.gapless_to_block + BlockIndex(1))
                        && session.transfer.gapless_to_block < session.transfer.block_count
                    {
                        session.transfer.gapless_to_block =
                            session.transfer.gapless_to_block + BlockIndex(1);
                    }

                    if matches!(this_type, BlockType::Normal) {
                        session.transfer.next_block = this_block + BlockIndex(1);
                    }

                    if session.transfer.restart_pending
                        && session.transfer.next_block >= session.transfer.restart_lastidx
                    {
                        session.transfer.restart_pending = false;
                    }

                    if matches!(this_type, BlockType::Final) {
                        if session.transfer.blocks_left == BlockIndex(0) {
                            break;
                        }
                        if !parameter.lossless
                            && session.transfer.retransmit.previous_table.is_empty()
                            && !session.transfer.restart_pending
                        {
                            break;
                        }

                        let mut block = session.transfer.gapless_to_block + BlockIndex(1);
                        while block < session.transfer.block_count {
                            super::protocol::ttp_request_retransmit(session, block);
                            block = block + BlockIndex(1);
                        }
                        super::protocol::ttp_repeat_retransmit(session)?;
                    }
                }
            }
            if session.transfer.stats.total_blocks.0 % 50 != 0 {
                continue;
            }
            if crate::common::get_usec_since(session.transfer.stats.this_time.unwrap()) <= 350000 {
                continue;
            }

            super::protocol::ttp_repeat_retransmit(session)?;
            super::protocol::ttp_update_stats(session, parameter, &mut stats_iteration)?;

            if parameter.blockdump {
                let mut postfix = format!(".bmap{}", dumpcount);
                if let Err(err) = dump_blockmap(&postfix, &session.transfer) {
                    eprintln!("Failed to write blockmap dump: {:?}", err);
                }
                dumpcount = dumpcount.wrapping_add(1);
            }
        }

        println!("Transfer complete. Flushing to disk and signaling server to stop...");
        unsafe {
            extc::close(session.transfer.udp_fd);
        }

        if let Err(err) = super::protocol::ttp_request_stop(session) {
            println!("WARNING: Could not request end of transfer: {:?}", err);
            successful = false;
            break;
        } else {
            session
                .transfer
                .ring_buffer
                .as_mut()
                .unwrap()
                .reserve_zero();
            session.transfer.ring_buffer.as_mut().unwrap().confirm();

            if let Err(err) = disk_thread_handle.join() {
                println!("Error in disk thread: {:?}", err);
            }

            session.transfer.stats.stop_time = Some(Instant::now());
            let delta = crate::common::get_usec_since(session.transfer.stats.start_time.unwrap());

            session.transfer.stats.total_lost = BlockIndex(0);
            let mut block = BlockIndex(1);
            while block <= session.transfer.block_count {
                if !got_block(session, block) {
                    session.transfer.stats.total_lost =
                        session.transfer.stats.total_lost + BlockIndex(1);
                }
                block = block + BlockIndex(1);
            }

            let bit_thru = 8.0f64
                * session.transfer.stats.total_blocks.0 as f64
                * parameter.block_size.0 as f64;
            let bit_good = bit_thru
                - 8.0f64
                    * session.transfer.stats.total_recvd_retransmits.0 as f64
                    * parameter.block_size.0 as f64;
            let bit_file = 8.0f64 * session.transfer.file_size.0 as f64;

            let mbit_thru = bit_thru / 1_000_000.0;
            let mbit_good = bit_good / 1_000_000.0;
            let mbit_file = bit_file / 1_000_000.0;

            let time_secs = delta as f64 / 1e6f64;

            println!("PC performance figure : {} packets dropped (if high this indicates receiving PC overload)",
                session.transfer.stats.this_udp_errors
                    .saturating_sub(session.transfer.stats.start_udp_errors),
            );
            println!("Transfer duration     : {:0>.2} seconds", time_secs);
            println!("Total packet data     : {:0>.2} Mbit", mbit_thru);
            println!("Goodput data          : {:0>.2} Mbit", mbit_good);
            println!("File data             : {:0>.2} Mbit", mbit_file);
            println!(
                "Throughput            : {:0>.2} Mbps",
                mbit_thru / time_secs
            );
            println!(
                "Goodput w/ restarts   : {:0>.2} Mbps",
                mbit_good / time_secs
            );
            println!(
                "Final file rate       : {:0>.2} Mbps",
                mbit_file / time_secs
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
                    100.0f64 * session.transfer.stats.total_lost.0 as f64
                        / session.transfer.block_count.0 as f64,
                );
            }
            println!();

            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::xscript_data_stop_client(
                    session,
                ));
                crate::common::transcript_warn_error(super::transcript::xscript_close_client(
                    session, parameter, delta,
                ));
            }

            if parameter.blockdump {
                if let Err(err) = dump_blockmap(".blockmap", &session.transfer) {
                    eprintln!("Failed to write blockmap: {}", err);
                }
            }

            session.transfer.retransmit.previous_table = vec![];

            if parameter.rate_adjust {
                parameter.target_rate =
                    TargetRate((1.15f64 * 1e6f64 * (mbit_file / time_secs)) as u64);
                println!(
                    "Adjusting target rate to {} Mbps for next transfer.",
                    (parameter.target_rate.0 as f64 / 1e6f64),
                );
            }
        }
    }

    if !successful {
        eprintln!("Transfer not successful.  (WARNING: You may need to reconnect.)");
        eprintln!();

        unsafe {
            extc::close(session.transfer.udp_fd);
        }
        session.transfer.retransmit.previous_table.clear();

        bail!("Transfer unsuccessful");
    }

    Ok(())
}

pub fn command_help(command: &[&str]) -> anyhow::Result<()> {
    if command.len() < 2 {
        println!("Help is available for the following commands:\n");
        println!("    close    connect    get    dir    help    quit    set\n");
        println!("Use 'help <command>' for help on an individual command.\n");
    } else if command[1].eq_ignore_ascii_case("close") {
        println!("Usage: close");
        println!();
        println!("Closes the current connection to a remote Tsunami server.\n");
    } else if command[1].eq_ignore_ascii_case("connect") {
        println!("Usage: connect");
        println!("       connect <remote-host>");
        println!("       connect <remote-host> <remote-port>\n");
        println!("Opens a connection to a remote Tsunami server.  If the host and port");
        println!("are not specified, default values are used.  (Use the 'set' command to");
        println!("modify these values.)\n");
        println!("After connecting, you will be prompted to enter a shared secret for");
        println!("authentication.\n");
    } else if command[1].eq_ignore_ascii_case("get") {
        println!("Usage: get <remote-file>");
        println!("       get <remote-file> <local-file>\n");
        println!("Attempts to retrieve the remote file with the given name using the");
        println!("Tsunami file transfer protocol.  If the local filename is not");
        println!("specified, the final part of the remote filename (after the last path");
        println!("separator) will be used.\n");
    } else if command[1].eq_ignore_ascii_case("dir") {
        println!("Usage: dir\n");
        println!("Attempts to list the available remote files.\n");
    } else if command[1].eq_ignore_ascii_case("help") {
        println!("Come on.  You know what that command does.\n");
    } else if command[1].eq_ignore_ascii_case("quit") {
        println!("Usage: quit\n");
        println!("Closes any open connection to a remote Tsunami server and exits the");
        println!("Tsunami client.\n");
    } else if command[1].eq_ignore_ascii_case("set") {
        println!("Usage: set");
        println!("       set <field>");
        println!("       set <field> <value>\n");
        println!("Sets one of the defaults to the given value.  If the value is omitted,");
        println!("the current value of the field is returned.  If the field is also");
        println!("omitted, the current values of all defaults are returned.\n");
    } else {
        println!("'{}' is not a recognized command.", command[1]);
        println!("Use 'help' for a list of commands.\n");
    }
    Ok(())
}

pub fn command_quit() {
    println!("Thank you for using Tsunami.\n\0");
    println!("The ANML web site can be found at:    http://www.anml.iu.edu/");
    println!("The SourceForge site can be found at: http://tsunami-udp.sf.net/");
    println!();

    std::process::exit(0);
}

pub fn command_set(command: &[&str], parameter: &mut Parameter) -> anyhow::Result<()> {
    let do_all = command.len() == 1;

    if command.len() == 3 {
        if command[1].eq_ignore_ascii_case("server") {
            parameter.server = command[2].to_owned();
        } else if command[1].eq_ignore_ascii_case("udpport") {
            parameter.client_port = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("buffer") {
            parameter.udp_buffer = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("blocksize") {
            parameter.block_size = BlockSize(command[2].parse()?);
        } else if command[1].eq_ignore_ascii_case("verbose") {
            parameter.verbose_yn = command[2] == "yes";
        } else if command[1].eq_ignore_ascii_case("transcript") {
            parameter.transcript_yn = command[2] == "yes";
        } else if command[1].eq_ignore_ascii_case("ip") {
            parameter.ipv6_yn = command[2] == "v6";
        } else if command[1].eq_ignore_ascii_case("output") {
            parameter.output_mode = if command[2] == "screen" {
                OutputMode::Screen
            } else {
                OutputMode::Line
            };
        } else if command[1].eq_ignore_ascii_case("rateadjust") {
            parameter.rate_adjust = command[2] == "yes";
        } else if command[1].eq_ignore_ascii_case("rate") {
            parameter.target_rate = parse_rate(command[2])?;
        } else if command[1].eq_ignore_ascii_case("error") {
            parameter.error_rate = ErrorRate(command[2].parse::<u32>()? * 1000);
        } else if command[1].eq_ignore_ascii_case("slowdown") {
            parameter.slower = parse_fraction(command[2])?;
        } else if command[1].eq_ignore_ascii_case("speedup") {
            parameter.faster = parse_fraction(command[2])?;
        } else if command[1].eq_ignore_ascii_case("history") {
            parameter.history = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("lossless") {
            parameter.lossless = command[2] == "yes";
        } else if command[1].eq_ignore_ascii_case("losswindow") {
            parameter.losswindow_ms = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("blockdump") {
            parameter.blockdump = command[2] == "yes";
        } else if command[1].eq_ignore_ascii_case("passphrase") {
            parameter.passphrase = Some(command[2].to_owned());
        }
    }
    if do_all || command[1].eq_ignore_ascii_case("server") {
        println!("server = {}", parameter.server);
    }
    if do_all || command[1].eq_ignore_ascii_case("udpport") {
        println!("udpport = {}", parameter.client_port as libc::c_int);
    }
    if do_all || command[1].eq_ignore_ascii_case("buffer") {
        println!("buffer = {}", parameter.udp_buffer);
    }
    if do_all || command[1].eq_ignore_ascii_case("blocksize") {
        println!("blocksize = {}", parameter.block_size);
    }
    if do_all || command[1].eq_ignore_ascii_case("verbose") {
        println!(
            "verbose = {}",
            if parameter.verbose_yn { "yes" } else { "no" },
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("transcript") {
        println!(
            "transcript = {}",
            if parameter.transcript_yn { "yes" } else { "no" },
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("ip") {
        println!("ip = {}", if parameter.ipv6_yn { "v6" } else { "v4" });
    }
    if do_all || command[1].eq_ignore_ascii_case("output") {
        println!(
            "output = {}",
            match parameter.output_mode {
                OutputMode::Screen => "screen",
                OutputMode::Line => "line",
            },
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("rate") {
        println!("rate = {}", parameter.target_rate);
    }
    if do_all || command[1].eq_ignore_ascii_case("rateadjust") {
        println!(
            "rateadjust = {}",
            if parameter.rate_adjust { "yes" } else { "no" },
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("error") {
        println!(
            "error = {:0>.2}%",
            parameter.error_rate.0 as f64 / 1000.0f64
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("slowdown") {
        println!(
            "slowdown = {}/{}",
            parameter.slower.numerator, parameter.slower.denominator,
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("speedup") {
        println!(
            "speedup = {}/{}",
            parameter.faster.numerator, parameter.faster.denominator,
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("history") {
        println!("history = {}%", parameter.history);
    }
    if do_all || command[1].eq_ignore_ascii_case("lossless") {
        println!(
            "lossless = {}",
            if parameter.lossless { "yes" } else { "no" },
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("losswindow") {
        println!("losswindow = {} msec", parameter.losswindow_ms);
    }
    if do_all || command[1].eq_ignore_ascii_case("blockdump") {
        println!(
            "blockdump = {}",
            if parameter.blockdump { "yes" } else { "no" },
        );
    }
    if do_all || command[1].eq_ignore_ascii_case("passphrase") {
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

pub fn disk_thread(
    ring_buffer: Arc<super::ring::RingBuffer>,
    block_count: BlockIndex,
    file_size: FileSize,
    mut file: std::fs::File,
) -> anyhow::Result<()> {
    loop {
        ring_buffer.peek(|datagram_view| {
            if datagram_view.header.block_index == BlockIndex(0) {
                bail!("!!!!");
            }
            super::io::accept_block(datagram_view, block_count, file_size, &mut file)?;
            Ok(())
        })?;
        ring_buffer.pop();
    }
}

pub fn parse_rate(rate: &str) -> anyhow::Result<TargetRate> {
    let (main_part, last_char) = rate.split_at(rate.len() - 1);
    let parsed: u64 = main_part.parse()?;

    let value = match last_char {
        "k" | "K" => parsed * 1000,
        "m" | "M" => parsed * 1000000,
        "g" | "G" => parsed * 1000000000,
        "t" | "T" => parsed * 1000000000000,
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => rate.parse()?,
        _ => bail!("Invalid unit specifier"),
    };

    Ok(TargetRate(value))
}

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

pub fn got_block(session: &Session, blocknr: BlockIndex) -> bool {
    if blocknr > session.transfer.block_count {
        return true;
    }

    session.transfer.received[(blocknr.0 / 8) as usize] & (1 << (blocknr.0 % 8)) != 0
}

pub fn dump_blockmap(postfix: &str, xfer: &Transfer) -> anyhow::Result<()> {
    let mut fname = xfer.local_filename.as_ref().unwrap().clone();
    let mut file_name_part = fname.file_name().unwrap().to_owned();
    file_name_part.push(postfix);
    fname.set_file_name(file_name_part);

    let mut fbits = std::fs::File::options()
        .write(true)
        .create(true)
        .open(fname)?;

    fbits.write_all(&xfer.block_count.0.to_le_bytes())?;

    let block_data = &xfer.received[0..((xfer.block_count.0 / 8).wrapping_add(1) as usize)];
    fbits.write_all(block_data)?;

    Ok(())
}
