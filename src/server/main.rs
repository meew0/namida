use std::{
    cmp::Ordering,
    io::{ErrorKind, Read},
    time::Instant,
};

use super::{Parameter, Session, Transfer};

use crate::{
    common::SocketWrapper,
    datagram::BlockType,
    message::{ClientToServer, FileRequest, NoiseHeader, TransmissionControl},
    server::Properties,
    types::{BlockIndex, ErrorRate, FileMetadata, FileSize},
};
use anyhow::bail;

/// The server's main function.
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn serve(mut parameter: Parameter) -> anyhow::Result<()> {
    // process our command-line options
    process_options(&mut parameter);

    // obtain our server socket
    let listener = super::network::create_tcp_socket(&parameter)?;

    // now show version / build information
    eprintln!(
        "namida server for protocol revision {} (block size = {}, magic = 0x{:x})\nVersion: {} (revision {})\nCompiled: {}\nWaiting for clients to connect.",
        crate::version::NAMIDA_PROTOCOL_REVISION,
        crate::common::BLOCK_SIZE,
        crate::version::magic(parameter.encrypted),
        crate::version::NAMIDA_VERSION,
        &crate::version::GIT_HASH[0..7],
        crate::version::COMPILE_DATE_TIME,
    );

    // “while our little world keeps turning”...
    for (session_id, result) in listener.incoming().enumerate() {
        // accept a new client connection
        let socket = result?;
        eprintln!("New client connecting from {}...", socket.peer_addr()?);

        // create a new thread to handle the client connection. (We use threads here instead of
        // sub-processes like Tsunami originally did)
        let parameter_cloned = parameter.clone();
        std::thread::spawn(move || {
            // set up the session structure
            let session = Session {
                transfer: Transfer::default(),
                properties: Properties::default(),
                client: SocketWrapper::new(socket),
                session_id,
            };

            // and run the client handler, catching any panics so we can inform the user about what
            // happened
            let result = client_handler(session, &parameter_cloned);

            match result {
                Ok(()) => eprintln!("Child server thread terminated succcessfully."),
                Err(err) => eprintln!("Child server thread terminated with error: {err}"),
            };
        });
    }

    Ok(())
}

/// This routine is run by the client processes that are created in response to incoming
/// connections.
#[allow(clippy::missing_errors_doc)]
#[allow(clippy::missing_panics_doc)]
pub fn client_handler(mut session: Session, parameter: &Parameter) -> anyhow::Result<()> {
    // negotiate the connection parameters
    // We call it negotiation, but we unilaterally impose our parameters on the client!
    super::protocol::negotiate(&mut session, parameter)?;

    // have the client try to authenticate to us, and potentially initiate an encrypted connection
    if parameter.encrypted {
        super::protocol::authenticate_encrypted(&mut session, &parameter.secret)?;
    } else {
        super::protocol::authenticate_unencrypted(&mut session, &parameter.secret)?;
    };

    if parameter.verbose_yn {
        println!("Client authenticated. Negotiated parameters are:");
        println!("Block size: {}", crate::common::BLOCK_SIZE);
        println!("Buffer size: {}", parameter.udp_buffer);
        println!(
            "Encryption: {}",
            if parameter.encrypted { "yes" } else { "no" }
        );
    }

    // while we haven't been told to stop
    loop {
        // Make the client socket blocking (for the case that it has been set to non-blocking
        // in a previous loop iteration)
        session.client.socket.set_nonblocking(false)?;

        // negotiate another transfer
        let request = session.client.read()?;

        match request {
            ClientToServer::FileRequest(file_request) => {
                handle_transfer(&mut session, parameter, file_request)?;
            }
            ClientToServer::FileListRequest => {
                super::protocol::send_file_list(&mut session, parameter)?;
            }
            _ => bail!("Expected a request from the client but got: {request:?}"),
        }
    }
}

