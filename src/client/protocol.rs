use std::{
    borrow::Cow,
    io::Write,
    path::PathBuf,
    time::{Duration, Instant},
};

use ::libc;
use anyhow::bail;

use super::{get, OutputMode, Retransmit, Session, Transfer};
use crate::{
    common::SocketWrapper,
    message::{self, ClientToServer, FileRequest, ServerToClient, TransmissionControl, UdpMethod},
    types::{BlockIndex, ErrorRate},
};

/// Opens a new control session to the specified server. On success, we return the created session
/// object.
///
/// Note that the default host and port stored in the parameter object are updated if they were
/// specified in the command itself.
///
/// # Errors
/// Returns an error on I/O failure.
pub fn connect(server: &str, encrypted: bool, secret: &[u8]) -> anyhow::Result<Session> {
    // obtain our client socket, and create a new session object with it
    let mut session = Session {
        transfer: Transfer::default(),
        server: SocketWrapper::new(super::network::create_tcp_socket(server)?),
    };

    // negotiate the connection parameters
    if let Err(err) = negotiate(&mut session, encrypted) {
        bail!("Protocol negotiation failed: {:?}", err);
    }

    // authenticate to the server, and potentially initiate an encrypted connection
    let auth_result = if encrypted {
        authenticate_encrypted(&mut session, secret)
    } else {
        authenticate_unencrypted(&mut session, secret)
    };

    if let Err(err) = auth_result {
        bail!("Authentication failure: {:?}", err);
    }

    Ok(session)
}

/// Given an active session, returns `Ok(())` if we were able to successfully authenticate to the
/// server, and an error otherwise. Used only for unencrypted connections. See the documentation of
/// `server::protocol::authenticate_unencrypted` for a description of the unencrypted authentication
/// process.
///
/// # Errors
/// Returns an error on authentication failure, I/O failure, or if the server sent unexpected data.
pub fn authenticate_unencrypted(session: &mut Session, secret: &[u8]) -> anyhow::Result<()> {
    // read in the shared secret and the challenge
    let ServerToClient::AuthenticationChallenge(mut random) = session.server.read()? else {
        bail!("Expected authentication challenge");
    };

    // prepare the proof of the shared secret
    // Tsunami manually overwrites the secret bytes with zero afterwards. I think this is snake oil.
    let digest: [u8; 16] = crate::common::prepare_proof(&mut random, secret).into();

    // send the response to the server
    session
        .server
        .write(ClientToServer::AuthenticationResponse(digest))?;

    // read the results back from the server
    let ServerToClient::AuthenticationStatus(success) = session.server.read()? else {
        bail!("Expected authentication status");
    };

    // check the result
    if !success {
        bail!("Authentication failed");
    }

    Ok(())
}

/// Authenticates to the server and establishes a shared encrypted connection.
///
/// # Errors
/// Returns an error on I/O failure, authentication failure, or when the encrypted connection could
/// not be established.
pub fn authenticate_encrypted(session: &mut Session, secret: &[u8]) -> anyhow::Result<()> {
    let mut noise_init_buffer = [0_u8; 1024];

    let builder = snow::Builder::new(crate::common::NOISE_PATTERN.parse()?);
    let static_key = builder.generate_keypair()?.private;
    let mut noise = builder
        .local_private_key(&static_key)
        .psk(3, secret)
        .build_initiator()?;

    // -> e
    let len = noise.write_message(&[], &mut noise_init_buffer)?;
    session
        .server
        .write(message::Noise(Cow::from(&noise_init_buffer[..len])))?;

    // <- e, ee, s, es
    let message::Noise(data) = session.server.read()?;
    noise.read_message(&data, &mut noise_init_buffer)?;

    // -> s, se
    let len = noise.write_message(&[], &mut noise_init_buffer)?;
    session
        .server
        .write(message::Noise(Cow::from(&noise_init_buffer[..len])))?;

    let noise = noise.into_stateless_transport_mode()?;
    session.server.set_noise_state(noise);

    println!("Encrypted session established.");

    Ok(())
}

/// Performs all of the negotiation with the remote server that is done prior to authentication.
/// At the moment, this consists of verifying identical protocol revisions between the client and
/// server. Returns `Ok(())` on success.
///
/// # Errors
/// Returns an error on negotiation failure, I/O failure, or if the server sent unexpected data.
pub fn negotiate(session: &mut Session, encrypted: bool) -> anyhow::Result<()> {
    // send our protocol revision number to the server
    let client_revision = crate::version::magic(encrypted);
    session.server.write(client_revision)?;

    // read the protocol revision number from the server
    let server_revision: u32 = session.server.read()?;

    // compare the numbers
    if client_revision != server_revision {
        bail!(
            "Protocol negotiation failed: client_revision = {}, server_revision = {}",
            client_revision,
            server_revision
        );
    }

    Ok(())
}

