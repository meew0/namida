use std::{
    borrow::Cow,
    io::{Seek, SeekFrom},
    net::ToSocketAddrs,
    os::unix::ffi::OsStrExt,
    path::Path,
    time::Instant,
};

use crate::{
    datagram::{self, BlockType},
    message::{
        self, ClientToServer, FileRequest, FileRequestError, ServerToClient, TransmissionControl,
        UdpMethod,
    },
    types::{BlockIndex, FileMetadata, FileSize},
};

use anyhow::{anyhow, bail};

use super::{IndexMode, Parameter, Session, Transfer};

/// Handles the given transmission control request. The actions taken depend on the nature of the
/// request:
///
///  * `Retransmit`: Retransmit the given block.
///  * `RestartAt`: Restart the transfer at the given block.
///  * `SubmitErrorRate`: Use the given error rate to adjust the IPD.
///
/// For `Retransmit` messsages, the given buffer must be large enough to hold `block_size + 6`
/// bytes. For other messages, the datagram parameters are ignored.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if we receive a `Retransmit` request, but no UDP socket is present.
pub fn accept_retransmit(
    session: &mut Session,
    parameter: &Parameter,
    retransmission: &TransmissionControl,
    datagram_block_buffer: &mut [u8],
    datagram_buffer: &mut [u8],
    iteration: &mut u32,
) -> anyhow::Result<()> {
    #[allow(clippy::match_wildcard_for_single_variants)]
    match *retransmission {
        TransmissionControl::SubmitErrorRate(error_rate) => {
            // if it's an error rate notification: calculate a new IPD
            if error_rate > session.properties.error_rate {
                let factor1: f64 = 1.0_f64 * f64::from(session.properties.slower.numerator)
                    / f64::from(session.properties.slower.denominator)
                    - 1.0_f64;
                let factor2: f64 = (1.0_f64 + f64::from(error_rate.0)
                    - f64::from(session.properties.error_rate.0))
                    / (100_000.0_f64 - f64::from(session.properties.error_rate.0));
                session.transfer.ipd_current *= factor1.mul_add(factor2, 1.0_f64);
            } else {
                session.transfer.ipd_current *= f64::from(session.properties.faster.numerator)
                    / f64::from(session.properties.faster.denominator);
            }

            // make sure the IPD is still in range, for later calculations
            session.transfer.ipd_current = session
                .transfer
                .ipd_current
                .clamp(f64::from(session.properties.ipd_time), 10000.0);

            // build the stats string
            let stats_line = format!(
                "{:6} {:3.2}µs {:5}µs {:7} {:6.2} {:3}\n",
                error_rate.0,
                session.transfer.ipd_current,
                session.properties.ipd_time,
                session.transfer.block.0,
                100.0_f64 * f64::from(session.transfer.block.0)
                    / f64::from(session.properties.block_count.0),
                session.session_id,
            );

            // print a status report
            if *iteration % 23 == 0 {
                println!(" erate     ipd  target   block   %done srvNr");
            }
            *iteration = iteration.wrapping_add(1);
            print!("{stats_line}");

            // print to the transcript if the user wants
            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::data_log(
                    session,
                    stats_line.as_str(),
                ));
            }
        }
        TransmissionControl::RestartAt(block) => {
            // if it's a restart request: do range-checking first
            if block.is_zero() || block > session.properties.block_count {
                bail!("Attempt to restart at illegal block {}", block.0);
            }

            session.transfer.block = block;
        }
        TransmissionControl::Retransmit(block) => {
            session.properties.retransmit_phase = true;

            // if it's a retransmit request: build the retransmission
            let datagram = super::io::build_datagram(
                session,
                block,
                BlockType::Retransmission,
                datagram_block_buffer,
            )?;

            // Send the block away
            send_datagram(session, parameter, datagram, datagram_buffer)?;
        }
        TransmissionControl::RetransmitOver(_) => {
            session.properties.retransmit_phase = false;
        }
        _ => {
            // if it's another kind of request
            bail!(
                "Received unknown retransmission request: {:?}",
                retransmission
            );
        }
    }

    // we're done
    Ok(())
}

