use crate::extc;
use ::libc;

extern "C" {
    static mut g_error: [libc::c_char; 0];
}

pub unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *mut libc::c_char) -> libc::c_int {
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
    let mut parameter: super::ttp_parameter_t = super::ttp_parameter_t {
        epoch: 0,
        verbose_yn: 0,
        transcript_yn: 0,
        ipv6_yn: 0,
        tcp_port: 0,
        udp_buffer: 0,
        hb_timeout: 0,
        secret: 0 as *const u8,
        client: 0 as *const libc::c_char,
        finishhook: 0 as *const u8,
        allhook: 0 as *const u8,
        block_size: 0,
        file_size: 0,
        block_count: 0,
        target_rate: 0,
        error_rate: 0,
        ipd_time: 0,
        slower_num: 0,
        slower_den: 0,
        faster_num: 0,
        faster_den: 0,
        ringbuf: 0 as *mut libc::c_char,
        fileout: 0,
        slotnumber: 0,
        totalslots: 0,
        samplerate: 0,
        file_names: 0 as *mut *mut libc::c_char,
        file_sizes: 0 as *mut u64,
        file_name_size: 0,
        total_files: 0,
        wait_u_sec: 0,
    };
    let mut session: super::ttp_session_t = super::ttp_session_t {
        parameter: 0 as *mut super::ttp_parameter_t,
        transfer: super::ttp_transfer_t {
            parameter: 0 as *mut super::ttp_parameter_t,
            filename: 0 as *mut libc::c_char,
            file: 0 as *mut extc::FILE,
            vsib: 0 as *mut extc::FILE,
            transcript: 0 as *mut extc::FILE,
            udp_fd: 0,
            udp_address: 0 as *mut extc::sockaddr,
            udp_length: 0,
            ipd_current: 0.,
            block: 0,
        },
        client_fd: 0,
        session_id: 0,
    };
    let mut child_pid: extc::pid_t = 0;
    extc::memset(
        &mut session as *mut super::ttp_session_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<super::ttp_session_t>() as libc::c_ulong,
    );
    super::config::reset_server(&mut parameter);
    process_options(argc, argv, &mut parameter);
    server_fd = super::network::create_tcp_socket_server(&mut parameter);
    if server_fd < 0 as libc::c_int {
        extc::sprintf(
            g_error.as_mut_ptr(),
            b"Could not create server socket on port %d\0" as *const u8 as *const libc::c_char,
            parameter.tcp_port as libc::c_int,
        );
        return crate::common::error::error_handler(
            b"main.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            g_error.as_mut_ptr(),
            1 as libc::c_int,
        );
    }
    extc::signal(
        17 as libc::c_int,
        Some(reap as unsafe extern "C" fn(libc::c_int) -> ()),
    );
    extc::fprintf(
        extc::stderr,
        b"Tsunami Server for protocol rev %X\nRevision: %s\nCompiled: %s %s\nWaiting for clients to connect.\n\0"
            as *const u8 as *const libc::c_char,
        crate::common::common::PROTOCOL_REVISION,
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
            crate::common::error::error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                142 as libc::c_int,
                b"Could not accept client connection\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            extc::fprintf(
                extc::stderr,
                b"New client connecting from %s...\n\0" as *const u8 as *const libc::c_char,
                extc::inet_ntoa(remote_address.sin_addr),
            );
            child_pid = extc::fork();
            if child_pid < 0 as libc::c_int {
                crate::common::error::error_handler(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    151 as libc::c_int,
                    b"Could not create child process\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            } else {
                session.session_id += 1;
                session.session_id;
                if child_pid == 0 as libc::c_int {
                    extc::close(server_fd);
                    session.client_fd = client_fd;
                    session.parameter = &mut parameter;
                    extc::memset(
                        &mut session.transfer as *mut super::ttp_transfer_t as *mut libc::c_void,
                        0 as libc::c_int,
                        ::core::mem::size_of::<super::ttp_transfer_t>() as libc::c_ulong,
                    );
                    session.transfer.ipd_current = 0.0f64;
                    client_handler(&mut session);
                    return 0 as libc::c_int;
                } else {
                    extc::close(client_fd);
                }
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn client_handler(mut session: *mut super::ttp_session_t) {
    let mut retransmission: super::retransmission_t = super::retransmission_t {
        request_type: 0,
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
    let mut datagram: [u8; 65536] = [0; 65536];
    let mut ipd_time: i64 = 0;
    let mut ipd_usleep_diff: i64 = 0;
    let mut ipd_time_max: i64 = 0;
    let mut status: libc::c_int = 0;
    let mut xfer: *mut super::ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut super::ttp_parameter_t = (*session).parameter;
    let mut delta: u64 = 0;
    let mut block_type: u8 = 0;
    status = super::protocol::ttp_negotiate_server(session);
    if status < 0 as libc::c_int {
        crate::common::error::error_handler(
            b"main.c\0" as *const u8 as *const libc::c_char,
            211 as libc::c_int,
            b"Protocol revision number mismatch\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = super::protocol::ttp_authenticate_server(session, (*(*session).parameter).secret);
    if status < 0 as libc::c_int {
        crate::common::error::error_handler(
            b"main.c\0" as *const u8 as *const libc::c_char,
            216 as libc::c_int,
            b"Client authentication failure\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if 1 as libc::c_int == (*param).verbose_yn as libc::c_int {
        extc::fprintf(
            extc::stderr,
            b"Client authenticated. Negotiated parameters are:\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::fprintf(
            extc::stderr,
            b"Block size: %d\n\0" as *const u8 as *const libc::c_char,
            (*param).block_size,
        );
        extc::fprintf(
            extc::stderr,
            b"Buffer size: %d\n\0" as *const u8 as *const libc::c_char,
            (*param).udp_buffer,
        );
        extc::fprintf(
            extc::stderr,
            b"Port: %d\n\0" as *const u8 as *const libc::c_char,
            (*param).tcp_port as libc::c_int,
        );
    }
    loop {
        status = extc::fcntl((*session).client_fd, 4 as libc::c_int, 0 as libc::c_int);
        if status < 0 as libc::c_int {
            crate::common::error::error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int,
                b"Could not make client socket blocking\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        status = super::protocol::ttp_open_transfer_server(session);
        if status < 0 as libc::c_int {
            crate::common::error::error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                236 as libc::c_int,
                b"Invalid file request\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            status = super::protocol::ttp_open_port_server(session);
            if status < 0 as libc::c_int {
                crate::common::error::error_handler(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    243 as libc::c_int,
                    b"UDP socket creation failed\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            } else {
                status = extc::fcntl(
                    (*session).client_fd,
                    4 as libc::c_int,
                    0o4000 as libc::c_int,
                );
                if status < 0 as libc::c_int {
                    crate::common::error::error_handler(
                        b"main.c\0" as *const u8 as *const libc::c_char,
                        250 as libc::c_int,
                        b"Could not make client socket non-blocking\0" as *const u8
                            as *const libc::c_char,
                        1 as libc::c_int,
                    );
                }
                extc::gettimeofday(&mut start, 0 as *mut libc::c_void);
                if (*param).transcript_yn != 0 {
                    super::transcript::xscript_data_start_server(session, &mut start);
                }
                lasthblostreport = start;
                lastfeedback = start;
                prevpacketT = start;
                deadconnection_counter = 0 as libc::c_int as u32;
                ipd_time = 0 as libc::c_int as i64;
                ipd_time_max = 0 as libc::c_int as i64;
                ipd_usleep_diff = 0 as libc::c_int as i64;
                retransmitlen = 0 as libc::c_int;
                (*xfer).block = 0 as libc::c_int as u32;
                while (*xfer).block <= (*param).block_count {
                    block_type = 'R' as i32 as u8;
                    extc::gettimeofday(&mut currpacketT, 0 as *mut libc::c_void);
                    ipd_usleep_diff = ((*xfer).ipd_current
                        + ((prevpacketT.tv_sec - currpacketT.tv_sec) as libc::c_double * 1e6f64
                            + (prevpacketT.tv_usec - currpacketT.tv_usec) as libc::c_double))
                        as i64;
                    prevpacketT = currpacketT;
                    if ipd_usleep_diff > 0 as libc::c_int as i64
                        || ipd_time > 0 as libc::c_int as i64
                    {
                        ipd_time += ipd_usleep_diff;
                    }
                    ipd_time_max = if ipd_time > ipd_time_max {
                        ipd_time
                    } else {
                        ipd_time_max
                    };
                    status = extc::read(
                        (*session).client_fd,
                        (&mut retransmission as *mut super::retransmission_t as *mut libc::c_char)
                            .offset(retransmitlen as isize)
                            as *mut libc::c_void,
                        ::core::mem::size_of::<super::retransmission_t>()
                            .wrapping_sub(retransmitlen as usize) as u64,
                    ) as libc::c_int;
                    if status <= 0 as libc::c_int && *extc::__errno_location() != 11 as libc::c_int
                    {
                        crate::common::error::error_handler(
                            b"main.c\0" as *const u8 as *const libc::c_char,
                            288 as libc::c_int,
                            b"Retransmission read failed\0" as *const u8 as *const libc::c_char,
                            1 as libc::c_int,
                        );
                    }
                    if status > 0 as libc::c_int {
                        retransmitlen += status;
                    }
                    if retransmitlen as libc::c_ulong
                        == ::core::mem::size_of::<super::retransmission_t>() as libc::c_ulong
                    {
                        lastfeedback = currpacketT;
                        lasthblostreport = currpacketT;
                        deadconnection_counter = 0 as libc::c_int as u32;
                        if extc::__bswap_16(retransmission.request_type) as libc::c_int
                            == crate::common::common::REQUEST_STOP as libc::c_int
                        {
                            extc::fprintf(
                                extc::stderr,
                                b"Transmission of %s complete.\n\0" as *const u8
                                    as *const libc::c_char,
                                (*xfer).filename,
                            );
                            if !((*param).finishhook).is_null() {
                                let MaxCommandLength: libc::c_int = 1024 as libc::c_int;
                                let vla = MaxCommandLength as usize;
                                let mut cmd: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
                                let mut v: libc::c_int = 0;
                                v = extc::snprintf(
                                    cmd.as_mut_ptr(),
                                    MaxCommandLength as libc::c_ulong,
                                    b"%s %s\0" as *const u8 as *const libc::c_char,
                                    (*param).finishhook,
                                    (*xfer).filename,
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
                            status = super::protocol::ttp_accept_retransmit(
                                session,
                                &mut retransmission,
                                datagram.as_mut_ptr(),
                            );
                            if status < 0 as libc::c_int {
                                crate::common::error::error_handler(
                                    b"main.c\0" as *const u8 as *const libc::c_char,
                                    333 as libc::c_int,
                                    b"Retransmission error\0" as *const u8 as *const libc::c_char,
                                    0 as libc::c_int,
                                );
                            }
                            retransmitlen = 0 as libc::c_int;
                        }
                    } else if (retransmitlen as libc::c_ulong)
                        < ::core::mem::size_of::<super::retransmission_t>() as libc::c_ulong
                    {
                        (*xfer).block = if ((*xfer).block).wrapping_add(1 as libc::c_int as u32)
                            < (*param).block_count
                        {
                            ((*xfer).block).wrapping_add(1 as libc::c_int as u32)
                        } else {
                            (*param).block_count
                        };
                        block_type = (if (*xfer).block == (*param).block_count {
                            'X' as i32
                        } else {
                            'O' as i32
                        }) as u8;
                        status = super::io::build_datagram(
                            session,
                            (*xfer).block,
                            block_type as u16,
                            datagram.as_mut_ptr(),
                        );
                        if status < 0 as libc::c_int {
                            extc::sprintf(
                                g_error.as_mut_ptr(),
                                b"Could not read block #%u\0" as *const u8 as *const libc::c_char,
                                (*xfer).block,
                            );
                            crate::common::error::error_handler(
                                b"main.c\0" as *const u8 as *const libc::c_char,
                                345 as libc::c_int,
                                g_error.as_mut_ptr(),
                                1 as libc::c_int,
                            );
                        }
                        status = extc::sendto(
                            (*xfer).udp_fd,
                            datagram.as_mut_ptr() as *const libc::c_void,
                            (6 as libc::c_int as u32).wrapping_add((*param).block_size) as u64,
                            0 as libc::c_int,
                            extc::__CONST_SOCKADDR_ARG {
                                __sockaddr__: (*xfer).udp_address,
                            },
                            (*xfer).udp_length,
                        ) as libc::c_int;
                        if status < 0 as libc::c_int {
                            extc::sprintf(
                                g_error.as_mut_ptr(),
                                b"Could not transmit block #%u\0" as *const u8
                                    as *const libc::c_char,
                                (*xfer).block,
                            );
                            crate::common::error::error_handler(
                                b"main.c\0" as *const u8 as *const libc::c_char,
                                352 as libc::c_int,
                                g_error.as_mut_ptr(),
                                0 as libc::c_int,
                            );
                            continue;
                        }
                    } else if retransmitlen as libc::c_ulong
                        > ::core::mem::size_of::<super::retransmission_t>() as libc::c_ulong
                    {
                        extc::fprintf(
                            extc::stderr,
                            b"warn: retransmitlen > %d\n\0" as *const u8 as *const libc::c_char,
                            ::core::mem::size_of::<super::retransmission_t>() as libc::c_ulong
                                as libc::c_int,
                        );
                        retransmitlen = 0 as libc::c_int;
                    }
                    let fresh0 = deadconnection_counter;
                    deadconnection_counter = deadconnection_counter.wrapping_add(1);
                    if fresh0 > 2048 as libc::c_int as u32 {
                        let mut stats_line: [libc::c_char; 160] = [0; 160];
                        deadconnection_counter = 0 as libc::c_int as u32;
                        if (crate::common::common::get_usec_since(&mut lasthblostreport)
                            as libc::c_double)
                            < 500000.0f64
                        {
                            continue;
                        }
                        extc::gettimeofday(&mut lasthblostreport, 0 as *mut libc::c_void);
                        retransmission.request_type =
                            extc::__bswap_16(crate::common::common::REQUEST_ERROR_RATE);
                        retransmission.error_rate = extc::__bswap_32(100000 as libc::c_int as u32);
                        retransmission.block = 0 as libc::c_int as u32;
                        super::protocol::ttp_accept_retransmit(
                            session,
                            &mut retransmission,
                            datagram.as_mut_ptr(),
                        );
                        delta = crate::common::common::get_usec_since(&mut lastfeedback);
                        extc::snprintf(
                            stats_line.as_mut_ptr(),
                            (::core::mem::size_of::<[libc::c_char; 160]>() as libc::c_ulong)
                                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                            b"   n/a     n/a     n/a %7u %6.2f %3u -- no heartbeat since %3.2fs\n\0"
                                as *const u8 as *const libc::c_char,
                            (*xfer).block,
                            100.0f64 * (*xfer).block as libc::c_double
                                / (*param).block_count as libc::c_double,
                            (*session).session_id,
                            1e-6f64 * delta as libc::c_double,
                        );
                        if (*param).transcript_yn != 0 {
                            super::transcript::xscript_data_log_server(
                                session,
                                stats_line.as_mut_ptr(),
                            );
                        }
                        extc::fprintf(
                            extc::stderr,
                            b"%s\0" as *const u8 as *const libc::c_char,
                            stats_line.as_mut_ptr(),
                        );
                        if 1e-6f64 * delta as libc::c_double
                            > (*param).hb_timeout as libc::c_int as libc::c_double
                        {
                            extc::fprintf(
                                extc::stderr,
                                b"Heartbeat timeout of %d seconds reached, terminating transfer.\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*param).hb_timeout as libc::c_int,
                            );
                            break;
                        }
                    }
                    if block_type as libc::c_int == 'X' as i32 {
                        crate::common::common::usleep_that_works(
                            (10 as libc::c_int as i64 * ipd_time_max) as u64,
                        );
                    }
                    if ipd_time > 0 as libc::c_int as i64 {
                        crate::common::common::usleep_that_works(ipd_time as u64);
                    }
                }
                extc::gettimeofday(&mut stop, 0 as *mut libc::c_void);
                if (*param).transcript_yn != 0 {
                    super::transcript::xscript_data_stop_server(session, &mut stop);
                }
                delta = (1000000 as libc::c_longlong
                    * (stop.tv_sec - start.tv_sec) as libc::c_longlong
                    + stop.tv_usec as libc::c_longlong
                    - start.tv_usec as libc::c_longlong) as u64;
                if (*param).verbose_yn != 0 {
                    extc::fprintf(
                        extc::stderr,
                        b"Server %d transferred %llu bytes in %0.2f seconds (%0.1f Mbps)\n\0"
                            as *const u8 as *const libc::c_char,
                        (*session).session_id,
                        (*param).file_size as u64,
                        delta as libc::c_double / 1000000.0f64,
                        8.0f64 * (*param).file_size as libc::c_double
                            / (delta as libc::c_double
                                * 1e-6f64
                                * 1024 as libc::c_int as libc::c_double
                                * 1024 as libc::c_int as libc::c_double),
                    );
                }
                if (*param).transcript_yn != 0 {
                    super::transcript::xscript_close_server(session, delta);
                }
                extc::fclose((*xfer).file);
                extc::close((*xfer).udp_fd);
                extc::memset(
                    xfer as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<super::ttp_transfer_t>() as libc::c_ulong,
                );
            }
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn process_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut parameter: *mut super::ttp_parameter_t,
) {
    let mut long_options: [extc::option; 12] = [
        {
            let mut init = extc::option {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"transcript\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"v6\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '6' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"port\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"secret\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"buffer\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"hbtimeout\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"v\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"client\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'c' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"finishhook\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'f' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: b"allhook\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'a' as i32,
            };
            init
        },
        {
            let mut init = extc::option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut filestat: extc::stat = extc::stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: extc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_mtim: extc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        st_ctim: extc::timespec {
            tv_sec: 0,
            tv_nsec: 0,
        },
        __glibc_reserved: [0; 3],
    };
    let mut which: libc::c_int = 0;
    loop {
        which = extc::getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"+\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(which > 0 as libc::c_int) {
            break;
        }
        match which {
            118 => {
                (*parameter).verbose_yn = 1 as libc::c_int as u8;
            }
            116 => {
                (*parameter).transcript_yn = 1 as libc::c_int as u8;
            }
            54 => {
                (*parameter).ipv6_yn = 1 as libc::c_int as u8;
            }
            112 => {
                (*parameter).tcp_port = extc::atoi(extc::optarg) as u16;
            }
            115 => {
                (*parameter).secret = extc::optarg as *mut libc::c_uchar;
            }
            99 => {
                (*parameter).client = extc::optarg;
            }
            102 => {
                (*parameter).finishhook = extc::optarg as *mut libc::c_uchar;
            }
            97 => {
                (*parameter).allhook = extc::optarg as *mut libc::c_uchar;
            }
            98 => {
                (*parameter).udp_buffer = extc::atoi(extc::optarg) as u32;
            }
            104 => {
                (*parameter).hb_timeout = extc::atoi(extc::optarg) as u16;
            }
            _ => {
                extc::fprintf(
                    extc::stderr,
                    b"Usage: tsunamid [--verbose] [--transcript] [--v6] [--port=n] [--buffer=bytes]\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"                [--hbtimeout=seconds] [--allhook=cmd] [--finishhook=cmd]\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"                \0" as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"[filename1 filename2 ...]\n\n\0" as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"verbose or v : turns on verbose output mode\n\0" as *const u8
                        as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"transcript   : turns on transcript mode for statistics recording\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"v6           : operates using IPv6 instead of (not in addition to!) IPv4\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"port         : specifies which TCP port on which to listen to incoming connections\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"secret       : specifies the shared secret for the client and server\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"client       : specifies an alternate client IP or host where to send data\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"buffer       : specifies the desired size for UDP socket send buffer (in bytes)\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"hbtimeout    : specifies the timeout in seconds for disconnect after client heartbeat lost\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"finishhook   : run command on transfer completion, file name is appended automatically\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"allhook      : run command on 'get *' to produce a custom file list for client downloads\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(
                    extc::stderr,
                    b"filenames    : list of files to share for downloaded via a client 'GET *'\n\0"
                        as *const u8 as *const libc::c_char,
                );
                extc::fprintf(extc::stderr, b"\n\0" as *const u8 as *const libc::c_char);
                extc::fprintf(
                    extc::stderr,
                    b"Defaults: verbose    = %d\n\0" as *const u8 as *const libc::c_char,
                    super::config::DEFAULT_VERBOSE_YN as libc::c_int,
                );
                extc::fprintf(
                    extc::stderr,
                    b"          transcript = %d\n\0" as *const u8 as *const libc::c_char,
                    super::config::DEFAULT_TRANSCRIPT_YN as libc::c_int,
                );
                extc::fprintf(
                    extc::stderr,
                    b"          v6         = %d\n\0" as *const u8 as *const libc::c_char,
                    super::config::DEFAULT_IPV6_YN as libc::c_int,
                );
                extc::fprintf(
                    extc::stderr,
                    b"          port       = %d\n\0" as *const u8 as *const libc::c_char,
                    super::config::DEFAULT_TCP_PORT as libc::c_int,
                );
                extc::fprintf(
                    extc::stderr,
                    b"          buffer     = %d bytes\n\0" as *const u8 as *const libc::c_char,
                    super::config::DEFAULT_UDP_BUFFER,
                );
                extc::fprintf(
                    extc::stderr,
                    b"          hbtimeout  = %d seconds\n\0" as *const u8 as *const libc::c_char,
                    super::config::DEFAULT_HEARTBEAT_TIMEOUT as libc::c_int,
                );
                extc::fprintf(extc::stderr, b"\n\0" as *const u8 as *const libc::c_char);
                extc::exit(1 as libc::c_int);
            }
        }
    }
    if argc > extc::optind {
        let mut counter: libc::c_int = 0;
        (*parameter).file_names = argv.offset(extc::optind as isize);
        (*parameter).file_name_size = 0 as libc::c_int as u16;
        (*parameter).total_files = (argc - extc::optind) as u16;
        (*parameter).file_sizes = extc::malloc(
            (::core::mem::size_of::<usize>() as libc::c_ulong)
                .wrapping_mul((*parameter).total_files as libc::c_ulong),
        ) as *mut u64;
        extc::fprintf(
            extc::stderr,
            b"\nThe specified %d files will be listed on GET *:\n\0" as *const u8
                as *const libc::c_char,
            (*parameter).total_files as libc::c_int,
        );
        counter = 0 as libc::c_int;
        while counter < argc - extc::optind {
            extc::stat(
                *((*parameter).file_names).offset(counter as isize),
                &mut filestat,
            );
            *((*parameter).file_sizes).offset(counter as isize) = filestat.st_size as u64;
            (*parameter).file_name_size = ((*parameter).file_name_size as libc::c_ulong)
                .wrapping_add(
                    (extc::strlen(*((*parameter).file_names).offset(counter as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as u16 as u16;
            extc::fprintf(
                extc::stderr,
                b" %3d)   %-20s  %llu bytes\n\0" as *const u8 as *const libc::c_char,
                counter + 1 as libc::c_int,
                *((*parameter).file_names).offset(counter as isize),
                *((*parameter).file_sizes).offset(counter as isize) as u64,
            );
            counter += 1;
            counter;
        }
        extc::fprintf(
            extc::stderr,
            b"total characters %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).file_name_size as libc::c_int,
        );
    }
    if 1 as libc::c_int == (*parameter).verbose_yn as libc::c_int {
        extc::fprintf(
            extc::stderr,
            b"Block size: %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).block_size,
        );
        extc::fprintf(
            extc::stderr,
            b"Buffer size: %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).udp_buffer,
        );
        extc::fprintf(
            extc::stderr,
            b"Port: %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).tcp_port as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn reap(mut signum: libc::c_int) {
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
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *mut libc::c_char,
        ) as i32)
    }
}
