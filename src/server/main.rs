use std::{
    cmp::Ordering,
    io::{ErrorKind, Read},
    net::ToSocketAddrs,
    time::Instant,
};

use super::{Parameter, Session, Transfer};

use crate::{
    datagram::BlockType,
    extc,
    message::TransmissionControl,
    server::Properties,
    types::{BlockIndex, ErrorRate, FileMetadata, FileSize},
};
use ::libc;
use anyhow::bail;

pub fn serve(mut parameter: Parameter) -> anyhow::Result<()> {
    process_options(&mut parameter);

    let listener = super::network::create_tcp_socket_server(&parameter).unwrap();
    eprintln!(
        "Tsunami Server for protocol rev {:X}\nRevision: {}\nCompiled: {} {}\nWaiting for clients to connect.",
        crate::common::PROTOCOL_REVISION,
        crate::common::NAMIDA_VERSION,
        crate::COMPILE_DATE,
        crate::COMPILE_TIME
    );

    // “while our little world keeps turning”...
    for (session_id, result) in listener.incoming().enumerate() {
        let client = result?;
        eprintln!("New client connecting from {}...", client.peer_addr()?);

        let parameter_cloned = parameter.clone();

        std::thread::spawn(move || {
            let session = Session {
                transfer: Transfer::default(),
                properties: Properties::default(),
                client,
                session_id,
            };

            let result =
                std::panic::catch_unwind(move || client_handler(session, parameter_cloned));

            match result {
                Ok(Ok(())) => eprintln!("Child server thread terminated succcessfully."),
                Ok(Err(err)) => eprintln!("Child server thread terminated with error: {}", err),
                Err(_panic) => eprintln!("Child server thread panicked"),
            };
        });
    }

    Ok(())
}