/// Send the given `datagram` view as a UDP packet. The `datagram_buffer` is used as an intermediate
/// and must be `BLOCK_SIZE + 6` bytes long if unencrypted or `BLOCK_SIZE + 30` bytes if encrypted.
///
/// # Errors
/// Returns an error on encoding, encryption, or I/O failure.
///
/// # Panics
/// Panics if no UDP socket is available.
pub fn send_datagram(
    session: &mut Session,
    parameter: &Parameter,
    datagram: datagram::View,
    datagram_buffer: &mut [u8],
) -> anyhow::Result<()> {
    if parameter.encrypted {
        let nonce = session.client.nonce();

        // Write the nonce into the first 8 bytes...
        bincode::encode_into_slice(
            nonce,
            &mut datagram_buffer[..8],
            crate::common::BINCODE_CONFIG,
        )?;

        // ...and the actual datagram into the rest
        let message_buffer = &mut datagram_buffer[8..];
        let message = session
            .client
            .encode_encrypt(message_buffer, nonce, datagram)?;
        assert_eq!(message.len(), message_buffer.len());
    } else {
        bincode::encode_into_slice(datagram, datagram_buffer, crate::common::BINCODE_CONFIG)?;
    }

    // try to send out the block
    session
        .transfer
        .udp_socket
        .as_ref()
        .expect("an UDP socket should have been opened")
        .send_to(
            datagram_buffer,
            session
                .transfer
                .udp_address
                .expect("an UDP address should have been set"),
        )?;

    Ok(())
}

/// Given an active session, returns `()` if we are able to negotiate authentication successfully
/// and an error otherwise. Used only for unencrypted connections.
///
/// The negotiation process works like this:
///
///  1. The server [this process] sends 512 bits of random data to the client.
///  2. The client XORs 512 bits of the shared secret onto this random data and responds with the
///     MD5 hash of the result.
///  3. The server does the same thing and compares the result. If the authentication succeeds, the
///     server transmits an `AuthenticationStatus(true)`. Otherwise, it transmits an
///     `AuthenticationStatus(false)`.
///
/// # Errors
/// Returns an error on I/O failure, when the client sends unexpected messages, or when
/// authentication is unsuccessful.
pub fn authenticate_unencrypted(session: &mut Session, secret: &[u8]) -> anyhow::Result<()> {
    use rand::Rng;

    // obtain the random data
    let mut random: [u8; 64] = [0; 64];
    rand::thread_rng().fill(&mut random);

    // send the random data to the client
    session
        .client
        .write(ServerToClient::AuthenticationChallenge(random))?;

    // calculate our own version of the digest
    let server_digest: [u8; 16] = crate::common::prepare_proof(&mut random, secret).into();

    // read the results back from the client
    let ClientToServer::AuthenticationResponse(client_digest) = session.client.read()? else {
        bail!("Expected authentication response");
    };

    // compare the two digests
    if server_digest != client_digest {
        session
            .client
            .write(ServerToClient::AuthenticationStatus(false))?;
        bail!("Authentication failed");
    }

    // try to tell the client it worked
    session
        .client
        .write(ServerToClient::AuthenticationStatus(true))?;

    // we succeeded
    Ok(())
}

/// Authenticates a client and initiates an encrypted connection with it using the Noise protocol.
///
/// # Errors
/// Returns an error on I/O failure, authentication failure, or if the Noise handshake was
/// unsuccessful.
pub fn authenticate_encrypted(session: &mut Session, secret: &[u8]) -> anyhow::Result<()> {
    let mut noise_init_buffer = [0_u8; 1024];

    let builder = snow::Builder::new(crate::common::NOISE_PATTERN.parse()?);
    let static_key = builder.generate_keypair()?.private;
    let mut noise = builder
        .local_private_key(&static_key)
        .psk(3, secret)
        .build_responder()?;

    // <- e
    let message::Noise(data) = session.client.read()?;
    noise.read_message(&data, &mut noise_init_buffer)?;

    // -> e, ee, s, es
    let len = noise.write_message(&[], &mut noise_init_buffer)?;
    session
        .client
        .write(message::Noise(Cow::from(&noise_init_buffer[..len])))?;

    // <- s, se
    let message::Noise(data) = session.client.read()?;
    noise.read_message(&data, &mut noise_init_buffer)?;

    let noise = noise.into_stateless_transport_mode()?;
    session.client.set_noise_state(noise);

    println!("Encrypted session established.");

    Ok(())
}

/// Negotiates the protocol version used between the server and the client. Needs to match exactly
/// for a connection to be initiated. This is the only part in the code where we send raw bytes
/// instead of message structs, to ensure that old Tsunami clients are appropriately rejected.
///
/// # Errors
/// Returns an error on I/O failure, or when negotiation was unsuccessful.
pub fn negotiate(session: &mut Session, parameter: &Parameter) -> anyhow::Result<()> {
    let server_revision = crate::version::magic(parameter.encrypted);

    // send our protocol revision number to the client
    session.client.write(server_revision)?;

    // read the protocol revision number from the client
    let client_revision: u32 = session.client.read()?;

    // compare the numbers
    if client_revision != server_revision {
        bail!("Protocol negotiation failed");
    }

    Ok(())
}