/// Tries to create a new TTP file request object for the given session by submitting a file request
/// to the server (which is waiting for the name of a file to transfer). If the request is accepted,
/// we retrieve the file parameters, open the file for writing, and return `Ok` with the server's
/// UDP port.
///
/// # Errors
/// Returns an error on I/O failure, if the file cannot be sent, or if the server sent unexpected
/// messages.
///
/// # Panics
/// Panics if no local path is set in the transfer object.
pub fn open_transfer(
    session: &mut Session,
    parameter: &get::Parameter,
    remote_filename: PathBuf,
    local_filename: PathBuf,
) -> anyhow::Result<u16> {
    // submit the transfer request
    session
        .server
        .write(ClientToServer::FileRequest(FileRequest {
            path: remote_filename.clone(),
            target_rate: parameter.target_rate,
            error_rate: parameter.error_rate,
            slowdown: parameter.slower,
            speedup: parameter.faster,
        }))?;

    // see if the request was successful
    let result = session.server.read()?;
    let remote_udp_port = match result {
        ServerToClient::FileRequestSuccess {
            file_size,
            block_count,
            epoch,
            udp_port,
        } => {
            // It was. Initialise the transfer
            session.transfer = Transfer::default();
            session.transfer.remote_filename = Some(remote_filename);
            session.transfer.local_filename = Some(local_filename);

            // Get the server's parameters
            session.transfer.file_size = file_size;
            session.transfer.block_count = block_count;
            session.transfer.epoch = epoch;

            // Return the server's UDP port to outside of the match block, so we can return it from
            // the function later
            udp_port
        }
        ServerToClient::FileRequestError(err) => {
            bail!(
                "Server: File does not exist or cannot be transmitted: {:?}",
                err
            );
        }
        _ => {
            bail!(
                "Expected `FileRequestSuccess` or `FileRequestError` but got: {:?}",
                result
            );
        }
    };

    // we start out with every block yet to transfer
    session.transfer.blocks_left = session.transfer.block_count;

    // try to open the local file for writing
    let local_path = session
        .transfer
        .local_filename
        .as_ref()
        .expect("there should be a local path")
        .as_path();
    if local_path.exists() {
        println!(
            "Warning: overwriting existing file '{}'",
            local_path.display()
        );
    }
    session.transfer.file = Some(
        std::fs::File::options()
            .write(true)
            .create(true)
            .open(local_path)?,
    );

    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let on_wire_estimate = BlockIndex(
        (0.5_f64 * parameter.target_rate.0 as f64
            / (f64::from(crate::common::BLOCK_SIZE) * 8.0_f64)) as u32,
    );
    session.transfer.on_wire_estimate =
        BlockIndex::min(session.transfer.block_count, on_wire_estimate);

    // if we're doing a transcript
    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::open(session, parameter));
    }

    // indicate success, and let the outside know of the server's UDP port
    Ok(remote_udp_port)
}

/// Creates a new UDP socket for receiving the file data associated with our pending transfer and
/// communicates the port number back to the server.
///
/// # Errors
/// Returns an error when a socket could not be opened or when the port number could not be
/// communicated to the server.
pub fn open_port(
    session: &mut Session,
    parameter: &get::Parameter,
    remote_port: u16,
) -> anyhow::Result<()> {
    // open a new UDP socket
    let udp_socket = session
        .transfer
        .udp_socket
        .insert(super::network::create_udp_socket(
            parameter,
            session.server.socket.local_addr()?.is_ipv6(),
        )?);

    // Let the server know of our UDP address, by one means or another.
    if parameter.discovery {
        // If discovery is desired, let the server know of that fact so it can listen for our
        // UDP message.
        session
            .server
            .write(ClientToServer::UdpInit(UdpMethod::Discovery))?;

        // Send some (non-)data to the server. This will open the port on NATs along the path to
        // the server, and also let the server know about the mapped port on the final NAT, if one
        // exists.
        let mut remote_address = session.server.socket.peer_addr()?;
        remote_address.set_port(remote_port);

        // Try to send the message every second, until the server tells us (over TCP) that it has
        // arrived.
        loop {
            udp_socket.send_to(b"namida", remote_address)?;

            if matches!(session.server.read()?, ServerToClient::UdpDone) {
                break;
            }

            std::thread::sleep(Duration::from_secs(1));
        }
    } else {
        // If discovery is not desired, find out the port number we're using and send it to the
        // server as-is.
        let port = udp_socket.local_addr()?.port();
        session
            .server
            .write(ClientToServer::UdpInit(UdpMethod::StaticPort(port)))?;
        session.server.flush()?;
    }

    Ok(())
}

