use std::{
    cmp::Ordering,
    ffi::{CStr, CString},
};

use super::{Parameter, Session, Transfer};

use crate::{extc, types::Retransmission};
use ::libc;

pub unsafe fn serve(mut parameter: Parameter) -> libc::c_int {
    let mut server_fd: libc::c_int = 0;
    let mut client_fd: libc::c_int = 0;
    let mut remote_address: extc::sockaddr_in = extc::sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: extc::in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut remote_length: extc::socklen_t =
        ::core::mem::size_of::<extc::sockaddr_in>() as libc::c_ulong as extc::socklen_t;
    let mut session: Session = Session {
        transfer: Transfer::default(),
        client_fd: 0,
        session_id: 0,
    };
    let mut child_pid: extc::pid_t = 0;
    process_options(&mut parameter);
    server_fd = super::network::create_tcp_socket_server(&parameter).unwrap();
    extc::signal(
        17 as libc::c_int,
        Some(reap as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    extc::fprintf(
        extc::stderr,
        b"Tsunami Server for protocol rev %X\nRevision: %s\nCompiled: %s %s\nWaiting for clients to connect.\n\0"
            as *const u8 as *const libc::c_char,
        crate::common::PROTOCOL_REVISION,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
        b"Nov 16 2023\0" as *const u8 as *const libc::c_char,
        b"21:24:19\0" as *const u8 as *const libc::c_char,
    );
    loop {
        client_fd = extc::accept(
            server_fd,
            extc::__SOCKADDR_ARG {
                __sockaddr__: &mut remote_address as *mut extc::sockaddr_in as *mut extc::sockaddr,
            },
            &mut remote_length,
        );
        if client_fd < 0 as libc::c_int {
            println!("WARNING: Could not accept client connection");
        } else {
            extc::fprintf(
                extc::stderr,
                b"New client connecting from %s...\n\0" as *const u8 as *const libc::c_char,
                extc::inet_ntoa(remote_address.sin_addr),
            );
            child_pid = extc::fork();
            if child_pid < 0 as libc::c_int {
                println!("WARNING: Could not create child process");
            } else {
                session.session_id += 1;
                if child_pid == 0 as libc::c_int {
                    extc::close(server_fd);
                    session.client_fd = client_fd;
                    session.transfer = Transfer::default();
                    session.transfer.ipd_current = 0.0f64;
                    client_handler(&mut session, &mut parameter);
                    return 0 as libc::c_int;
                } else {
                    extc::close(client_fd);
                }
            }
        }
    }
}

pub unsafe fn client_handler(session: &mut Session, parameter: &mut Parameter) {
    let mut retransmission: Retransmission = Retransmission {
        block: 0,
        error_rate: 0,
    };
    let mut start: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut stop: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut prevpacketT: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut currpacketT: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut lastfeedback: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut lasthblostreport: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut deadconnection_counter: u32 = 0;
    let mut retransmitlen: libc::c_int = 0;

    let mut datagram_block_buffer: Vec<u8> = vec![0_u8; parameter.block_size as usize];
    let mut datagram_buffer: Vec<u8> = vec![0_u8; parameter.block_size as usize + 6];

    let mut ipd_time: i64 = 0;
    let mut ipd_usleep_diff: i64 = 0;
    let mut ipd_time_max: i64 = 0;
    let mut status: libc::c_int = 0;
    let mut delta: u64 = 0;
    let mut block_type: u8 = 0;
    super::protocol::ttp_negotiate_server(session).unwrap();
    super::protocol::ttp_authenticate_server(session, parameter.secret.as_bytes()).unwrap();
    if 1 as libc::c_int == parameter.verbose_yn as libc::c_int {
        extc::fprintf(
            extc::stderr,
            b"Client authenticated. Negotiated parameters are:\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::fprintf(
            extc::stderr,
            b"Block size: %d\n\0" as *const u8 as *const libc::c_char,
            parameter.block_size,
        );
        extc::fprintf(
            extc::stderr,
            b"Buffer size: %d\n\0" as *const u8 as *const libc::c_char,
            parameter.udp_buffer,
        );
        extc::fprintf(
            extc::stderr,
            b"Port: %d\n\0" as *const u8 as *const libc::c_char,
            parameter.tcp_port as libc::c_int,
        );
    }
    loop {
        status = extc::fcntl(session.client_fd, 4 as libc::c_int, 0 as libc::c_int);
        if status < 0 as libc::c_int {
            panic!("Could not make client socket blocking");
        }
        if let Err(err) = super::protocol::ttp_open_transfer_server(session, parameter) {
            println!("WARNING: Invalid file request, error: {:?}", err);
        } else if let Err(err) = super::protocol::ttp_open_port_server(session, parameter) {
            println!("WARNING: UDP socket creation failed: {:?}", err);
        } else {
            status = extc::fcntl(session.client_fd, 4 as libc::c_int, 0o4000 as libc::c_int);
            if status < 0 as libc::c_int {
                panic!("Could not make client socket non-blocking");
            }
            extc::gettimeofday(&mut start, std::ptr::null_mut::<libc::c_void>());
            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::xscript_data_start_server(
                    session, start,
                ));
            }
            lasthblostreport = start;
            lastfeedback = start;
            prevpacketT = start;
            deadconnection_counter = 0 as libc::c_int as u32;
            ipd_time = 0 as libc::c_int as i64;
            ipd_time_max = 0 as libc::c_int as i64;
            ipd_usleep_diff = 0 as libc::c_int as i64;
            retransmitlen = 0 as libc::c_int;
            session.transfer.block = 0 as libc::c_int as u32;
            while session.transfer.block <= parameter.block_count {
                block_type = 'R' as i32 as u8;
                extc::gettimeofday(&mut currpacketT, std::ptr::null_mut::<libc::c_void>());
                ipd_usleep_diff = (session.transfer.ipd_current
                    + ((prevpacketT.tv_sec - currpacketT.tv_sec) as libc::c_double * 1e6f64
                        + (prevpacketT.tv_usec - currpacketT.tv_usec) as libc::c_double))
                    as i64;
                prevpacketT = currpacketT;
                if ipd_usleep_diff > 0 as libc::c_int as i64 || ipd_time > 0 as libc::c_int as i64 {
                    ipd_time += ipd_usleep_diff;
                }
                ipd_time_max = if ipd_time > ipd_time_max {
                    ipd_time
                } else {
                    ipd_time_max
                };
                status = extc::read(
                    session.client_fd,
                    (&mut retransmission as *mut Retransmission as *mut libc::c_char)
                        .offset(retransmitlen as isize) as *mut libc::c_void,
                    ::core::mem::size_of::<Retransmission>().wrapping_sub(retransmitlen as usize)
                        as u64,
                ) as libc::c_int;
                if status <= 0 as libc::c_int && *extc::__errno_location() != 11 as libc::c_int {
                    panic!("Retransmission read failed");
                }
                if status > 0 as libc::c_int {
                    retransmitlen += status;
                }

                match (retransmitlen as libc::c_ulong)
                    .cmp(&(::core::mem::size_of::<Retransmission>() as libc::c_ulong))
                {
                    Ordering::Equal => {
                        lastfeedback = currpacketT;
                        lasthblostreport = currpacketT;
                        deadconnection_counter = 0 as libc::c_int as u32;
                        if extc::__bswap_16(retransmission.request_type) as libc::c_int
                            == crate::common::REQUEST_STOP as libc::c_int
                        {
                            eprintln!(
                                "Transmission of {} complete.",
                                session.transfer.filename.as_ref().unwrap()
                            );
                            if let Some(finishhook) = &parameter.finishhook {
                                let MaxCommandLength: libc::c_int = 1024 as libc::c_int;
                                let vla = MaxCommandLength as usize;
                                let mut cmd: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
                                let mut v: libc::c_int = 0;

                                let finishhook_c = CString::new(finishhook.as_str()).unwrap();
                                let filename_c = CString::new(
                                    session.transfer.filename.as_ref().unwrap().as_str(),
                                )
                                .unwrap();

                                v = extc::snprintf(
                                    cmd.as_mut_ptr(),
                                    MaxCommandLength as libc::c_ulong,
                                    b"%s %s\0" as *const u8 as *const libc::c_char,
                                    finishhook_c.as_ptr(),
                                    filename_c.as_ptr(),
                                );
                                if v >= MaxCommandLength {
                                    extc::fprintf(
                                        extc::stderr,
                                        b"Error: command buffer too short\n\0" as *const u8
                                            as *const libc::c_char,
                                    );
                                } else {
                                    extc::fprintf(
                                        extc::stderr,
                                        b"Executing: %s\n\0" as *const u8 as *const libc::c_char,
                                        cmd.as_mut_ptr(),
                                    );
                                    extc::system(cmd.as_mut_ptr());
                                }
                            }
                            break;
                        } else {
                            if let Err(err) = super::protocol::ttp_accept_retransmit(
                                session,
                                parameter,
                                &mut retransmission,
                                datagram_block_buffer.as_mut_slice(),
                                datagram_buffer.as_mut_slice(),
                            ) {
                                println!("WARNING: Retransmission error: {:?}", err);
                            }
                            retransmitlen = 0 as libc::c_int;
                        }
                    }
                    Ordering::Less => {
                        session.transfer.block = if (session.transfer.block)
                            .wrapping_add(1 as libc::c_int as u32)
                            < parameter.block_count
                        {
                            (session.transfer.block).wrapping_add(1 as libc::c_int as u32)
                        } else {
                            parameter.block_count
                        };
                        block_type = (if session.transfer.block == parameter.block_count {
                            'X' as i32
                        } else {
                            'O' as i32
                        }) as u8;
                        let datagram = super::io::build_datagram(
                            session,
                            parameter,
                            session.transfer.block,
                            block_type as u16,
                            datagram_block_buffer.as_mut_slice(),
                        )
                        .unwrap();
                        datagram.write_to(datagram_buffer.as_mut_slice());

                        status = extc::sendto(
                            session.transfer.udp_fd,
                            datagram_buffer.as_mut_ptr() as *const libc::c_void,
                            (6 as libc::c_int as u32).wrapping_add(parameter.block_size) as u64,
                            0 as libc::c_int,
                            extc::__CONST_SOCKADDR_ARG {
                                __sockaddr__: session.transfer.udp_address,
                            },
                            session.transfer.udp_length,
                        ) as libc::c_int;
                        if status < 0 as libc::c_int {
                            println!(
                                "WARNING: Could not transmit block #{}",
                                session.transfer.block
                            );
                            continue;
                        }
                    }
                    Ordering::Greater => {
                        extc::fprintf(
                            extc::stderr,
                            b"warn: retransmitlen > %d\n\0" as *const u8 as *const libc::c_char,
                            ::core::mem::size_of::<Retransmission>() as libc::c_ulong
                                as libc::c_int,
                        );
                        retransmitlen = 0 as libc::c_int;
                    }
                }
                let fresh0 = deadconnection_counter;
                deadconnection_counter = deadconnection_counter.wrapping_add(1);
                if fresh0 > 2048 as libc::c_int as u32 {
                    let mut stats_line: [libc::c_char; 160] = [0; 160];
                    deadconnection_counter = 0 as libc::c_int as u32;
                    if (crate::common::get_usec_since(&mut lasthblostreport) as libc::c_double)
                        < 500000.0f64
                    {
                        continue;
                    }
                    extc::gettimeofday(&mut lasthblostreport, std::ptr::null_mut::<libc::c_void>());
                    retransmission.request_type =
                        extc::__bswap_16(crate::common::REQUEST_ERROR_RATE);
                    retransmission.error_rate = extc::__bswap_32(100000 as libc::c_int as u32);
                    retransmission.block = 0 as libc::c_int as u32;
                    if let Err(err) = super::protocol::ttp_accept_retransmit(
                        session,
                        parameter,
                        &mut retransmission,
                        datagram_block_buffer.as_mut_slice(),
                        datagram_buffer.as_mut_slice(),
                    ) {
                        println!("Error in accept_retransmit: {:?}", err);
                    }
                    delta = crate::common::get_usec_since(&mut lastfeedback);
                    extc::snprintf(
                        stats_line.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 160]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"   n/a     n/a     n/a %7u %6.2f %3u -- no heartbeat since %3.2fs\n\0"
                            as *const u8 as *const libc::c_char,
                        session.transfer.block,
                        100.0f64 * session.transfer.block as libc::c_double
                            / parameter.block_count as libc::c_double,
                        session.session_id,
                        1e-6f64 * delta as libc::c_double,
                    );
                    if parameter.transcript_yn {
                        crate::common::transcript_warn_error(
                            super::transcript::xscript_data_log_server(
                                session,
                                CStr::from_ptr(stats_line.as_mut_ptr()).to_str().unwrap(),
                            ),
                        );
                    }
                    extc::fprintf(
                        extc::stderr,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        stats_line.as_mut_ptr(),
                    );
                    if 1e-6f64 * delta as libc::c_double
                        > parameter.hb_timeout as libc::c_int as libc::c_double
                    {
                        extc::fprintf(
                            extc::stderr,
                            b"Heartbeat timeout of %d seconds reached, terminating transfer.\n\0"
                                as *const u8 as *const libc::c_char,
                            parameter.hb_timeout as libc::c_int,
                        );
                        break;
                    }
                }
                if block_type as libc::c_int == 'X' as i32 {
                    crate::common::usleep_that_works(
                        (10 as libc::c_int as i64 * ipd_time_max) as u64,
                    );
                }
                if ipd_time > 0 as libc::c_int as i64 {
                    crate::common::usleep_that_works(ipd_time as u64);
                }
            }
            extc::gettimeofday(&mut stop, std::ptr::null_mut::<libc::c_void>());
            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::xscript_data_stop_server(
                    session, stop,
                ));
            }
            delta = (1000000 as libc::c_longlong * (stop.tv_sec - start.tv_sec) as libc::c_longlong
                + stop.tv_usec as libc::c_longlong
                - start.tv_usec as libc::c_longlong) as u64;
            if parameter.verbose_yn {
                extc::fprintf(
                    extc::stderr,
                    b"Server %d transferred %llu bytes in %0.2f seconds (%0.1f Mbps)\n\0"
                        as *const u8 as *const libc::c_char,
                    session.session_id,
                    parameter.file_size,
                    delta as libc::c_double / 1000000.0f64,
                    8.0f64 * parameter.file_size as libc::c_double
                        / (delta as libc::c_double
                            * 1e-6f64
                            * 1024 as libc::c_int as libc::c_double
                            * 1024 as libc::c_int as libc::c_double),
                );
            }
            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::xscript_close_server(
                    session, parameter, delta,
                ));
            }
            extc::close(session.transfer.udp_fd);
            session.transfer = Transfer::default();
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
                    parameter.files.push((path.clone(), metadata.len()));

                    parameter.file_name_size += path.as_os_str().len();
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

        eprintln!("total characters {}", parameter.file_name_size);
    }
    if 1 as libc::c_int == parameter.verbose_yn as libc::c_int {
        eprintln!("Block size: {}", parameter.block_size);
        eprintln!("Buffer size: {}", parameter.udp_buffer);
        eprintln!("Port: {}", parameter.tcp_port as libc::c_int);
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