/// Determine the address to which we should send UDP data. This can be done using one of two
/// methods, depending on the client's choice:
///
///  * `StaticPort`, where we send it UDP data on the same address as the TCP one, but with a
///    client-specified UDP port number, or
///  * `Discovery`, where the client sends us UDP data and we send our data back where that came
///    from.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if discovery has been specified, but no UDP socket is available to send data from.
pub fn determine_client_udp_address(
    session: &mut Session,
    parameter: &Parameter,
    method: UdpMethod,
) -> anyhow::Result<()> {
    let address = match &parameter.client {
        Some(parameter_client) => {
            // If a client address has been specified in the command line, we don't care about what
            // the TCP client told us, we will send the data there.
            parameter_client.to_socket_addrs()?.next().ok_or_else(|| {
                anyhow!(
                    "Could not resolve specified client address: {}",
                    parameter_client
                )
            })?
        }
        None => {
            // If no dedicated client parameter is set, determine the address based on the desired
            // method
            match method {
                UdpMethod::StaticPort(static_port) => {
                    // Get the client's TCP address, overwrite the port, and use that as the target
                    // UDP address
                    let mut tcp_peer_addr = session.client.socket.peer_addr()?;
                    tcp_peer_addr.set_port(static_port);
                    tcp_peer_addr
                }
                UdpMethod::Discovery => {
                    // Listen on our socket until we receive a `namida` message.
                    let mut buffer = [0_u8; 6];
                    let udp_socket = session.transfer.udp_socket.as_mut().expect("A UDP socket should have been set before calling `determine_client_udp_address`");
                    let client_udp_address = loop {
                        let (len, address) = udp_socket.recv_from(&mut buffer)?;
                        if len == 6 && &buffer == b"namida" {
                            break address;
                        }
                    };

                    // Tell the client that it worked
                    session.client.write(ServerToClient::UdpDone)?;

                    client_udp_address
                }
            }
        }
    };

    // print out the client address and port number
    if parameter.verbose_yn {
        println!("Sending to client {address}");
    }

    // we succeeded
    session.transfer.udp_address = Some(address);
    Ok(())
}

/// Sends a list of available files to the client.
///
/// # Errors
/// Returns an error on I/O failure.
pub fn send_file_list(
    session: &mut Session,
    parameter: &Parameter,
    files: &mut Vec<FileMetadata>,
) -> anyhow::Result<()> {
    // The list of files on the system might have changed since the server has started. However,
    // reindexing is expensive, so we only want to do it if the user actually desires this
    // behaviour.
    if matches!(parameter.index, IndexMode::Always) {
        files.clear();
        super::io::index_files(&parameter.file_names, files);
        #[allow(clippy::min_ident_chars)]
        let s = if files.len() == 1 { "" } else { "s" };
        eprintln!("Found {} file{s} after reindexing.", files.len());
    }

    session
        .client
        .write(ServerToClient::FileCount(files.len() as u64))?;

    for file_metadata in files {
        session
            .client
            .write(ServerToClient::FileListEntry(file_metadata.clone()))?;
    }

    Ok(())
}

/// Reads a file request from the client — either a request to have an individual file sent,
/// or a file listing request. On success, returns true if the client should continue to send
/// an individual file request, or false if the client may send any request afterwards.
///
/// # Errors
/// Returns an error on I/O failure, or when the client sends an unexpected message.
///
/// # Panics
/// Panics on file or block count overflow.
pub fn open_transfer(
    session: &mut Session,
    parameter: &Parameter,
    request: FileRequest,
) -> anyhow::Result<()> {
    session.transfer = Transfer::default();

    let FileRequest {
        path,
        target_rate,
        error_rate,
        slowdown,
        speedup,
    } = request;

    // store the filename in the transfer object
    let requested_path = session.transfer.filename.insert(path);

    // make a note of the request
    if parameter.verbose_yn {
        println!("Request for file: '{}'", requested_path.display());
    };

    // Check if the file is within one of the served paths, to prevent the client from retrieving
    // files it is not supposed to (files outside of explicitly specified paths, or
    // `namida get ../../../etc/passwd`-style path traversal attacks in case no explicit paths were
    // specified)
    if !file_accessible(parameter, requested_path) {
        session.client.write(ServerToClient::FileRequestError(
            FileRequestError::Nonexistent,
        ))?;
        bail!(
            "Requested path '{}' is outside the served directories",
            requested_path.display()
        );
    }

    // try to open the file for reading
    let file = match std::fs::File::open(&requested_path) {
        Ok(opened_file) => session.transfer.file.insert(opened_file),
        Err(err) => {
            session.client.write(ServerToClient::FileRequestError(
                FileRequestError::Nonexistent,
            ))?;
            bail!(
                "File '{}' does not exist or cannot be read: {}",
                requested_path.display(),
                err
            );
        }
    };

    // store other requested property values
    session.properties.target_rate = target_rate;
    session.properties.error_rate = error_rate;
    session.properties.slower = slowdown;
    session.properties.faster = speedup;

    // determine the file size, and calculate the number of blocks based on that
    session.properties.file_size = FileSize(file.seek(SeekFrom::End(0))?);
    file.seek(SeekFrom::Start(0))?;

    let mut block_count_base = session
        .properties
        .file_size
        .0
        .checked_div(u64::from(crate::common::BLOCK_SIZE))
        .expect("block size is zero");
    let tail_size = session
        .properties
        .file_size
        .0
        .checked_rem(u64::from(crate::common::BLOCK_SIZE))
        .expect("block size is zero");

    if tail_size != 0 {
        block_count_base = block_count_base
            .checked_add(1)
            .expect("block count overflow");
    }

    session.properties.block_count =
        BlockIndex(block_count_base.try_into().expect("block count overflow"));
    session.properties.epoch = crate::common::epoch();

    // open a UDP socket now, so we have a port number that the client can try to connect to
    let udp_socket = session
        .transfer
        .udp_socket
        .insert(super::network::create_udp_socket(parameter)?);

    // signal success to the client and send it the required metadata fields
    session.client.write(ServerToClient::FileRequestSuccess {
        file_size: session.properties.file_size,
        block_count: session.properties.block_count,
        epoch: session.properties.epoch,
        udp_port: udp_socket.local_addr()?.port(),
    })?;

    Ok(())
}

