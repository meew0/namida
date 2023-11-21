use std::{ffi::CStr, sync::Arc};

use ::libc;
use anyhow::bail;

use super::{ring, Command, Parameter, Session, Statistics, Transfer};
use crate::{datagram, extc};

pub unsafe fn command_close(parameter: &Parameter, session: &mut Session) -> anyhow::Result<()> {
    if (session.server).is_null() {
        bail!("Tsunami session was not active");
    }

    extc::fclose(session.server);
    session.server = std::ptr::null_mut::<extc::FILE>();
    if parameter.verbose_yn != 0 {
        extc::printf(b"Connection closed.\n\n\0" as *const u8 as *const libc::c_char);
    }
    Ok(())
}
pub unsafe fn command_connect(
    command: &mut Command,
    parameter: &mut Parameter,
) -> anyhow::Result<Session> {
    let mut server_fd: libc::c_int = 0;
    if command.count as libc::c_int > 1 as libc::c_int {
        parameter.server_name = CStr::from_ptr(command.text[1 as libc::c_int as usize])
            .to_str()
            .unwrap()
            .to_owned();
    }
    if command.count as libc::c_int > 2 as libc::c_int {
        parameter.server_port = extc::atoi(command.text[2 as libc::c_int as usize]) as u16;
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
    if parameter.verbose_yn != 0 {
        extc::printf(b"Connected.\n\n\0" as *const u8 as *const libc::c_char);
    }
    Ok(session)
}
pub unsafe fn command_dir(_command: &mut Command, session: &mut Session) -> anyhow::Result<()> {
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
    crate::common::common::fread_line(
        session.server,
        &mut *read_str.as_mut_ptr().offset(1 as libc::c_int as isize),
        (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    num_files = extc::atoi(read_str.as_mut_ptr()) as u16;
    extc::fprintf(
        extc::stderr,
        b"Remote file list:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as u16;
    while (i as libc::c_int) < num_files as libc::c_int {
        crate::common::common::fread_line(
            session.server,
            read_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        extc::fprintf(
            extc::stderr,
            b" %2d) %-64s\0" as *const u8 as *const libc::c_char,
            i as libc::c_int + 1 as libc::c_int,
            read_str.as_mut_ptr(),
        );
        crate::common::common::fread_line(
            session.server,
            read_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        filelen = extc::atol(read_str.as_mut_ptr()) as usize;
        extc::fprintf(
            extc::stderr,
            b"%8Lu bytes\n\0" as *const u8 as *const libc::c_char,
            filelen as u64,
        );
        i = i.wrapping_add(1);
        i;
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
    command: &mut Command,
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
    if (command.count as libc::c_int) < 2 as libc::c_int {
        bail!("Invalid command syntax (use 'help get' for details)");
    }
    if (session.server).is_null() {
        bail!("Not connected to a Tsunami server");
    }

    session.transfer = Transfer::default();

    if extc::strcmp(
        b"*\0" as *const u8 as *const libc::c_char,
        command.text[1 as libc::c_int as usize],
    ) == 0
    {
        let mut filearray_size: [libc::c_char; 10] = [0; 10];
        let mut file_count: [libc::c_char; 10] = [0; 10];
        multimode = 1 as libc::c_int;
        extc::printf(b"Requesting all available files\n\0" as *const u8 as *const libc::c_char);
        extc::gettimeofday(&mut ping_s, std::ptr::null_mut::<libc::c_void>());
        status = extc::fprintf(
            session.server,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            command.text[1 as libc::c_int as usize],
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
                crate::common::common::fread_line(
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
                f_counter;
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
        if multimode == 0 {
            session.transfer.remote_filename = command.text[1 as libc::c_int as usize];
        } else {
            session.transfer.remote_filename = *file_names.offset(f_counter as isize);
        }
        if multimode == 0 {
            if command.count as libc::c_int >= 3 as libc::c_int {
                session.transfer.local_filename = command.text[2 as libc::c_int as usize];
            } else {
                session.transfer.local_filename =
                    extc::strrchr(command.text[1 as libc::c_int as usize], '/' as i32);
                if (session.transfer.local_filename).is_null() {
                    session.transfer.local_filename = command.text[1 as libc::c_int as usize];
                } else {
                    session.transfer.local_filename = (session.transfer.local_filename).offset(1);
                }
            }
        } else {
            session.transfer.local_filename = *file_names.offset(f_counter as isize);
            extc::printf(
                b"GET *: now requesting file '%s'\n\0" as *const u8 as *const libc::c_char,
                session.transfer.local_filename,
            );
        }
        super::protocol::ttp_open_transfer_client(
            session,
            parameter,
            session.transfer.remote_filename,
            session.transfer.local_filename,
        )?;

        super::protocol::ttp_open_port_client(session, parameter)?;
        session.transfer.retransmit.table = extc::calloc(
            super::config::DEFAULT_TABLE_SIZE as libc::c_ulong,
            ::core::mem::size_of::<u32>() as libc::c_ulong,
        ) as *mut u32;
        if (session.transfer.retransmit.table).is_null() {
            panic!("Could not allocate retransmission table");
        }
        session.transfer.received = extc::calloc(
            (session.transfer.block_count / 8 as libc::c_int as u32)
                .wrapping_add(2 as libc::c_int as u32) as libc::c_ulong,
            ::core::mem::size_of::<u8>() as libc::c_ulong,
        ) as *mut u8;
        if (session.transfer.received).is_null() {
            panic!("Could not allocate received-data bitfield");
        }
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

        session.transfer.stats.start_udp_errors = crate::common::common::get_udp_in_errors();
        session.transfer.stats.this_udp_errors = session.transfer.stats.start_udp_errors;
        extc::gettimeofday(
            &mut session.transfer.stats.start_time,
            std::ptr::null_mut::<libc::c_void>(),
        );
        extc::gettimeofday(
            &mut session.transfer.stats.this_time,
            std::ptr::null_mut::<libc::c_void>(),
        );
        if parameter.transcript_yn != 0 {
            super::transcript::xscript_data_start_client(
                session,
                parameter,
                session.transfer.stats.start_time,
            );
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
                && (got_block(session, this_block) == 0
                    || this_type as libc::c_int == 'X' as i32
                    || session.transfer.restart_pending as libc::c_int != 0)
            {
                if got_block(session, this_block) == 0 {
                    session
                        .transfer
                        .ring_buffer
                        .as_mut()
                        .unwrap()
                        .reserve(local_datagram_view);
                    session.transfer.ring_buffer.as_mut().unwrap().confirm();

                    let fresh1 = &mut (*(session.transfer.received)
                        .offset((this_block / 8 as libc::c_int as u32) as isize));
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
                            if parameter.lossless == 0 {
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
                        ) != 0
                            && session.transfer.gapless_to_block < session.transfer.block_count
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
                            if parameter.lossless == 0
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
            if crate::common::common::get_usec_since(&mut session.transfer.stats.this_time)
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
                if parameter.blockdump != 0 {
                    let mut postfix: [libc::c_char; 64] = [0; 64];
                    let fresh2 = dumpcount;
                    dumpcount = dumpcount.wrapping_add(1);
                    extc::snprintf(
                        postfix.as_mut_ptr(),
                        63 as libc::c_int as libc::c_ulong,
                        b".bmap%u\0" as *const u8 as *const libc::c_char,
                        fresh2,
                    );
                    dump_blockmap(postfix.as_mut_ptr(), &session.transfer);
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
            delta = crate::common::common::get_usec_since(&mut session.transfer.stats.start_time);
            session.transfer.stats.total_lost = 0 as libc::c_int as u32;
            block = 1 as libc::c_int as u32;
            while block <= session.transfer.block_count {
                if got_block(session, block) == 0 {
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
            if parameter.lossless != 0 {
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
            if parameter.transcript_yn != 0 {
                super::transcript::xscript_data_stop_client(
                    session,
                    parameter,
                    session.transfer.stats.stop_time,
                );
                super::transcript::xscript_close_client(session, parameter, delta);
            }
            if parameter.blockdump != 0 {
                dump_blockmap(
                    b".blockmap\0" as *const u8 as *const libc::c_char,
                    &session.transfer,
                );
            }
            if !(session.transfer.retransmit.table).is_null() {
                extc::free(session.transfer.retransmit.table as *mut libc::c_void);
                session.transfer.retransmit.table = std::ptr::null_mut::<u32>();
            }
            if !(session.transfer.received).is_null() {
                extc::free(session.transfer.received as *mut libc::c_void);
                session.transfer.received = std::ptr::null_mut::<u8>();
            }
            if parameter.rate_adjust != 0 {
                parameter.target_rate = (1.15f64 * 1e6f64 * (mbit_file / time_secs)) as u32;
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
            if !(session.transfer.received).is_null() {
                extc::free(session.transfer.received as *mut libc::c_void);
                session.transfer.received = std::ptr::null_mut::<u8>();
            }
            bail!("Transfer unsuccessful");
        }
        _ => {
            if multimode != 0 {
                f_counter = 0 as libc::c_int as u32;
                while f_counter < f_total {
                    extc::free(*file_names.offset(f_counter as isize) as *mut libc::c_void);
                    f_counter = f_counter.wrapping_add(1);
                    f_counter;
                }
                extc::free(file_names as *mut libc::c_void);
            }
            Ok(())
        }
    }
}
pub unsafe fn command_help(command: &mut Command) -> anyhow::Result<()> {
    if (command.count as libc::c_int) < 2 as libc::c_int {
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
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"close\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        extc::printf(b"Usage: close\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Closes the current connection to a remote Tsunami server.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"connect\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
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
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"get\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
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
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"dir\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        extc::printf(b"Usage: dir\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Attempts to list the available remote files.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"help\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        extc::printf(
            b"Come on.  You know what that command does.\n\n\0" as *const u8 as *const libc::c_char,
        );
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"quit\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        extc::printf(b"Usage: quit\n\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"Closes any open connection to a remote Tsunami server and exits the\n\0" as *const u8
                as *const libc::c_char,
        );
        extc::printf(b"Tsunami client.\n\n\0" as *const u8 as *const libc::c_char);
    } else if extc::strcasecmp(
        command.text[1 as libc::c_int as usize],
        b"set\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
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
        extc::printf(
            b"'%s' is not a recognized command.\n\0" as *const u8 as *const libc::c_char,
            command.text[1 as libc::c_int as usize],
        );
        extc::printf(
            b"Use 'help' for a list of commands.\n\n\0" as *const u8 as *const libc::c_char,
        );
    }
    Ok(())
}
pub unsafe fn command_quit(session: &mut Session) -> libc::c_int {
    if !(session.server).is_null() {
        extc::fclose(session.server);
    }
    extc::printf(b"Thank you for using Tsunami.\n\0" as *const u8 as *const libc::c_char);
    extc::printf(
        b"The ANML web site can be found at:    http://www.anml.iu.edu/\n\0" as *const u8
            as *const libc::c_char,
    );
    extc::printf(
        b"The SourceForge site can be found at: http://tsunami-udp.sf.net/\n\n\0" as *const u8
            as *const libc::c_char,
    );
    extc::exit(1 as libc::c_int);
}
pub unsafe fn command_set(command: &mut Command, parameter: &mut Parameter) -> anyhow::Result<()> {
    let mut do_all: libc::c_int = (command.count as libc::c_int == 1 as libc::c_int) as libc::c_int;
    if command.count as libc::c_int == 3 as libc::c_int {
        if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"server\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.server_name = CStr::from_ptr(command.text[2 as libc::c_int as usize])
                .to_str()
                .unwrap()
                .to_owned();
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"port\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.server_port = extc::atoi(command.text[2 as libc::c_int as usize]) as u16;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"udpport\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.client_port = extc::atoi(command.text[2 as libc::c_int as usize]) as u16;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"buffer\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.udp_buffer = extc::atol(command.text[2 as libc::c_int as usize]) as u32;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"blocksize\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.block_size = extc::atol(command.text[2 as libc::c_int as usize]) as u32;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"verbose\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.verbose_yn = (extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"transcript\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.transcript_yn = (extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"ip\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.ipv6_yn = (extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"v6\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"output\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.output_mode = (if extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"screen\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"rateadjust\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.rate_adjust = (extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"rate\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut multiplier: libc::c_long = 1 as libc::c_int as libc::c_long;
            let mut cmd: *mut libc::c_char =
                command.text[2 as libc::c_int as usize] as *mut libc::c_char;
            let mut cpy: [libc::c_char; 256] = [0; 256];
            let mut l: libc::c_int = extc::strlen(cmd) as libc::c_int;
            extc::strcpy(cpy.as_mut_ptr(), cmd);
            if l > 1 as libc::c_int
                && ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        __res = extc::toupper(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int);
                    } else {
                        __res = *(*extc::__ctype_toupper_loc())
                            .offset(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int as isize);
                    }
                    __res
                }) == 'M' as i32
            {
                multiplier = 1000000 as libc::c_int as libc::c_long;
                cpy[(l - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            } else if l > 1 as libc::c_int
                && ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        __res = extc::toupper(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int);
                    } else {
                        __res = *(*extc::__ctype_toupper_loc())
                            .offset(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int as isize);
                    }
                    __res
                }) == 'G' as i32
            {
                multiplier = 1000000000 as libc::c_int as libc::c_long;
                cpy[(l - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            }
            parameter.target_rate = (multiplier * extc::atol(cpy.as_mut_ptr())) as u32;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"error\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.error_rate =
                (extc::atof(command.text[2 as libc::c_int as usize]) * 1000.0f64) as u32;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"slowdown\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parse_fraction(
                command.text[2 as libc::c_int as usize],
                &mut parameter.slower_num,
                &mut parameter.slower_den,
            );
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"speedup\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parse_fraction(
                command.text[2 as libc::c_int as usize],
                &mut parameter.faster_num,
                &mut parameter.faster_den,
            );
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"history\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.history = extc::atoi(command.text[2 as libc::c_int as usize]) as u16;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"lossless\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.lossless = (extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"losswindow\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.losswindow_ms = extc::atol(command.text[2 as libc::c_int as usize]) as u32;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"blockdump\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.blockdump = (extc::strcmp(
                command.text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u8;
        } else if extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"passphrase\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parameter.passphrase = Some(
                CStr::from_ptr(command.text[2 as libc::c_int as usize])
                    .to_str()
                    .unwrap()
                    .to_owned(),
            );
        }
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"server\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        println!("server = {}", parameter.server_name);
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"port\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"port = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.server_port as libc::c_int,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"udpport\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"udpport = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.client_port as libc::c_int,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"buffer\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"buffer = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.udp_buffer,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"blocksize\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"blocksize = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.block_size,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"verbose\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"verbose = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.verbose_yn as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"transcript\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"transcript = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.transcript_yn as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"ip\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"ip = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.ipv6_yn as libc::c_int != 0 {
                b"v6\0" as *const u8 as *const libc::c_char
            } else {
                b"v4\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"output\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"output = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.output_mode as libc::c_int == 0 as libc::c_int {
                b"screen\0" as *const u8 as *const libc::c_char
            } else {
                b"line\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"rate\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"rate = %u\n\0" as *const u8 as *const libc::c_char,
            parameter.target_rate,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"rateadjust\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"rateadjust = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.rate_adjust as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"error\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"error = %0.2f%%\n\0" as *const u8 as *const libc::c_char,
            parameter.error_rate as libc::c_double / 1000.0f64,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"slowdown\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"slowdown = %d/%d\n\0" as *const u8 as *const libc::c_char,
            parameter.slower_num as libc::c_int,
            parameter.slower_den as libc::c_int,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"speedup\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"speedup = %d/%d\n\0" as *const u8 as *const libc::c_char,
            parameter.faster_num as libc::c_int,
            parameter.faster_den as libc::c_int,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"history\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"history = %d%%\n\0" as *const u8 as *const libc::c_char,
            parameter.history as libc::c_int,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"lossless\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"lossless = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.lossless as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"losswindow\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"losswindow = %d msec\n\0" as *const u8 as *const libc::c_char,
            parameter.losswindow_ms,
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"blockdump\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        extc::printf(
            b"blockdump = %s\n\0" as *const u8 as *const libc::c_char,
            if parameter.blockdump as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || extc::strcasecmp(
            command.text[1 as libc::c_int as usize],
            b"passphrase\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
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

pub unsafe fn parse_fraction(
    mut fraction: *const libc::c_char,
    mut num: *mut u16,
    mut den: *mut u16,
) -> anyhow::Result<()> {
    let mut slash: *const libc::c_char = std::ptr::null::<libc::c_char>();
    slash = extc::strchr(fraction, '/' as i32);
    if slash.is_null() {
        bail!("Value is not a fraction");
    }
    *num = extc::atoi(fraction) as u16;
    *den = extc::atoi(slash.offset(1 as libc::c_int as isize)) as u16;
    Ok(())
}
pub unsafe fn got_block(session: &mut Session, mut blocknr: u32) -> libc::c_int {
    if blocknr > session.transfer.block_count {
        return 1 as libc::c_int;
    }
    *(session.transfer.received).offset((blocknr / 8 as libc::c_int as u32) as isize) as libc::c_int
        & (1 as libc::c_int) << (blocknr % 8 as libc::c_int as u32)
}
pub unsafe fn dump_blockmap(mut postfix: *const libc::c_char, xfer: &Transfer) {
    let mut fbits: *mut extc::FILE = std::ptr::null_mut::<extc::FILE>();
    let mut fname: *mut libc::c_char = std::ptr::null_mut::<libc::c_char>();
    fname = extc::calloc(
        (extc::strlen(xfer.local_filename))
            .wrapping_add(extc::strlen(postfix))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<u8>() as libc::c_ulong,
    ) as *mut libc::c_char;
    extc::strcpy(fname, xfer.local_filename);
    extc::strcat(fname, postfix);
    fbits = extc::fopen(fname, b"wb\0" as *const u8 as *const libc::c_char);
    if !fbits.is_null() {
        extc::fwrite(
            &xfer.block_count as *const u32 as *const libc::c_void,
            ::core::mem::size_of::<u32>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fbits,
        );
        extc::fwrite(
            xfer.received as *const libc::c_void,
            ::core::mem::size_of::<u8>() as libc::c_ulong,
            (xfer.block_count / 8 as libc::c_int as u32).wrapping_add(1 as libc::c_int as u32)
                as libc::c_ulong,
            fbits,
        );
        extc::fclose(fbits);
    } else {
        extc::fprintf(
            extc::stderr,
            b"Could not create a file for the blockmap dump\0" as *const u8 as *const libc::c_char,
        );
    }
    extc::free(fname as *mut libc::c_void);
}