/// Tries to repeat all of the outstanding retransmit requests for the current transfer on the
/// given session. This also takes care of maintenance operations on the transmission table,
/// such as relocating the entries toward the bottom of the array.
///
/// # Errors
/// Returns an error if the retransmit requests could not be resent due to I/O failure.
///
/// # Panics
/// Panics on arithmetic overflow.
pub fn repeat_retransmit(session: &mut Session) -> anyhow::Result<()> {
    session.transfer.stats.this_retransmits = BlockIndex(0);

    // Tsunami implements the retransmit table as one array that is modified in place.
    // In Rust, that is not feasible, so we use two arrays that are swapped when necessary.
    // At the start, `previous_table` contains the retransmit requests; `next_table` is only used
    // in this function as a temporary buffer.

    // Discard received blocks from the list, by iterating over the `previous_table` and inserting
    // all blocks we don't yet have into a pristine `next_table`.
    session.transfer.retransmit.next_table.clear();
    for block in &session.transfer.retransmit.previous_table {
        if session.transfer.retransmit.next_table.len()
            >= Retransmit::MAX_RETRANSMISSION_BUFFER as usize
        {
            break;
        }

        if !block.is_zero() && !session.got_block(*block) {
            session.transfer.retransmit.next_table.push(*block);
        }
    }

    // How many blocks were left over after filtering. If this is MAX_RETRANSMISSION_BUFFER
    // (or more) then we need to restart the transfer entirely.
    let next_table_len = session.transfer.retransmit.next_table.len();
    let count = BlockIndex(
        next_table_len
            .try_into()
            .expect("retransmit count overflow"),
    );

    // if there are too many entries, restart transfer from earlier point
    if next_table_len >= Retransmit::MAX_RETRANSMISSION_BUFFER as usize {
        // restart from first missing block
        let block = BlockIndex::min(
            session.transfer.block_count,
            session.transfer.gapless_to_block.safe_add(BlockIndex(1)),
        );

        // send out the request
        session
            .server
            .write(TransmissionControl::RestartAt(block))?;

        // remember the request so we can then ignore blocks that are still on the wire
        session.transfer.restart_pending = true;
        session.transfer.restart_lastidx = session
            .transfer
            .retransmit
            .previous_table
            .last()
            .copied()
            .unwrap_or(BlockIndex(0));
        session.transfer.restart_wireclearidx = BlockIndex::min(
            session.transfer.block_count,
            session
                .transfer
                .restart_lastidx
                .safe_add(session.transfer.on_wire_estimate),
        );

        // reset the retransmission table and head block
        session.transfer.retransmit.previous_table.clear();
        session.transfer.retransmit.next_table.clear();
        session.transfer.next_block = block;
        session.transfer.stats.this_retransmits = BlockIndex(Retransmit::MAX_RETRANSMISSION_BUFFER);
    } else {
        // update statistics
        session.transfer.stats.this_retransmits = count;
        session.transfer.stats.total_retransmits =
            session.transfer.stats.total_retransmits.safe_add(count);

        // send out the requests
        if next_table_len > 0 {
            let mut retransmits = Vec::with_capacity(session.transfer.retransmit.next_table.len());
            for block_index in &session.transfer.retransmit.next_table {
                retransmits.push(TransmissionControl::Retransmit(*block_index));
            }

            for retransmit in retransmits {
                session.server.write(retransmit)?;
            }

            // let the server know that we're done sending out retransmits
            session
                .server
                .write(TransmissionControl::RetransmitOver(0))?;
        }

        // clear the previous table which has now become invalid, and swap it for the next table
        session.transfer.retransmit.previous_table.clear();
        session.transfer.retransmit.swap_tables();
    }
    session.server.flush()?;
    Ok(())
}

const MAX_RETRANSMIT_TABLE_LENGTH: usize = 32 * Retransmit::MAX_RETRANSMISSION_BUFFER as usize;

