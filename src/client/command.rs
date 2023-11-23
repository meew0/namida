use std::{ffi::CString, io::Write, path::Path, sync::Arc};

use ::libc;
use anyhow::bail;

use super::{ring, Fraction, OutputMode, Parameter, Session, Statistics, Transfer};
use crate::{datagram, extc};

pub unsafe fn command_close(parameter: &Parameter, session: &mut Session) -> anyhow::Result<()> {
    if (session.server).is_null() {
        bail!("Tsunami session was not active");
    }

    extc::fclose(session.server);
    session.server = std::ptr::null_mut::<extc::FILE>();
    if parameter.verbose_yn {
        extc::printf(b"Connection closed.\n\n\0" as *const u8 as *const libc::c_char);
    }
    Ok(())
}
pub unsafe fn command_connect(
    command: &[&str],
    parameter: &mut Parameter,
) -> anyhow::Result<Session> {
    let mut server_fd: libc::c_int = 0;

    if command.len() > 1 {
        parameter.server_name = command[1].to_owned();
    }
    if command.len() > 2 {
        parameter.server_port = command[2].parse()?;
    }

    let mut session = Session {
        transfer: Default::default(),
        server: std::ptr::null_mut(),
        server_address: std::ptr::null_mut(),
        server_address_length: 0,
    };
    server_fd = super::network::create_tcp_socket_client(&mut session, parameter)?;
    if server_fd < 0 as libc::c_int {
        bail!(
            "Could not connect to {}:{}.",
            parameter.server_name,
            parameter.server_port
        );
    }
    session.server = extc::fdopen(server_fd, b"w+\0" as *const u8 as *const libc::c_char);
    if (session.server).is_null() {
        extc::close(server_fd);
        bail!("Could not convert control channel into a stream");
    }
    if let Err(err) = super::protocol::ttp_negotiate_client(&mut session) {
        extc::fclose(session.server);
        bail!("Protocol negotiation failed: {:?}", err);
    }

    let secret = match &parameter.passphrase {
        Some(passphrase) => passphrase.clone(),
        None => "kitten".to_owned(),
    };
    if let Err(err) = super::protocol::ttp_authenticate_client(&mut session, secret) {
        extc::fclose(session.server);
        bail!("Authentication failure: {:?}", err);
    }
    if parameter.verbose_yn {
        extc::printf(b"Connected.\n\n\0" as *const u8 as *const libc::c_char);
    }
    Ok(session)
}
pub unsafe fn command_dir(_command: &[&str], session: &mut Session) -> anyhow::Result<()> {
    let mut result: u8 = 0;
    let mut read_str: [libc::c_char; 2048] = [0; 2048];
    let mut num_files: u16 = 0;
    let mut i: u16 = 0;
    let mut filelen: usize = 0;
    let mut status: u16 = 0 as libc::c_int as u16;
    if (session.server).is_null() {
        bail!("Not connected to a Tsunami server");
    }
    extc::fprintf(
        session.server,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"!#DIR??\0" as *const u8 as *const libc::c_char,
    );
    status = extc::fread(
        &mut result as *mut u8 as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    ) as u16;
    if (status as libc::c_int) < 1 as libc::c_int {
        bail!("Could not read response to directory request");
    }
    if result as libc::c_int == 8 as libc::c_int {
        bail!("Server does no support listing of shared files");
    }
    read_str[0 as libc::c_int as usize] = result as libc::c_char;
    crate::common::fread_line(
        session.server,
        &mut *read_str.as_mut_ptr().offset(1 as libc::c_int as isize),
        (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong),
    )?;
    num_files = extc::atoi(read_str.as_mut_ptr()) as u16;
    extc::fprintf(
        extc::stderr,
        b"Remote file list:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as u16;
    while (i as libc::c_int) < num_files as libc::c_int {
        crate::common::fread_line(
            session.server,
            read_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )?;
        extc::fprintf(
            extc::stderr,
            b" %2d) %-64s\0" as *const u8 as *const libc::c_char,
            i as libc::c_int + 1 as libc::c_int,
            read_str.as_mut_ptr(),
        );
        crate::common::fread_line(
            session.server,
            read_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        )?;
        filelen = extc::atol(read_str.as_mut_ptr()) as usize;
        extc::fprintf(
            extc::stderr,
            b"%8Lu bytes\n\0" as *const u8 as *const libc::c_char,
            filelen as u64,
        );
        i = i.wrapping_add(1);
    }
    extc::fprintf(extc::stderr, b"\n\0" as *const u8 as *const libc::c_char);
    extc::fwrite(
        b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        session.server,
    );
    Ok(())
}
pub unsafe fn command_get(
    command: &[&str],
    parameter: &mut Parameter,
    session: &mut Session,
) -> anyhow::Result<()> {
    let mut current_block: u64;
    let mut this_block: u32 = 0 as libc::c_int as u32;
    let mut this_type: u16 = 0 as libc::c_int as u16;
    let mut delta: u64 = 0 as libc::c_int as u64;
    let mut block: u32 = 0 as libc::c_int as u32;
    let mut dumpcount: u32 = 0 as libc::c_int as u32;
    let mut mbit_thru: libc::c_double = 0.;
    let mut mbit_good: libc::c_double = 0.;
    let mut mbit_file: libc::c_double = 0.;
    let mut time_secs: libc::c_double = 0.;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut multimode: libc::c_int = 0 as libc::c_int;
    let mut file_names: *mut *mut libc::c_char = std::ptr::null_mut::<*mut libc::c_char>();
    let mut f_counter: u32 = 0 as libc::c_int as u32;
    let mut f_total: u32 = 0 as libc::c_int as u32;
    let mut f_arrsize: u32 = 0 as libc::c_int as u32;
    let mut ping_s: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ping_e: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut wait_u_sec: libc::c_long = 1 as libc::c_int as libc::c_long;
    if (command.len() as libc::c_int) < 2 as libc::c_int {
        bail!("Invalid command syntax (use 'help get' for details)");
    }
    if (session.server).is_null() {
        bail!("Not connected to a Tsunami server");
    }

    session.transfer = Transfer::default();

    if command[1] == "*" {
        let mut filearray_size: [libc::c_char; 10] = [0; 10];
        let mut file_count: [libc::c_char; 10] = [0; 10];
        multimode = 1 as libc::c_int;
        extc::printf(b"Requesting all available files\n\0" as *const u8 as *const libc::c_char);
        extc::gettimeofday(&mut ping_s, std::ptr::null_mut::<libc::c_void>());
        let command_1_c = CString::new(command[1]).unwrap();
        status = extc::fprintf(
            session.server,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            command_1_c.as_ptr(),
        );
        status = extc::fread(
            filearray_size.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            10 as libc::c_int as libc::c_ulong,
            session.server,
        ) as libc::c_int;
        extc::gettimeofday(&mut ping_e, std::ptr::null_mut::<libc::c_void>());
        status = extc::fread(
            file_count.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            10 as libc::c_int as libc::c_ulong,
            session.server,
        ) as libc::c_int;
        extc::fprintf(
            session.server,
            b"got size\0" as *const u8 as *const libc::c_char,
        );
        if status <= 0 as libc::c_int || extc::fflush(session.server) != 0 {
            bail!("Could not request file");
        }
        if status < 1 as libc::c_int {
            bail!("Could not read response to file request");
        }
        wait_u_sec = (ping_e.tv_sec - ping_s.tv_sec) * 1000000 as libc::c_int as extc::__time_t
            + (ping_e.tv_usec - ping_s.tv_usec);
        wait_u_sec =
            wait_u_sec + (wait_u_sec as libc::c_double * 0.1f64) as libc::c_int as libc::c_long;
        extc::sscanf(
            filearray_size.as_mut_ptr(),
            b"%u\0" as *const u8 as *const libc::c_char,
            &mut f_arrsize as *mut u32,
        );
        extc::sscanf(
            file_count.as_mut_ptr(),
            b"%u\0" as *const u8 as *const libc::c_char,
            &mut f_total as *mut u32,
        );
        if f_total <= 0 as libc::c_int as u32 {
            let mut dummy: [libc::c_char; 1] = [0; 1];
            status = extc::fread(
                dummy.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                session.server,
            ) as libc::c_int;
            bail!("Server advertised no files to get");
        } else {
            extc::printf(
                b"\nServer is sharing %u files\n\0" as *const u8 as *const libc::c_char,
                f_total,
            );
            file_names = extc::malloc(
                (f_total as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if file_names.is_null() {
                panic!("Could not allocate memory");
            }
            extc::printf(
                b"Multi-GET of %d files:\n\0" as *const u8 as *const libc::c_char,
                f_total,
            );
            f_counter = 0 as libc::c_int as u32;
            while f_counter < f_total {
                let mut tmpname: [libc::c_char; 1024] = [0; 1024];
                crate::common::fread_line(
                    session.server,
                    tmpname.as_mut_ptr(),
                    1024 as libc::c_int as u64,
                )?;
                let fresh0 = &mut (*file_names.offset(f_counter as isize));
                *fresh0 = extc::strdup(tmpname.as_mut_ptr());
                extc::printf(
                    b"%s \0" as *const u8 as *const libc::c_char,
                    *file_names.offset(f_counter as isize),
                );
                f_counter = f_counter.wrapping_add(1);
            }
            extc::fprintf(
                session.server,
                b"got list\0" as *const u8 as *const libc::c_char,
            );
            extc::printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        f_total = 1 as libc::c_int as u32;
    }
    f_counter = 0 as libc::c_int as u32;

    's_202: loop {
        let remote_filename = if multimode == 0 {
            command[1].to_owned()
        } else {
            extc::c_to_string(*file_names.offset(f_counter as isize))
        };

        let local_filename = if multimode == 0 {
            if command.len() as libc::c_int >= 3 as libc::c_int {
                // Local filename was specified
                command[2].to_owned()
            } else if let Some(last_slash) = remote_filename.rfind('/') {
                // Remote filename contains slash, use only the last part as the local filename
                remote_filename[(last_slash + 1)..].to_owned()
            } else {
                // Remote filename does not contain slash, use it as the local filename in its
                // entirety
                remote_filename.clone()
            }
        } else {
            let local_filename = extc::c_to_string(*file_names.offset(f_counter as isize));
            println!("GET *: now requesting file '{}'", local_filename);
            local_filename
        };

        super::protocol::ttp_open_transfer_client(
            session,
            parameter,
            remote_filename,
            local_filename,
        )?;

        super::protocol::ttp_open_port_client(session, parameter)?;
        session.transfer.retransmit.table = extc::calloc(
            super::config::DEFAULT_TABLE_SIZE as libc::c_ulong,
            ::core::mem::size_of::<u32>() as libc::c_ulong,
        ) as *mut u32;
        if (session.transfer.retransmit.table).is_null() {
            panic!("Could not allocate retransmission table");
        }
        session.transfer.received = vec![0; (session.transfer.block_count / 8 + 2) as usize];
        session.transfer.ring_buffer = Some(Arc::new(super::ring::RingBuffer::create(
            parameter.block_size,
        )));

        let mut local_datagram_buffer =
            ring::allocate_zeroed_boxed_slice(6 + parameter.block_size as usize);

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

        session.transfer.retransmit.table_size = super::config::DEFAULT_TABLE_SIZE as u32;
        session.transfer.retransmit.index_max = 0 as libc::c_int as u32;
        session.transfer.next_block = 1 as libc::c_int as u32;
        session.transfer.gapless_to_block = 0 as libc::c_int as u32;

        session.transfer.stats = Statistics::default();

        session.transfer.stats.start_udp_errors = crate::common::get_udp_in_errors();
        session.transfer.stats.this_udp_errors = session.transfer.stats.start_udp_errors;
        extc::gettimeofday(
            &mut session.transfer.stats.start_time,
            std::ptr::null_mut::<libc::c_void>(),
        );
        extc::gettimeofday(
            &mut session.transfer.stats.this_time,
            std::ptr::null_mut::<libc::c_void>(),
        );
        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::xscript_data_start_client(
                session,
                parameter,
                session.transfer.stats.start_time,
            ));
        }
        loop {
            status = extc::recvfrom(
                session.transfer.udp_fd,
                local_datagram_buffer.as_mut_ptr() as *mut libc::c_void,
                (6 as libc::c_int as u32).wrapping_add(parameter.block_size) as usize,
                0 as libc::c_int,
                extc::__SOCKADDR_ARG {
                    __sockaddr__: std::ptr::null_mut::<libc::c_void>() as *mut extc::sockaddr,
                },
                std::ptr::null_mut::<extc::socklen_t>(),
            ) as libc::c_int;
            if status < 0 as libc::c_int {
                println!("WARNING: UDP data transmission error");
                extc::printf(
                    b"Apparently frozen transfer, trying to do retransmit request\n\0" as *const u8
                        as *const libc::c_char,
                );
                if let Err(err) = super::protocol::ttp_repeat_retransmit(session) {
                    println!(
                        "WARNING: Repeat of retransmission requests failed: {:?}",
                        err
                    );
                    current_block = 78252603380123710;
                    break 's_202;
                }
            }

            let local_datagram_view = datagram::View::parse(&local_datagram_buffer);

            this_block = local_datagram_view.header.block_index;
            this_type = local_datagram_view.header.block_type;
            session.transfer.stats.total_blocks =
                (session.transfer.stats.total_blocks).wrapping_add(1);
            if this_type as libc::c_int != 'R' as i32 {
                session.transfer.stats.this_flow_originals =
                    (session.transfer.stats.this_flow_originals).wrapping_add(1);
            } else {
                session.transfer.stats.this_flow_retransmitteds =
                    (session.transfer.stats.this_flow_retransmitteds).wrapping_add(1);
                session.transfer.stats.total_recvd_retransmits =
                    (session.transfer.stats.total_recvd_retransmits).wrapping_add(1);
            }

            if !session.transfer.ring_buffer.as_mut().unwrap().is_full()
                && (!got_block(session, this_block)
                    || this_type as libc::c_int == 'X' as i32
                    || session.transfer.restart_pending as libc::c_int != 0)
            {
                if !got_block(session, this_block) {
                    session
                        .transfer
                        .ring_buffer
                        .as_mut()
                        .unwrap()
                        .reserve(local_datagram_view);
                    session.transfer.ring_buffer.as_mut().unwrap().confirm();

                    let fresh1 = &mut (session.transfer.received[(this_block / 8) as usize]);
                    *fresh1 = (*fresh1 as libc::c_int
                        | (1 as libc::c_int) << (this_block % 8 as libc::c_int as u32))
                        as u8;
                    if session.transfer.blocks_left > 0 as libc::c_int as u32 {
                        session.transfer.blocks_left =
                            (session.transfer.blocks_left).wrapping_sub(1);
                    } else {
                        extc::printf(
                                b"Oops! Negative-going blocks_left count at block: type=%c this=%u final=%u left=%u\n\0"
                                    as *const u8 as *const libc::c_char,
                                this_type as libc::c_int,
                                this_block,
                                session.transfer.block_count,
                                session.transfer.blocks_left,
                            );
                    }
                }
                if session.transfer.restart_pending as libc::c_int != 0
                    && this_type as libc::c_int != 'X' as i32
                {
                    if this_block > session.transfer.restart_lastidx
                        && this_block <= session.transfer.restart_wireclearidx
                    {
                        current_block = 13361531435213260772;
                    } else {
                        current_block = 8937240710477387595;
                    }
                } else {
                    current_block = 8937240710477387595;
                }
                match current_block {
                    13361531435213260772 => {}
                    _ => {
                        if this_block > session.transfer.next_block {
                            if !parameter.lossless {
                                if parameter.losswindow_ms == 0 as libc::c_int as u32 {
                                    session.transfer.gapless_to_block = this_block;
                                } else {
                                    let mut path_capability: libc::c_double = 0.;
                                    path_capability = 0.8f64
                                        * (session.transfer.stats.this_transmit_rate
                                            + session.transfer.stats.this_retransmit_rate);
                                    path_capability *=
                                        0.001f64 * parameter.losswindow_ms as libc::c_double;
                                    let mut earliest_block: u32 = (this_block as libc::c_double
                                        - (if ((1024 as libc::c_int * 1024 as libc::c_int)
                                            as libc::c_double
                                            * path_capability
                                            / (8 as libc::c_int as u32 * parameter.block_size)
                                                as libc::c_double)
                                            < this_block
                                                .wrapping_sub(session.transfer.gapless_to_block)
                                                as libc::c_double
                                        {
                                            (1024 as libc::c_int * 1024 as libc::c_int)
                                                as libc::c_double
                                                * path_capability
                                                / (8 as libc::c_int as u32 * parameter.block_size)
                                                    as libc::c_double
                                        } else {
                                            this_block
                                                .wrapping_sub(session.transfer.gapless_to_block)
                                                as libc::c_double
                                        }))
                                        as u32;
                                    block = earliest_block;
                                    while block < this_block {
                                        if let Err(err) =
                                            super::protocol::ttp_request_retransmit(session, block)
                                        {
                                            println!(
                                                "WARNING: Retransmission request failed: {:?}",
                                                err
                                            );
                                            current_block = 78252603380123710;
                                            break 's_202;
                                        } else {
                                            block = block.wrapping_add(1);
                                        }
                                    }
                                    session.transfer.next_block = earliest_block;
                                    session.transfer.gapless_to_block = earliest_block;
                                }
                            } else {
                                block = session.transfer.next_block;
                                while block < this_block {
                                    if let Err(err) =
                                        super::protocol::ttp_request_retransmit(session, block)
                                    {
                                        println!(
                                            "WARNING: Retransmission request failed: {:?}",
                                            err
                                        );
                                        current_block = 78252603380123710;
                                        break 's_202;
                                    } else {
                                        block = block.wrapping_add(1);
                                    }
                                }
                            }
                        }
                        while got_block(
                            session,
                            (session.transfer.gapless_to_block)
                                .wrapping_add(1 as libc::c_int as u32),
                        ) && session.transfer.gapless_to_block < session.transfer.block_count
                        {
                            session.transfer.gapless_to_block =
                                (session.transfer.gapless_to_block).wrapping_add(1);
                        }
                        if this_type as libc::c_int == 'O' as i32 {
                            session.transfer.next_block =
                                this_block.wrapping_add(1 as libc::c_int as u32);
                        }
                        if session.transfer.restart_pending as libc::c_int != 0
                            && session.transfer.next_block >= session.transfer.restart_lastidx
                        {
                            session.transfer.restart_pending = 0 as libc::c_int as u8;
                        }
                        if this_type as libc::c_int == 'X' as i32 {
                            if session.transfer.blocks_left == 0 as libc::c_int as u32 {
                                break;
                            }
                            if !parameter.lossless
                                && session.transfer.retransmit.index_max == 0 as libc::c_int as u32
                                && session.transfer.restart_pending == 0
                            {
                                break;
                            }
                            block = (session.transfer.gapless_to_block)
                                .wrapping_add(1 as libc::c_int as u32);
                            while block < session.transfer.block_count {
                                if let Err(err) =
                                    super::protocol::ttp_request_retransmit(session, block)
                                {
                                    println!("WARNING: Retransmission request failed: {:?}", err);
                                    current_block = 78252603380123710;
                                    break 's_202;
                                } else {
                                    block = block.wrapping_add(1);
                                }
                            }
                            super::protocol::ttp_repeat_retransmit(session)?;
                        }
                    }
                }
            }
            if session.transfer.stats.total_blocks % 50 as libc::c_int as u32 != 0 {
                continue;
            }
            if crate::common::get_usec_since(&mut session.transfer.stats.this_time)
                as libc::c_ulonglong
                <= 350000 as libc::c_longlong as libc::c_ulonglong
            {
                continue;
            }
            if let Err(err) = super::protocol::ttp_repeat_retransmit(session) {
                println!(
                    "WARNING: Repeat of retransmission requests failed: {:?}",
                    err
                );
                current_block = 78252603380123710;
                break 's_202;
            } else {
                super::protocol::ttp_update_stats(session, parameter)?;
                if parameter.blockdump {
                    let mut postfix = format!(".bmap{}", dumpcount);
                    if let Err(err) = dump_blockmap(&postfix, &session.transfer) {
                        eprintln!("Failed to write blockmap dump: {:?}", err);
                    }
                    dumpcount = dumpcount.wrapping_add(1);
                }
            }
        }
        extc::printf(
            b"Transfer complete. Flushing to disk and signaling server to stop...\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::close(session.transfer.udp_fd);
        if let Err(err) = super::protocol::ttp_request_stop(session) {
            println!("WARNING: Could not request end of transfer: {:?}", err);
            current_block = 78252603380123710;
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

            extc::gettimeofday(
                &mut session.transfer.stats.stop_time,
                std::ptr::null_mut::<libc::c_void>(),
            );
            delta = crate::common::get_usec_since(&mut session.transfer.stats.start_time);
            session.transfer.stats.total_lost = 0 as libc::c_int as u32;
            block = 1 as libc::c_int as u32;
            while block <= session.transfer.block_count {
                if !got_block(session, block) {
                    session.transfer.stats.total_lost =
                        (session.transfer.stats.total_lost).wrapping_add(1);
                }
                block = block.wrapping_add(1);
            }
            mbit_thru = 8.0f64
                * session.transfer.stats.total_blocks as libc::c_double
                * parameter.block_size as libc::c_double;
            mbit_good = mbit_thru
                - 8.0f64
                    * session.transfer.stats.total_recvd_retransmits as libc::c_double
                    * parameter.block_size as libc::c_double;
            mbit_file = 8.0f64 * session.transfer.file_size as libc::c_double;
            mbit_thru /= 1024.0f64 * 1024.0f64;
            mbit_good /= 1024.0f64 * 1024.0f64;
            mbit_file /= 1024.0f64 * 1024.0f64;
            time_secs = delta as libc::c_double / 1e6f64;
            extc::printf(
                b"PC performance figure : %llu packets dropped (if high this indicates receiving PC overload)\n\0"
                    as *const u8 as *const libc::c_char,
                (session.transfer.stats.this_udp_errors)
                    .wrapping_sub(session.transfer.stats.start_udp_errors),
            );
            extc::printf(
                b"Transfer duration     : %0.2f seconds\n\0" as *const u8 as *const libc::c_char,
                time_secs,
            );
            extc::printf(
                b"Total packet data     : %0.2f Mbit\n\0" as *const u8 as *const libc::c_char,
                mbit_thru,
            );
            extc::printf(
                b"Goodput data          : %0.2f Mbit\n\0" as *const u8 as *const libc::c_char,
                mbit_good,
            );
            extc::printf(
                b"File data             : %0.2f Mbit\n\0" as *const u8 as *const libc::c_char,
                mbit_file,
            );
            extc::printf(
                b"Throughput            : %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                mbit_thru / time_secs,
            );
            extc::printf(
                b"Goodput w/ restarts   : %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                mbit_good / time_secs,
            );
            extc::printf(
                b"Final file rate       : %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                mbit_file / time_secs,
            );
            extc::printf(b"Transfer mode         : \0" as *const u8 as *const libc::c_char);
            if parameter.lossless {
                if session.transfer.stats.total_lost == 0 as libc::c_int as u32 {
                    extc::printf(b"lossless\n\0" as *const u8 as *const libc::c_char);
                } else {
                    extc::printf(
                        b"lossless mode - but lost count=%u > 0, please file a bug report!!\n\0"
                            as *const u8 as *const libc::c_char,
                        session.transfer.stats.total_lost,
                    );
                }
            } else {
                if parameter.losswindow_ms == 0 as libc::c_int as u32 {
                    extc::printf(b"lossy\n\0" as *const u8 as *const libc::c_char);
                } else {
                    extc::printf(
                        b"semi-lossy, time window %d ms\n\0" as *const u8 as *const libc::c_char,
                        parameter.losswindow_ms,
                    );
                }
                extc::printf(
                    b"Data blocks lost      : %llu (%.2f%% of data) per user-specified time window constraint\n\0"
                        as *const u8 as *const libc::c_char,
                    session.transfer.stats.total_lost as u64,
                    100.0f64 * session.transfer.stats.total_lost as libc::c_double
                        / session.transfer.block_count as libc::c_double,
                );
            }
            extc::printf(b"\n\0" as *const u8 as *const libc::c_char);
            if parameter.transcript_yn {
                crate::common::transcript_warn_error(super::transcript::xscript_data_stop_client(
                    session,
                    parameter,
                    session.transfer.stats.stop_time,
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
            if !(session.transfer.retransmit.table).is_null() {
                extc::free(session.transfer.retransmit.table as *mut libc::c_void);
                session.transfer.retransmit.table = std::ptr::null_mut::<u32>();
            }
            if parameter.rate_adjust {
                parameter.target_rate = (1.15f64 * 1e6f64 * (mbit_file / time_secs)) as u64;
                extc::printf(
                    b"Adjusting target rate to %d Mbps for next transfer.\n\0" as *const u8
                        as *const libc::c_char,
                    (parameter.target_rate as libc::c_double / 1e6f64) as libc::c_int,
                );
            }
            f_counter = f_counter.wrapping_add(1);
            if f_counter >= f_total {
                current_block = 6000599718051633247;
                break;
            }
        }
    }
    match current_block {
        78252603380123710 => {
            extc::fprintf(
                extc::stderr,
                b"Transfer not successful.  (WARNING: You may need to reconnect.)\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            extc::close(session.transfer.udp_fd);
            if !(session.transfer.retransmit.table).is_null() {
                extc::free(session.transfer.retransmit.table as *mut libc::c_void);
                session.transfer.retransmit.table = std::ptr::null_mut::<u32>();
            }
            bail!("Transfer unsuccessful");
        }
        _ => {
            if multimode != 0 {
                f_counter = 0 as libc::c_int as u32;
                while f_counter < f_total {
                    extc::free(*file_names.offset(f_counter as isize) as *mut libc::c_void);
                    f_counter = f_counter.wrapping_add(1);
                }
                extc::free(file_names as *mut libc::c_void);
            }
            Ok(())
        }
    }
}
pub unsafe fn command_help(command: &[&str]) -> anyhow::Result<()> {
    if (command.len() as libc::c_int) < 2 as libc::c_int {
        extc::printf(
            b"Help is available for the following commands:\n\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(
            b"    close    connect    get    dir    help    quit    set\n\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(
            b"Use 'help <command>' for help on an individual command.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if command[1].eq_ignore_ascii_case("close") {
        extc::printf(b"Usage: close\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Closes the current connection to a remote Tsunami server.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if command[1].eq_ignore_ascii_case("connect") {
        extc::printf(b"Usage: connect\n\0" as *const u8 as *const libc::c_char);
        extc::printf(b"       connect <remote-host>\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"       connect <remote-host> <remote-port>\n\n\0" as *const u8 as *const libc::c_char,
        );
        extc::printf(
            b"Opens a connection to a remote Tsunami server.  If the host and port\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(
            b"are not specified, default values are used.  (Use the 'set' command to\n\0"
                as *const u8 as *const libc::c_char,
        );
        extc::printf(b"modify these values.)\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"After connecting, you will be prompted to enter a shared secret for\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(b"authentication.\n\n\0" as *const u8 as *const libc::c_char);
    } else if command[1].eq_ignore_ascii_case("get") {
        extc::printf(b"Usage: get <remote-file>\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"       get <remote-file> <local-file>\n\n\0" as *const u8 as *const libc::c_char,
        );
        extc::printf(
            b"Attempts to retrieve the remote file with the given name using the\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(
            b"Tsunami file transfer protocol.  If the local filename is not\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(
            b"specified, the final part of the remote filename (after the last path\n\0"
                as *const u8 as *const libc::c_char,
        );
        extc::printf(b"separator) will be used.\n\n\0" as *const u8 as *const libc::c_char);
    } else if command[1].eq_ignore_ascii_case("dir") {
        extc::printf(b"Usage: dir\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Attempts to list the available remote files.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if command[1].eq_ignore_ascii_case("help") {
        extc::printf(
            b"Come on.  You know what that command does.\n\n\0" as *const u8 as *const libc::c_char,
        );
    } else if command[1].eq_ignore_ascii_case("quit") {
        extc::printf(b"Usage: quit\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Closes any open connection to a remote Tsunami server and exits the\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(b"Tsunami client.\n\n\0" as *const u8 as *const libc::c_char);
    } else if command[1].eq_ignore_ascii_case("set") {
        extc::printf(b"Usage: set\n\0" as *const u8 as *const libc::c_char);
        extc::printf(b"       set <field>\n\0" as *const u8 as *const libc::c_char);
        extc::printf(b"       set <field> <value>\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Sets one of the defaults to the given value.  If the value is omitted,\n\0"
                as *const u8 as *const libc::c_char,
        );
        extc::printf(
            b"the current value of the field is returned.  If the field is also\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(
            b"omitted, the current values of all defaults are returned.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        println!("'{}' is not a recognized command.", command[1]);
        extc::printf(
            b"Use 'help' for a list of commands.\n\n\0" as *const u8 as *const libc::c_char,
        );
    }
    Ok(())
}

pub fn command_quit(maybe_session: &mut Option<Session>) {
    if let Some(session) = maybe_session {
        if !(session.server).is_null() {
            unsafe {
                extc::fclose(session.server);
            }
        }
    }

    println!("Thank you for using Tsunami.\n\0");
    println!("The ANML web site can be found at:    http://www.anml.iu.edu/");
    println!("The SourceForge site can be found at: http://tsunami-udp.sf.net/");
    println!();

    std::process::exit(0);
}

pub unsafe fn command_set(command: &[&str], parameter: &mut Parameter) -> anyhow::Result<()> {
    let mut do_all: libc::c_int = (command.len() as libc::c_int == 1 as libc::c_int) as libc::c_int;
    if command.len() as libc::c_int == 3 as libc::c_int {
        if command[1].eq_ignore_ascii_case("server") {
            parameter.server_name = command[2].to_owned();
        } else if command[1].eq_ignore_ascii_case("port") {
            parameter.server_port = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("udpport") {
            parameter.client_port = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("buffer") {
            parameter.udp_buffer = command[2].parse()?;
        } else if command[1].eq_ignore_ascii_case("blocksize") {
            parameter.block_size = command[2].parse()?;
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
                OutputMode::Default
            };
        } else if command[1].eq_ignore_ascii_case("rateadjust") {
            parameter.rate_adjust = command[2] == "yes";
        } else if command[1].eq_ignore_ascii_case("rate") {
            parameter.target_rate = parse_rate(command[2])?;
        } else if command[1].eq_ignore_ascii_case("error") {
            parameter.error_rate = command[2].parse::<u32>()? * 1000;
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
    if do_all != 0 || command[1].eq_ignore_ascii_case("server") {
        println!("server = {}", parameter.server_name);
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("port") {
        extc::printf(
            b"port = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.server_port as libc::c_int,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("udpport") {
        extc::printf(
            b"udpport = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.client_port as libc::c_int,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("buffer") {
        extc::printf(
            b"buffer = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.udp_buffer,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("blocksize") {
        extc::printf(
            b"blocksize = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.block_size,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("verbose") {
        extc::printf(
            b"verbose = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.verbose_yn as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("transcript") {
        extc::printf(
            b"transcript = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.transcript_yn as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("ip") {
        extc::printf(
            b"ip = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.ipv6_yn as libc::c_int != 0 {
                b"v6\0" as *const u8 as *const libc::c_char
            } else {
                b"v4\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("output") {
        extc::printf(
            b"output = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.output_mode as libc::c_int == 0 as libc::c_int {
                b"screen\0" as *const u8 as *const libc::c_char
            } else {
                b"line\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("rate") {
        extc::printf(
            b"rate = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.target_rate,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("rateadjust") {
        extc::printf(
            b"rateadjust = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.rate_adjust as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("error") {
        extc::printf(
            b"error = %0.2f%%\n\0" as *const u8 as *const libc::c_char,
            parameter.error_rate as libc::c_double / 1000.0f64,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("slowdown") {
        extc::printf(
            b"slowdown = %d/%d\n\0" as *const u8 as *const libc::c_char,
            parameter.slower.numerator as libc::c_int,
            parameter.slower.denominator as libc::c_int,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("speedup") {
        extc::printf(
            b"speedup = %d/%d\n\0" as *const u8 as *const libc::c_char,
            parameter.faster.numerator as libc::c_int,
            parameter.faster.denominator as libc::c_int,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("history") {
        extc::printf(
            b"history = %d%%\n\0" as *const u8 as *const libc::c_char,
            parameter.history as libc::c_int,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("lossless") {
        extc::printf(
            b"lossless = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.lossless as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("losswindow") {
        extc::printf(
            b"losswindow = %d msec\n\0" as *const u8 as *const libc::c_char,
            parameter.losswindow_ms,
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("blockdump") {
        extc::printf(
            b"blockdump = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.blockdump as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0 || command[1].eq_ignore_ascii_case("passphrase") {
        extc::printf(
            b"passphrase = %s\n\0" as *const u8 as *const libc::c_char,
            if (parameter.passphrase).is_none() {
                b"default\0" as *const u8 as *const libc::c_char
            } else {
                b"<user-specified>\0" as *const u8 as *const libc::c_char
            },
        );
    }
    extc::printf(b"\n\0" as *const u8 as *const libc::c_char);
    Ok(())
}

pub fn disk_thread(
    ring_buffer: Arc<super::ring::RingBuffer>,
    block_count: u32,
    file_size: u64,
    mut file: std::fs::File,
) -> anyhow::Result<()> {
    loop {
        ring_buffer.peek(|datagram_view| {
            if datagram_view.header.block_index == 0 {
                bail!("!!!!");
            }
            super::io::accept_block(datagram_view, block_count, file_size, &mut file)?;
            Ok(())
        })?;
        ring_buffer.pop();
    }
}

pub fn parse_rate(rate: &str) -> anyhow::Result<u64> {
    let (main_part, last_char) = rate.split_at(rate.len() - 1);
    let parsed: u64 = main_part.parse()?;

    match last_char {
        "k" | "K" => Ok(parsed * 1000),
        "m" | "M" => Ok(parsed * 1000000),
        "g" | "G" => Ok(parsed * 1000000000),
        "t" | "T" => Ok(parsed * 1000000000000),
        "0" | "1" | "2" | "3" | "4" | "5" | "6" | "7" | "8" | "9" => {
            let full_parsed: u64 = rate.parse()?;
            Ok(full_parsed)
        }
        _ => bail!("Invalid unit specifier"),
    }
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

pub fn got_block(session: &Session, blocknr: u32) -> bool {
    if blocknr > session.transfer.block_count {
        return true;
    }

    session.transfer.received[(blocknr / 8) as usize] & (1 << (blocknr % 8)) != 0
}

pub fn dump_blockmap(postfix: &str, xfer: &Transfer) -> anyhow::Result<()> {
    let fname = format!("{}{}", xfer.local_filename.as_ref().unwrap(), postfix);
    let mut fbits = std::fs::File::options()
        .write(true)
        .create(true)
        .open(Path::new(&fname))?;

    fbits.write_all(&xfer.block_count.to_le_bytes())?;

    let block_data = &xfer.received[0..((xfer.block_count / 8).wrapping_add(1) as usize)];
    fbits.write_all(block_data)?;

    Ok(())
}