fn handle_transfer(
    session: &mut Session,
    parameter: &Parameter,
    file_request: FileRequest,
) -> anyhow::Result<()> {
    if let Err(err) = super::protocol::open_transfer(session, parameter, file_request) {
        println!("WARNING: Invalid file request, error: {err:?}");
        bail!("Closing connection to client.");
    }

    // Find out which method the client would like to use to receive UDP data. Use the opportunity
    // to measure the round trip time
    let ping_start = Instant::now();
    let ClientToServer::UdpInit(udp_method) = session.client.read()? else {
        bail!("Expected UdpInit");
    };
    let ping_end = Instant::now();
    super::protocol::start_transfer_timing(session, parameter, ping_start, ping_end);

    // Get the client's UDP address
    if let Err(err) = super::protocol::determine_client_udp_address(session, parameter, udp_method)
    {
        bail!("UDP address determination failed: {err:?}");
    }

    // Make the client socket non-blocking, to be able to skip reading a transmission control
    // request if none has been sent.
    session.client.socket.set_nonblocking(true)?;

    // Start timing
    let start = Instant::now();
    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::data_start(session));
    }

    let mut delta_µs: u64;
    let mut lasthblostreport = start;
    let mut lastfeedback = start;
    let mut previous_packet_time = start;
    let mut deadconnection_counter = 0_i32;
    let mut ipd_time = 0_i64;
    let mut ipd_time_max = 0_u64;

    let mut control_slice = [0_u8; 100];
    let mut control_slice_cursor = 0_usize;
    let mut maybe_header: Option<NoiseHeader> = None;
    let mut maybe_transmission_control: Option<TransmissionControl> = None;

    let mut retransmit_accept_iteration = 0;

    let mut datagram_block_buffer: Vec<u8> = vec![0_u8; crate::common::BLOCK_SIZE as usize];
    let datagram_buffer_extra_length = if parameter.encrypted { 30 } else { 6 };
    let mut datagram_buffer: Vec<u8> = vec![
        0_u8;
        (crate::common::BLOCK_SIZE as usize)
            .checked_add(datagram_buffer_extra_length)
            .expect("datagram buffer size overflow")
    ];

    session.transfer.block = BlockIndex(0);

    // start by blasting out every block
    while session.transfer.block <= session.properties.block_count {
        // default: flag as retransmitted block
        let mut block_type = BlockType::Retransmission;

        // precalculate time to wait after sending the next packet
        let current_packet_time = Instant::now();

        // Only perform time adjustment if the difference in microseconds fits into a u32.
        // 2^32 microseconds is about 1 hour and 12 minutes. If more time than that has
        // passed, probably something has gone horribly wrong and we should try again after the
        // next packet.
        let micros_diff_maybe: Result<u32, _> = current_packet_time
            .duration_since(previous_packet_time)
            .as_micros()
            .try_into();
        if let Ok(micros_diff_u32) = micros_diff_maybe {
            let micros_diff = f64::from(micros_diff_u32);
            #[allow(clippy::cast_possible_truncation)]
            let ipd_usleep_diff: i64 = (session.transfer.ipd_current - micros_diff) as i64;
            previous_packet_time = current_packet_time;

            if ipd_usleep_diff > 0 || ipd_time > 0 {
                ipd_time = ipd_time
                    .checked_add(ipd_usleep_diff)
                    .expect("ipd_time over- or underflow");
            }

            let ipd_time_non_negative: u64 = ipd_time.try_into().unwrap_or(0);
            ipd_time_max = ipd_time_max.max(ipd_time_non_negative);
        }

        // see if transmit requests are available
        if parameter.encrypted {
            if let Some(header) = maybe_header.as_ref() {
                let done = try_read(
                    session,
                    &mut control_slice,
                    &mut control_slice_cursor,
                    header.length as usize,
                )?;

                if done {
                    // Decrypt and decode transmission control payload
                    maybe_transmission_control = Some(session.client.decrypt_decode(
                        header.nonce,
                        &control_slice[..(header.length as usize)],
                    )?);
                    maybe_header = None;
                }
            } else {
                // Encrypted connection, but no header was read yet. Try to read one
                let done = try_read(
                    session,
                    &mut control_slice,
                    &mut control_slice_cursor,
                    NoiseHeader::SIZE,
                )?;

                if done {
                    // we have read enough bytes for a full header
                    maybe_header = Some(
                        bincode::decode_from_slice(
                            &control_slice[..NoiseHeader::SIZE],
                            crate::common::BINCODE_CONFIG,
                        )?
                        .0,
                    );
                }
            }
        } else {
            // Unencrypted connection. Try to read a `TransmissionControl` directly
            let done = try_read(
                session,
                &mut control_slice,
                &mut control_slice_cursor,
                TransmissionControl::SIZE,
            )?;

            if done {
                // we have read enough bytes for a full transmission control request
                maybe_transmission_control = Some(
                    bincode::decode_from_slice(
                        &control_slice[..TransmissionControl::SIZE],
                        crate::common::BINCODE_CONFIG,
                    )?
                    .0,
                );
            }
        }

        if let Some(transmission_control) = maybe_transmission_control {
            // store current time
            lastfeedback = current_packet_time;
            lasthblostreport = current_packet_time;
            deadconnection_counter = 0;

            // if it's a stop request, go back to waiting for a file request
            if let TransmissionControl::EndTransmission(_) = transmission_control {
                let filename = session
                    .transfer
                    .filename
                    .as_ref()
                    .expect("filename should be available");
                eprintln!("Transmission of {} complete.", filename.display());

                if let Some(finishhook) = &parameter.finishhook {
                    eprintln!("Executing: {} {}", finishhook.display(), filename.display());

                    let spawned = std::process::Command::new(finishhook).arg(filename).spawn();

                    if let Err(err) = spawned {
                        eprintln!("Could not execute finish hook: {err}");
                    }
                }
                break;
            }

            // otherwise, handle the retransmission request
            if let Err(err) = super::protocol::accept_retransmit(
                session,
                parameter,
                &transmission_control,
                datagram_block_buffer.as_mut_slice(),
                datagram_buffer.as_mut_slice(),
                &mut retransmit_accept_iteration,
            ) {
                println!("WARNING: Retransmission error: {err:?}");
            }

            maybe_transmission_control = None; // wait for the next one
        } else {
            // we could not read a full transmission control request so far, so simply send
            // some blocks that haven't yet been sent
            let cont = send_next_block(
                session,
                parameter,
                &mut block_type,
                &mut datagram_block_buffer,
                &mut datagram_buffer,
            )?;
            if cont {
                continue;
            }
        }

        // monitor client heartbeat and disconnect dead client
        let old_deadconnection_counter = deadconnection_counter;
        deadconnection_counter = deadconnection_counter.saturating_add(1);
        if old_deadconnection_counter > 2048 {
            deadconnection_counter = 0;

            // limit 'heartbeat lost' reports to 500ms intervals
            if crate::common::get_µs_since(lasthblostreport) < 500_000 {
                continue;
            }

            lasthblostreport = Instant::now();

            let retransmission = TransmissionControl::SubmitErrorRate(ErrorRate(100_000));
            if let Err(err) = super::protocol::accept_retransmit(
                session,
                parameter,
                &retransmission,
                datagram_block_buffer.as_mut_slice(),
                datagram_buffer.as_mut_slice(),
                &mut retransmit_accept_iteration,
            ) {
                println!("Error in accept_retransmit: {err:?}");
            }

            delta_µs = crate::common::get_µs_since(lastfeedback);
            #[allow(clippy::cast_precision_loss)]
            let delta_seconds = 1e-6_f64 * delta_µs as f64;

            // show an (additional) statistics line
            let stats_line = format!(
                "   n/a     n/a     n/a {:7} {:6.2} {:3} -- no heartbeat since {:3.2}s\n",
                session.transfer.block.0,
                100.0_f64 * f64::from(session.transfer.block.0)
                    / f64::from(session.properties.block_count.0),
                session.session_id,
                delta_seconds,
            );
            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::data_log(
                    session,
                    &stats_line,
                ));
            }
            eprint!("{stats_line}");

            // handle timeout for normal file transfers
            if delta_seconds > f64::from(parameter.hb_timeout) {
                eprintln!(
                    "Heartbeat timeout of {} seconds reached, terminating transfer.",
                    parameter.hb_timeout,
                );
                break;
            }
        }

        // wait before handling the next packet
        if matches!(block_type, BlockType::Final) {
            crate::common::µsleep_that_works(ipd_time_max.saturating_mul(10));
        }
        if let Ok(ipd_time_non_negative) = ipd_time.try_into() {
            crate::common::µsleep_that_works(ipd_time_non_negative);
        }
    }

    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::data_stop(session));
    }

    // Stop timing
    let stop = Instant::now();
    delta_µs = stop
        .duration_since(start)
        .as_micros()
        .try_into()
        .expect("timing delta microseconds overflow");

    if parameter.verbose_yn {
        #[allow(clippy::cast_precision_loss)]
        let delta_seconds = delta_µs as f64 / 1_000_000.0_f64;

        #[allow(clippy::cast_precision_loss)]
        let megabits_per_second = 8.0_f64 * session.properties.file_size.0 as f64 / delta_µs as f64;

        eprintln!(
            "Server {} transferred {} bytes in {:0>.2} seconds ({:0>.1} Mbps)",
            session.session_id, session.properties.file_size.0, delta_seconds, megabits_per_second,
        );
    }

    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::close(session, delta_µs));
    }

    session.transfer.udp_socket.take();

    Ok(())
}