pub fn client_handler(mut session: Session, parameter: Parameter) -> anyhow::Result<()> {
    let mut datagram_block_buffer: Vec<u8> = vec![0_u8; session.properties.block_size.0 as usize];
    let mut datagram_buffer: Vec<u8> = vec![0_u8; session.properties.block_size.0 as usize + 6];

    let mut delta: u64 = 0;

    super::protocol::ttp_negotiate_server(&mut session)?;
    super::protocol::ttp_authenticate_server(&mut session, parameter.secret.as_bytes())?;

    // We call it negotiation, but we unilaterally impose our parameters on the client!
    if parameter.verbose_yn {
        println!("Client authenticated. Negotiated parameters are:");
        println!("Block size: {}", session.properties.block_size);
        println!("Buffer size: {}", parameter.udp_buffer);
        println!(
            "Port: {}",
            parameter
                .bind
                .to_socket_addrs()
                .unwrap()
                .next()
                .unwrap()
                .port()
        );
    }

    let mut control_slice = [0_u8; 8];
    let mut control_slice_cursor = 0_usize;
    let mut retransmit_accept_iteration = 0;

    loop {
        // Make the client socket blocking (for the case that it has been set to non-blocking
        // in a previous loop iteration)
        session.client.set_nonblocking(false)?;

        match super::protocol::ttp_open_transfer_server(&mut session, &parameter) {
            Ok(true) => {}
            Ok(false) => {
                // No error was encountered, but we should not try to continue with file
                // transmission now
                continue;
            }
            Err(err) => {
                println!("WARNING: Invalid file request, error: {:?}", err);
                bail!("Closing connection to client.");
            }
        }

        if let Err(err) = unsafe { super::protocol::ttp_open_port_server(&mut session, &parameter) }
        {
            println!("WARNING: UDP socket creation failed: {:?}", err);
            continue;
        }

        // Make the client socket non-blocking, to be able to skip reading a transmission control
        // request if none has been sent.
        session.client.set_nonblocking(true)?;

        let start = Instant::now();
        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::xscript_data_start_server(
                &mut session,
            ));
        }
        let mut lasthblostreport = start;
        let mut lastfeedback = start;
        let mut prevpacketT = start;
        let mut deadconnection_counter = 0;
        let mut ipd_time = 0;
        let mut ipd_time_max = 0;
        session.transfer.block = BlockIndex(0);

        while session.transfer.block <= session.properties.block_count {
            let mut block_type = BlockType::Retransmission;
            let currpacketT = Instant::now();

            let micros_diff = (currpacketT - prevpacketT).as_micros() as f64;
            let ipd_usleep_diff = (session.transfer.ipd_current - micros_diff) as i64;
            prevpacketT = currpacketT;

            if ipd_usleep_diff > 0 || ipd_time > 0 {
                ipd_time += ipd_usleep_diff;
            }
            ipd_time_max = if ipd_time > ipd_time_max {
                ipd_time
            } else {
                ipd_time_max
            };

            let read_result = session
                .client
                .read(&mut control_slice[control_slice_cursor..]);

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

            control_slice_cursor += read_count;

            match control_slice_cursor.cmp(&TransmissionControl::SIZE) {
                Ordering::Equal => {
                    let retransmission =
                        bincode::decode_from_slice(&control_slice, crate::common::BINCODE_CONFIG)?
                            .0;

                    lastfeedback = currpacketT;
                    lasthblostreport = currpacketT;
                    deadconnection_counter = 0;

                    if let TransmissionControl::EndTransmission(_) = retransmission {
                        let filename = session.transfer.filename.as_ref().unwrap();
                        eprintln!("Transmission of {} complete.", filename.display());

                        if let Some(finishhook) = &parameter.finishhook {
                            eprintln!("Executing: {} {}", finishhook.display(), filename.display());

                            let spawned =
                                std::process::Command::new(finishhook).arg(filename).spawn();

                            if let Err(err) = spawned {
                                eprintln!("Could not execute finish hook: {}", err);
                            }
                        }
                        break;
                    } else {
                        if let Err(err) = super::protocol::ttp_accept_retransmit(
                            &mut session,
                            &parameter,
                            retransmission,
                            datagram_block_buffer.as_mut_slice(),
                            datagram_buffer.as_mut_slice(),
                            &mut retransmit_accept_iteration,
                        ) {
                            println!("WARNING: Retransmission error: {:?}", err);
                        }
                        control_slice_cursor = 0;
                    }
                }
                Ordering::Less => {
                    let incremented = session.transfer.block + BlockIndex(1);
                    session.transfer.block = if incremented < session.properties.block_count {
                        incremented
                    } else {
                        session.properties.block_count
                    };

                    block_type = if session.transfer.block == session.properties.block_count {
                        BlockType::Final
                    } else {
                        BlockType::Normal
                    };

                    let block_index = session.transfer.block;
                    let datagram = super::io::build_datagram(
                        &mut session,
                        block_index,
                        block_type,
                        datagram_block_buffer.as_mut_slice(),
                    )
                    .unwrap();
                    datagram.write_to(datagram_buffer.as_mut_slice());

                    unsafe {
                        let status = extc::sendto(
                            session.transfer.udp_fd,
                            datagram_buffer.as_mut_ptr() as *const libc::c_void,
                            (6 as libc::c_int as u32).wrapping_add(session.properties.block_size.0)
                                as u64,
                            0 as libc::c_int,
                            extc::__CONST_SOCKADDR_ARG {
                                __sockaddr__: session.transfer.udp_address,
                            },
                            session.transfer.udp_length,
                        ) as libc::c_int;
                        if status < 0 as libc::c_int {
                            println!(
                                "WARNING: Could not transmit block #{}",
                                session.transfer.block.0
                            );
                            continue;
                        }
                    }
                }
                Ordering::Greater => {
                    eprintln!("warn: retransmitlen > {}", TransmissionControl::SIZE);
                    control_slice_cursor = 0;
                }
            }

            let old_deadconnection_counter = deadconnection_counter;
            deadconnection_counter += 1;

            if old_deadconnection_counter > 2048 {
                deadconnection_counter = 0;
                if crate::common::get_usec_since(lasthblostreport) < 500000 {
                    continue;
                }

                lasthblostreport = Instant::now();

                let retransmission = TransmissionControl::SubmitErrorRate(ErrorRate(100000));
                if let Err(err) = super::protocol::ttp_accept_retransmit(
                    &mut session,
                    &parameter,
                    retransmission,
                    datagram_block_buffer.as_mut_slice(),
                    datagram_buffer.as_mut_slice(),
                    &mut retransmit_accept_iteration,
                ) {
                    println!("Error in accept_retransmit: {:?}", err);
                }

                delta = crate::common::get_usec_since(lastfeedback);

                let stats_line = format!(
                    "   n/a     n/a     n/a {:7} {:6.2} {:3} -- no heartbeat since {:3.2}s\n",
                    session.transfer.block.0,
                    100.0f64 * session.transfer.block.0 as f64
                        / session.properties.block_count.0 as f64,
                    session.session_id,
                    1e-6f64 * delta as f64,
                );

                if parameter.transcript_yn {
                    crate::common::transcript_warn_error(
                        super::transcript::xscript_data_log_server(&mut session, &stats_line),
                    );
                }

                eprint!("{}", stats_line);

                if 1e-6f64 * delta as f64 > parameter.hb_timeout as f64 {
                    eprintln!(
                        "Heartbeat timeout of {} seconds reached, terminating transfer.",
                        parameter.hb_timeout,
                    );
                    break;
                }
            }

            if matches!(block_type, BlockType::Final) {
                crate::common::usleep_that_works((10 * ipd_time_max) as u64);
            }

            if ipd_time > 0 {
                crate::common::usleep_that_works(ipd_time as u64);
            }
        }

        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::xscript_data_stop_server(
                &mut session,
            ));
        }

        let stop = Instant::now();
        delta = (stop - start).as_micros().try_into().unwrap();

        if parameter.verbose_yn {
            eprintln!(
                "Server {} transferred {} bytes in {:0>.2} seconds ({:0>.1} Mbps)",
                session.session_id,
                session.properties.file_size.0,
                delta as f64 / 1000000.0f64,
                8.0f64 * session.properties.file_size.0 as f64 / delta as f64,
            );
        }

        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::xscript_close_server(
                &mut session,
                delta,
            ));
        }

        unsafe {
            extc::close(session.transfer.udp_fd);
        }
    }
}

pub fn process_options(parameter: &mut Parameter) {
    if !parameter.file_names.is_empty() {
        let total_files = parameter.file_names.len();
        eprintln!(
            "\nThe specified {} files will be listed on GET *:",
            total_files
        );

        for (counter, path) in parameter.file_names.iter().enumerate() {
            match std::fs::metadata(path) {
                Ok(metadata) => {
                    parameter.files.push(FileMetadata {
                        path: path.clone(),
                        size: FileSize(metadata.len()),
                    });
                    eprintln!(
                        " {:3}   {:<20}  {} bytes\n",
                        counter + 1,
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

    if parameter.verbose_yn {
        eprintln!("Buffer size: {}", parameter.udp_buffer);
        eprintln!("Bind: {}", parameter.bind);
    }
}

pub unsafe extern "C" fn reap(mut _signum: libc::c_int) {
    let mut status: libc::c_int = 0;
    while extc::waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int) > 0 as libc::c_int {
        extc::fprintf(
            extc::stderr,
            b"Child server process terminated with status code 0x%X\n\0" as *const u8
                as *const libc::c_char,
            status,
        );
    }
    extc::signal(
        17 as libc::c_int,
        Some(reap as unsafe extern "C" fn(libc::c_int) -> ()),
    );
}