// Checks whether the given file should be accessible to the client, i.e. whether it is located
// within one of the served paths (or is itself one of the served paths)
fn file_accessible(parameter: &Parameter, file: &Path) -> bool {
    let Ok(canonical) = file.canonicalize() else {
        eprintln!("Could not canonicalise requested path {}", file.display());
        return false;
    };

    for base in &parameter.file_names {
        let Ok(base_canonical) = base.canonicalize() else {
            eprintln!("WARNING: Could not canonicalise served path '{}'. Files in it will not be accessible to the client.", base.display());
            continue;
        };

        // Check whether `base_canonical` is a prefix of `canonical`
        if canonical
            .as_os_str()
            .as_bytes()
            .starts_with(base_canonical.as_os_str().as_bytes())
        {
            return true;
        }
    }

    false
}

/// Send the client a list of checksums of chunks within the current file. Then, wait for the
/// client to let us know which of the chunks it already has. We can then skip transmitting these
/// blocks.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no file has been opened.
pub fn resume(session: &mut Session) -> anyhow::Result<()> {
    let file = session
        .transfer
        .file
        .as_mut()
        .expect("File should have been opened");

    let chunk_blocks = crate::common::chunk_blocks(session.properties.file_size);
    let checksums = crate::common::calculate_checksums(
        file,
        session.properties.file_size,
        session.properties.block_count,
        chunk_blocks,
    )?;
    session.client.write(ServerToClient::Checksums(checksums))?;

    let ClientToServer::SkipChunks(skip_chunks) = session.client.read()? else {
        bail!("Expected `SkipChunks`");
    };
    session.transfer.skip_chunks = Some(skip_chunks);

    Ok(())
}

/// Takes the given ping `Instant`s and uses them to calculate an initial inter-packet delay, which
/// will be set for the session.
///
/// # Panics
/// Panics on arithmetic overflow.
pub fn start_transfer_timing(
    session: &mut Session,
    parameter: &Parameter,
    ping_start: Instant,
    ping_end: Instant,
) {
    // calculate and convert RTT to microseconds...
    session.properties.wait_µs = ping_end
        .duration_since(ping_start)
        .as_micros()
        .try_into()
        .expect("RTT microseconds conversion overflow");

    // ...add a 10% safety margin...
    #[allow(clippy::cast_precision_loss)]
    #[allow(clippy::cast_possible_truncation)]
    let safety_margin = (session.properties.wait_µs as f64 * 0.1_f64) as i64;
    session.properties.wait_µs = session
        .properties
        .wait_µs
        .checked_add(safety_margin)
        .expect("RTT safety margin overflow");

    // ...and store the inter-packet delay
    session.properties.ipd_time = (u64::from(crate::common::BLOCK_SIZE)
        .checked_mul(8_000_000_u64)
        .expect("IPD time calculation overflow (1)")
        .checked_div(session.properties.target_rate.0)
        .expect("target rate is zero"))
    .try_into()
    .expect("IPD time calculation overflow (2)");
    session.transfer.ipd_current = f64::from(session.properties.ipd_time) * 3.0_f64;

    // if we're doing a transcript
    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::open(session, parameter));
    }
}