/// Requests a retransmission of the given block in the current transfer.
pub fn request_retransmit(session: &mut Session, block: BlockIndex) {
    // double checking: if we already got the block, don't add it
    if session.got_block(block) {
        return;
    }

    // don't overgrow the table
    if session.transfer.retransmit.previous_table.len() > MAX_RETRANSMIT_TABLE_LENGTH {
        return;
    }

    // store the request
    session.transfer.retransmit.previous_table.push(block);
}

/// Requests that the server stop transmitting data for the current file transfer in the given
/// session. This is done by sending a transmission control request with a type of
/// `EndTransmission`.
///
/// # Errors
/// Returns an error on I/O failure.
pub fn request_stop(session: &mut Session) -> anyhow::Result<()> {
    session
        .server
        .write(TransmissionControl::EndTransmission(0))?;
    Ok(())
}

/// This routine must be called every interval to update the statistics for the progress of the
/// ongoing file transfer.
///
/// # Errors
/// Returns an error on I/O failure when sending the error rate to the server, or when flushing
/// the standard output.
///
/// # Panics
/// Panics if timings are uninitialised.
pub fn update_stats(
    session: &mut Session,
    parameter: &get::Parameter,
    iteration: &mut u64,
) -> anyhow::Result<()> {
    let u_mega: f64 = 1_000_000.0;
    let u_giga: f64 = 1_000_000_000.0;

    // find the total time elapsed
    let delta = session
        .transfer
        .stats
        .this_time
        .expect("this_time should be present")
        .elapsed();
    let delta_total = session
        .transfer
        .stats
        .start_time
        .expect("start_time should be present")
        .elapsed();

    let milliseconds = delta_total.subsec_millis();

    let mut temp = delta_total.as_secs();
    let seconds = temp % 60;
    temp /= 60;
    let minutes = temp % 60;
    temp /= 60;
    let hours = temp;

    let d_seconds = delta.as_secs_f64();
    let d_seconds_total = delta_total.as_secs_f64();

    // find the amount of data transferred (bytes)
    let data_total =
        f64::from(crate::common::BLOCK_SIZE) * f64::from(session.transfer.stats.total_blocks.0);
    let data_this = f64::from(crate::common::BLOCK_SIZE)
        * f64::from(
            (session
                .transfer
                .stats
                .total_blocks
                .safe_sub(session.transfer.stats.this_blocks))
            .0,
        );
    let data_this_rexmit = f64::from(crate::common::BLOCK_SIZE)
        * f64::from(session.transfer.stats.this_flow_retransmitteds.0);

    // update the UDP receive error count reported by the operating system
    session.transfer.stats.udp_errors.update();

    // precalculate some fractions
    let retransmits_fraction = f64::from(session.transfer.stats.this_retransmits.0)
        / (1.0_f64
            + f64::from(session.transfer.stats.this_retransmits.0)
            + f64::from(session.transfer.stats.total_blocks.0)
            - f64::from(session.transfer.stats.this_blocks.0));
    #[allow(clippy::cast_precision_loss)]
    let ringfill_fraction = f64::from(
        session
            .transfer
            .ring_buffer
            .as_ref()
            .map_or(0, |ring| ring.count()),
    ) / f64::from(super::ring::MAX_BLOCKS_QUEUED);
    let total_retransmits_fraction = f64::from(session.transfer.stats.total_retransmits.0)
        / f64::from(
            (session
                .transfer
                .stats
                .total_retransmits
                .safe_add(session.transfer.stats.total_blocks))
            .0,
        );

    // update the rate statistics
    // incoming transmit rate R = goodput R (Mbit/s) + retransmit R (Mbit/s)
    session.transfer.stats.this_transmit_rate = 8.0_f64 * data_this / (d_seconds * u_mega);
    session.transfer.stats.this_retransmit_rate = 8.0_f64 * data_this_rexmit / (d_seconds * u_mega);

    let data_total_rate = 8.0_f64 * data_total / (d_seconds_total * u_mega);
    let feedback = f64::from(parameter.history) / 100.0_f64;
    let feedforward = 1.0_f64 - feedback;

    // IIR filter rate R
    session.transfer.stats.transmit_rate = feedback.mul_add(
        session.transfer.stats.transmit_rate,
        feedforward * session.transfer.stats.this_transmit_rate,
    );
    // IIR filtered composite error and loss, some sort of knee function
    session.transfer.stats.error_rate = feedback.mul_add(
        session.transfer.stats.error_rate,
        feedforward
            * f64::from(500 as libc::c_int)
            * f64::from(100 as libc::c_int)
            * (retransmits_fraction + ringfill_fraction),
    );

    // send the current error rate information to the server
    #[allow(clippy::cast_sign_loss)]
    #[allow(clippy::cast_possible_truncation)]
    session
        .server
        .write(TransmissionControl::SubmitErrorRate(ErrorRate(
            session.transfer.stats.error_rate as u32,
        )))?;

    // build the stats string
    let stats_flags = format!(
        "{}{}",
        if session.transfer.restart_pending {
            'R' as i32
        } else {
            '-' as i32
        },
        if session
            .transfer
            .ring_buffer
            .as_ref()
            .map_or(false, |ring| ring.is_full())
        {
            'F' as i32
        } else {
            '-' as i32
        },
    );
    let stats_line = format!(
        "{:02}:{:02}:{:02}.{:03} {:4} {:6.2}M {:6.1}Mbps {:5.1}% {:7} {:6.1}G {:6.1}Mbps {:5.1}% {:5} {:5} {:7} {:8} {:8} {}\n",
        hours,
        minutes,
        seconds,
        milliseconds,
        (session.transfer.stats.total_blocks.safe_sub(session.transfer.stats.this_blocks)).0,
        session.transfer.stats.this_retransmit_rate,
        session.transfer.stats.this_transmit_rate,
        100.0_f64 * retransmits_fraction,
        session.transfer.stats.total_blocks.0,
        data_total / u_giga,
        data_total_rate,
        100.0_f64 * total_retransmits_fraction,
        session.transfer.retransmit.previous_table.len(),
        session.transfer.ring_buffer
        .as_ref().map_or(0, |ring| ring.count()),
        session.transfer.blocks_left.0,
        session.transfer.stats.this_retransmits.0,
        session.transfer.stats.udp_errors,
        stats_flags,
    );

    // give the user a show if they want it
    if parameter.verbose_yn {
        if parameter.output_mode == OutputMode::Screen {
            print!("\x1B[2J\x1B[H");
            println!("Current time:   {}", 0); // TODO
            println!("Elapsed time:   {hours:02}:{minutes:02}:{seconds:02}.{milliseconds:03}",);
            println!();
            println!("Last interval");
            println!("--------------------------------------------------");
            println!(
                "Blocks count:     {}",
                (session
                    .transfer
                    .stats
                    .total_blocks
                    .safe_sub(session.transfer.stats.this_blocks))
                .0,
            );
            println!("Data transferred: {:02} GB", data_this / u_giga);
            println!(
                "Transfer rate:    {:02} Mbps",
                session.transfer.stats.this_transmit_rate,
            );
            println!(
                "Retransmissions:  {} ({:02}%)",
                session.transfer.stats.this_retransmits.0,
                100.0_f64 * retransmits_fraction,
            );
            println!();
            println!("Cumulative");
            println!("--------------------------------------------------");
            println!(
                "Blocks count:     {}",
                session.transfer.stats.total_blocks.0,
            );
            println!("Data transferred: {:02} GB", data_total / u_giga,);
            println!("Transfer rate:    {data_total_rate:02} Mbps",);
            println!(
                "Retransmissions:  {} ({:02}%)",
                session.transfer.stats.total_retransmits.0,
                100.0_f64 * total_retransmits_fraction,
            );
            println!("Flags          :  {stats_flags}");
            println!();
            println!("OS UDP rx errors: {}", session.transfer.stats.udp_errors);
        } else {
            // print a header if necessary
            // TODO: Tsunami has a STATS_NOHEADER compile-time constant that is checked here.
            // It might be worth implementing this as a runtime flag
            if *iteration % 23 == 0 {
                println!(
                    "             last_interval                   transfer_total                   buffers      transfer_remaining  OS UDP"
                );
                println!(
                    "time          blk    data       rate rexmit     blk    data       rate rexmit queue  ring     blk   rt_len      err "
                );
            }
            *iteration = iteration.wrapping_add(1);
            print!("{stats_line}");
        }

        // and flush the output
        std::io::stdout().flush()?;
    }

    // print to the transcript if the user wants
    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::data_log(
            session,
            parameter,
            stats_line.as_str(),
        ));
    }

    // reset the statistics for the next interval
    session.transfer.stats.this_blocks = session.transfer.stats.total_blocks;
    session.transfer.stats.this_retransmits = BlockIndex(0);
    session.transfer.stats.this_flow_originals = BlockIndex(0);
    session.transfer.stats.this_flow_retransmitteds = BlockIndex(0);
    session.transfer.stats.this_time = Some(Instant::now());

    Ok(())
}