/// Tries to read enough bytes to fill the `slice` from `cursor` to `max`. If not enough bytes are
/// available, it increments the cursor by the given amount and returns `false`. If enough bytes
/// were available, it resets the cursor to 0 and returns `true`.
fn try_read(
    session: &mut Session,
    slice: &mut [u8],
    cursor: &mut usize,
    max: usize,
) -> anyhow::Result<bool> {
    let read_result = session.client.socket.read(&mut slice[*cursor..max]);
    let read_count = match read_result {
        Ok(read_count) => read_count,
        Err(err) => {
            if matches!(err.kind(), ErrorKind::WouldBlock) {
                0
            } else {
                bail!(
                    "Error while trying to read transmission control request: {}",
                    err
                );
            }
        }
    };

    *cursor = cursor.checked_add(read_count).expect("cursor overflow");

    match (*cursor).cmp(&max) {
        Ordering::Equal => {
            *cursor = 0;
            Ok(true)
        }
        Ordering::Less => Ok(false),
        Ordering::Greater => {
            // somehow we read too much??
            panic!("read too many bytes: {cursor} > {max}");
        }
    }
}

fn send_next_block(
    session: &mut Session,
    parameter: &Parameter,
    block_type: &mut BlockType,
    datagram_block_buffer: &mut [u8],
    datagram_buffer: &mut [u8],
) -> anyhow::Result<bool> {
    // increment block index for the next datagram
    let incremented = session.transfer.block.safe_add(BlockIndex(1));
    session.transfer.block = BlockIndex::min(incremented, session.properties.block_count);

    // check whether we're sending the final block
    *block_type = if session.transfer.block == session.properties.block_count {
        BlockType::Final
    } else {
        BlockType::Original
    };

    // build the datagram
    let block_index = session.transfer.block;
    let datagram =
        super::io::build_datagram(session, block_index, *block_type, datagram_block_buffer)?;
    bincode::encode_into_slice(datagram, datagram_buffer, crate::common::BINCODE_CONFIG)?;

    // transmit the datagram
    if let Err(err) = super::protocol::send_datagram(session, parameter, datagram, datagram_buffer)
    {
        println!(
            "WARNING: Could not transmit block #{}: {}",
            session.transfer.block.0, err
        );
        return Ok(true);
    }

    Ok(false)
}

/// Perform required processing on command line options. Primarily this involves trying to open all
/// files that were specified to be served, and obtaining their file size.
pub fn process_options(parameter: &mut Parameter) {
    // Load the secret key from the secret file, if specified
    crate::common::load_secret(&parameter.secret_file, &mut parameter.secret);

    if !parameter.file_names.is_empty() {
        let total_files = parameter.file_names.len();
        eprintln!("\nThe specified {total_files} files will be listed on GET *:");

        for (counter, path) in parameter.file_names.iter().enumerate() {
            match std::fs::metadata(path) {
                Ok(metadata) => {
                    parameter.files.push(FileMetadata {
                        path: path.clone(),
                        size: FileSize(metadata.len()),
                    });
                    eprintln!(
                        " {:3}   {:<20}  {} bytes\n",
                        counter.saturating_add(1),
                        path.display(),
                        metadata.len(),
                    );
                }
                Err(err) => {
                    eprintln!(
                        "Could not get metadata of specified file: '{}', error: {}",
                        path.display(),
                        err
                    );
                }
            }
        }
    }

    // Print some specified options if the user desires
    if parameter.verbose_yn {
        eprintln!("Buffer size: {}", parameter.udp_buffer);
        eprintln!("Bind: {}", parameter.bind);
    }
}
