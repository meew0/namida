use std::{
    ffi::{CStr, CString},
    io::{Seek, SeekFrom},
    path::Path,
};

use crate::{extc, types::Retransmission};
use ::libc;
use anyhow::bail;

use super::{Parameter, Session, Transfer};

pub unsafe fn ttp_accept_retransmit(
    session: &mut Session,
    parameter: &Parameter,
    retransmission: &mut Retransmission,
    datagram_block_buffer: &mut [u8],
    datagram_buffer: &mut [u8],
) -> anyhow::Result<()> {
    static mut iteration: libc::c_int = 0 as libc::c_int;
    static mut stats_line: [libc::c_char; 80] = [0; 80];
    let mut status: libc::c_int = 0;
    let mut type_0: u16 = 0;
    retransmission.block = extc::__bswap_32(retransmission.block);
    retransmission.error_rate = extc::__bswap_32(retransmission.error_rate);
    type_0 = extc::__bswap_16(retransmission.request_type);
    if type_0 as libc::c_int == crate::common::REQUEST_ERROR_RATE as libc::c_int {
        if retransmission.error_rate > parameter.error_rate {
            let mut factor1: libc::c_double = 1.0f64
                * parameter.slower_num as libc::c_int as libc::c_double
                / parameter.slower_den as libc::c_int as libc::c_double
                - 1.0f64;
            let mut factor2: libc::c_double = (1.0f64
                + retransmission.error_rate as libc::c_double
                - parameter.error_rate as libc::c_double)
                / (100000.0f64 - parameter.error_rate as libc::c_double);
            session.transfer.ipd_current *= 1.0f64 + factor1 * factor2;
        } else {
            session.transfer.ipd_current *= parameter.faster_num as libc::c_double
                / parameter.faster_den as libc::c_int as libc::c_double;
        }
        session.transfer.ipd_current = if (if session.transfer.ipd_current < 10000.0f64 {
            session.transfer.ipd_current
        } else {
            10000.0f64
        }) > parameter.ipd_time as libc::c_double
        {
            if session.transfer.ipd_current < 10000.0f64 {
                session.transfer.ipd_current
            } else {
                10000.0f64
            }
        } else {
            parameter.ipd_time as libc::c_double
        };
        extc::sprintf(
            stats_line.as_mut_ptr(),
            b"%6u %3.2fus %5uus %7u %6.2f %3u\n\0" as *const u8 as *const libc::c_char,
            retransmission.error_rate,
            session.transfer.ipd_current as libc::c_float as libc::c_double,
            parameter.ipd_time,
            session.transfer.block,
            100.0f64 * session.transfer.block as libc::c_double
                / parameter.block_count as libc::c_double,
            session.session_id,
        );
        let fresh0 = iteration;
        iteration += 1;
        if fresh0 % 23 as libc::c_int == 0 {
            extc::printf(
                b" erate     ipd  target   block   %%done srvNr\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        extc::printf(
            b"%s\0" as *const u8 as *const libc::c_char,
            stats_line.as_mut_ptr(),
        );
        if parameter.transcript_yn {
            crate::common::transcript_warn_error(super::transcript::xscript_data_log_server(
                session,
                CStr::from_ptr(stats_line.as_mut_ptr()).to_str().unwrap(),
            ));
        }
    } else if type_0 as libc::c_int == crate::common::REQUEST_RESTART as libc::c_int {
        if retransmission.block == 0 as libc::c_int as u32
            || retransmission.block > parameter.block_count
        {
            bail!(
                "Attempt to restart at illegal block {}",
                retransmission.block
            );
        } else {
            session.transfer.block = retransmission.block;
        }
    } else if type_0 as libc::c_int == crate::common::REQUEST_RETRANSMIT as libc::c_int {
        let datagram = super::io::build_datagram(
            session,
            parameter,
            retransmission.block,
            'R' as i32 as u16,
            datagram_block_buffer,
        )?;
        datagram.write_to(datagram_buffer);

        status = extc::sendto(
            session.transfer.udp_fd,
            datagram_buffer.as_ptr() as *const libc::c_void,
            (6 as libc::c_int as u32).wrapping_add(parameter.block_size) as u64,
            0 as libc::c_int,
            extc::__CONST_SOCKADDR_ARG {
                __sockaddr__: session.transfer.udp_address,
            },
            session.transfer.udp_length,
        ) as libc::c_int;
        if status < 0 as libc::c_int {
            bail!("Could not retransmit block {}", retransmission.block,);
        }
    } else {
        bail!(
            "Received unknown retransmission request of type {}",
            extc::__bswap_16(retransmission.request_type) as libc::c_int,
        );
    }
    Ok(())
}
pub unsafe fn ttp_authenticate_server(
    session: &mut Session,
    mut secret: &[u8],
) -> anyhow::Result<()> {
    use rand::Rng;

    let mut random: [u8; 64] = [0; 64];
    let mut _server_digest: [u8; 16] = [0; 16];
    let mut client_digest: [u8; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;

    rand::thread_rng().fill(&mut random);

    status = crate::common::full_write(
        session.client_fd,
        random.as_mut_ptr() as *const libc::c_void,
        64 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not send authentication challenge to client");
    }
    status = crate::common::full_read(
        session.client_fd,
        client_digest.as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not read authentication response from client");
    }
    let server_digest: [u8; 16] = crate::common::prepare_proof(&mut random, secret).into();
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if client_digest[i as usize] as libc::c_int != server_digest[i as usize] as libc::c_int {
            crate::common::full_write(
                session.client_fd,
                b"\x01\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as u64,
            );
            bail!("Authentication failed");
        }
        i += 1;
    }
    status = crate::common::full_write(
        session.client_fd,
        b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not send authentication confirmation to client");
    }
    Ok(())
}
pub unsafe fn ttp_negotiate_server(session: &mut Session) -> anyhow::Result<()> {
    let mut server_revision: u32 = extc::__bswap_32(crate::common::PROTOCOL_REVISION);
    let mut client_revision: u32 = 0;
    let mut status: libc::c_int = 0;
    status = crate::common::full_write(
        session.client_fd,
        &mut server_revision as *mut u32 as *const libc::c_void,
        4 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not send protocol revision number");
    }
    status = crate::common::full_read(
        session.client_fd,
        &mut client_revision as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not read protocol revision number");
    }
    if client_revision != server_revision {
        bail!("Protocol negotiation failed");
    };
    Ok(())
}
pub unsafe fn ttp_open_port_server(
    session: &mut Session,
    parameter: &mut Parameter,
) -> anyhow::Result<()> {
    let mut address: *mut extc::sockaddr = std::ptr::null_mut::<extc::sockaddr>();
    let mut status: libc::c_int = 0;
    let mut port: u16 = 0;
    let mut ipv6_yn: bool = parameter.ipv6_yn;
    if let Some(client) = &parameter.client {
        let mut result: *mut extc::addrinfo = std::ptr::null_mut::<extc::addrinfo>();
        let mut _errmsg: [libc::c_char; 256] = [0; 256];
        let client_c = CString::new(client.as_str()).unwrap();
        let mut status_0: libc::c_int = extc::getaddrinfo(
            client_c.as_ptr(),
            std::ptr::null::<libc::c_char>(),
            std::ptr::null::<extc::addrinfo>(),
            &mut result,
        );
        if status_0 != 0 {
            bail!(
                "error in getaddrinfo: {}",
                extc::gai_strerror_wrap(status_0),
            );
        }
        ipv6_yn = (*result).ai_family == 10 as libc::c_int;
        parameter.ipv6_yn = ipv6_yn;
        session.transfer.udp_length = (*result).ai_addrlen;
        address = extc::malloc((*result).ai_addrlen as libc::c_ulong) as *mut extc::sockaddr;
        if address.is_null() {
            panic!("Could not allocate space for UDP socket address");
        }
        extc::memcpy(
            address as *mut libc::c_void,
            (*result).ai_addr as *const libc::c_void,
            (*result).ai_addrlen as libc::c_ulong,
        );
        if !((*result).ai_canonname).is_null() {
            extc::printf(
                b"Sending data to: %s\n\0" as *const u8 as *const libc::c_char,
                (*result).ai_canonname,
            );
        }
        extc::freeaddrinfo(result);
    } else {
        session.transfer.udp_length = (if ipv6_yn as libc::c_int != 0 {
            ::core::mem::size_of::<extc::sockaddr_in6>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<extc::sockaddr_in>() as libc::c_ulong
        }) as extc::socklen_t;
        address = extc::malloc(session.transfer.udp_length as libc::c_ulong) as *mut extc::sockaddr;
        if address.is_null() {
            panic!("Could not allocate space for UDP socket address");
        }
        extc::getpeername(
            session.client_fd,
            extc::__SOCKADDR_ARG {
                __sockaddr__: address,
            },
            &mut session.transfer.udp_length,
        );
    }

    status = crate::common::full_read(
        session.client_fd,
        &mut port as *mut u16 as *mut libc::c_void,
        2 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not read UDP port number");
    }
    if ipv6_yn {
        (*(address as *mut extc::sockaddr_in6)).sin6_port = port;
    } else {
        (*(address as *mut extc::sockaddr_in)).sin_port = port;
    }
    if parameter.verbose_yn {
        extc::printf(
            b"Sending to client port %d\n\0" as *const u8 as *const libc::c_char,
            extc::__bswap_16(port) as libc::c_int,
        );
    }
    session.transfer.udp_fd = super::network::create_udp_socket_server(parameter)?;
    session.transfer.udp_address = address;
    Ok(())
}
pub unsafe fn ttp_open_transfer_server(
    session: &mut Session,
    parameter: &mut Parameter,
) -> anyhow::Result<()> {
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut file_size: u64 = 0;
    let mut block_size: u32 = 0;
    let mut block_count: u32 = 0;
    let mut epoch: extc::time_t = 0;
    let mut status: libc::c_int = 0;
    let mut size: [libc::c_char; 10] = [0; 10];
    let mut file_no: [libc::c_char; 10] = [0; 10];
    let mut message: [libc::c_char; 20] = [0; 20];
    let mut i: u16 = 0;
    let mut ping_s: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ping_e: extc::timeval = extc::timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    session.transfer = Transfer::default();
    crate::common::read_line(
        session.client_fd,
        filename.as_mut_ptr(),
        1024 as libc::c_int as usize,
    )
    .unwrap();
    filename[(1024 as libc::c_int - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
    if extc::strcmp(
        filename.as_mut_ptr(),
        b"!#DIR??\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        extc::snprintf(
            file_no.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            parameter.files.len() as libc::c_int,
        );
        crate::common::full_write(
            session.client_fd,
            file_no.as_mut_ptr() as *const libc::c_void,
            (extc::strlen(file_no.as_mut_ptr())).wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        for (file_name, file_size) in &parameter.files {
            let bytes = file_name.as_os_str().as_encoded_bytes();
            let mut null_terminated = vec![0; bytes.len() + 1];
            null_terminated[0..(bytes.len())].copy_from_slice(bytes);

            crate::common::full_write(
                session.client_fd,
                null_terminated.as_ptr() as *const libc::c_void,
                null_terminated.len() as u64,
            );
            extc::snprintf(
                message.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b"%Lu\0" as *const u8 as *const libc::c_char,
                *file_size,
            );
            crate::common::full_write(
                session.client_fd,
                message.as_mut_ptr() as *const libc::c_void,
                (extc::strlen(message.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
        }
        crate::common::full_read(
            session.client_fd,
            message.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as u64,
        );
        bail!("File list sent!");
    } else if extc::strcmp(
        filename.as_mut_ptr(),
        b"*\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        if let Some(allhook) = &parameter.allhook {
            let MaxFileListLength: libc::c_int = 32768 as libc::c_int;
            let vla = MaxFileListLength as usize;
            let mut fileList: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
            let mut fl: *const libc::c_char = std::ptr::null::<libc::c_char>();
            let mut nFile: libc::c_int = 0 as libc::c_int;
            let mut length: libc::c_int = 0 as libc::c_int;
            let mut l: libc::c_int = 0;
            let mut p: *mut extc::FILE = std::ptr::null_mut::<extc::FILE>();
            eprintln!("Using allhook program: {}", allhook);
            let allhook_c = CString::new(allhook.as_str()).unwrap();
            p = extc::popen(
                allhook_c.as_ptr(),
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if !p.is_null() {
                extc::memset(
                    fileList.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    MaxFileListLength as libc::c_ulong,
                );
                while !(extc::fgets(
                    message.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong as libc::c_int,
                    p,
                ))
                .is_null()
                {
                    l = 0 as libc::c_int;
                    while message[l as usize] as libc::c_int >= ' ' as i32 {
                        l += 1;
                    }
                    message[l as usize] = 0 as libc::c_int as libc::c_char;
                    extc::fprintf(
                        extc::stdout,
                        b"  '%s'\n\0" as *const u8 as *const libc::c_char,
                        message.as_mut_ptr(),
                    );
                    if l + length >= MaxFileListLength {
                        break;
                    }
                    extc::strncpy(
                        fileList.as_mut_ptr().offset(length as isize),
                        message.as_mut_ptr(),
                        l as libc::c_ulong,
                    );
                    length += l + 1 as libc::c_int;
                    nFile += 1;
                }
            }
            extc::pclose(p);
            extc::memset(
                size.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            extc::snprintf(
                size.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                length,
            );
            crate::common::full_write(
                session.client_fd,
                size.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as u64,
            );
            extc::memset(
                file_no.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            extc::snprintf(
                file_no.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                nFile,
            );
            crate::common::full_write(
                session.client_fd,
                file_no.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as u64,
            );
            extc::printf(
                b"\nSent multi-GET filename count and array size to client\n\0" as *const u8
                    as *const libc::c_char,
            );
            extc::memset(
                message.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
            crate::common::full_read(
                session.client_fd,
                message.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as u64,
            );
            extc::printf(
                b"Client response: %s\n\0" as *const u8 as *const libc::c_char,
                message.as_mut_ptr(),
            );
            fl = fileList.as_mut_ptr();
            if nFile > 0 as libc::c_int {
                i = 0 as libc::c_int as u16;
                while (i as libc::c_int) < nFile {
                    l = extc::strlen(fl) as libc::c_int;
                    crate::common::full_write(
                        session.client_fd,
                        fl as *const libc::c_void,
                        (l + 1 as libc::c_int) as u64,
                    );
                    fl = fl.offset((l + 1 as libc::c_int) as isize);
                    i = i.wrapping_add(1);
                }
                extc::memset(
                    message.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                );
                crate::common::full_read(
                    session.client_fd,
                    message.as_mut_ptr() as *mut libc::c_void,
                    8 as libc::c_int as u64,
                );
                extc::printf(
                    b"Sent file list, client response: %s\n\0" as *const u8 as *const libc::c_char,
                    message.as_mut_ptr(),
                );
                crate::common::read_line(
                    session.client_fd,
                    filename.as_mut_ptr(),
                    1024 as libc::c_int as usize,
                )?;
            }
        } else {
            extc::memset(
                size.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            extc::snprintf(
                size.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                parameter.file_name_size as libc::c_int,
            );
            crate::common::full_write(
                session.client_fd,
                size.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as u64,
            );
            extc::memset(
                file_no.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            extc::snprintf(
                file_no.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                parameter.file_names.len() as libc::c_int,
            );
            crate::common::full_write(
                session.client_fd,
                file_no.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as u64,
            );
            extc::printf(
                b"\nSent multi-GET filename count and array size to client\n\0" as *const u8
                    as *const libc::c_char,
            );
            extc::memset(
                message.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
            crate::common::full_read(
                session.client_fd,
                message.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as u64,
            );
            extc::printf(
                b"Client response: %s\n\0" as *const u8 as *const libc::c_char,
                message.as_mut_ptr(),
            );

            for file_name in &parameter.file_names {
                let bytes = file_name.as_os_str().as_encoded_bytes();
                let mut null_terminated = vec![0; bytes.len() + 1];
                null_terminated[0..(bytes.len())].copy_from_slice(bytes);

                crate::common::full_write(
                    session.client_fd,
                    null_terminated.as_ptr() as *const libc::c_void,
                    null_terminated.len() as u64,
                );
            }

            extc::memset(
                message.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
            crate::common::full_read(
                session.client_fd,
                message.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as u64,
            );
            extc::printf(
                b"Sent file list, client response: %s\n\0" as *const u8 as *const libc::c_char,
                message.as_mut_ptr(),
            );
            crate::common::read_line(
                session.client_fd,
                filename.as_mut_ptr(),
                1024 as libc::c_int as usize,
            )?;
        }
    }

    let filename_rust = session
        .transfer
        .filename
        .insert(extc::c_to_string(filename.as_mut_ptr()));
    if parameter.verbose_yn {
        extc::printf(
            b"Request for file: '%s'\n\0" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }

    let path = Path::new(filename_rust);
    let file = match std::fs::File::open(path) {
        Ok(opened_file) => session.transfer.file.insert(opened_file),
        Err(err) => {
            status = crate::common::full_write(
                session.client_fd,
                b"\x08\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as u64,
            ) as libc::c_int;
            if status < 0 as libc::c_int {
                println!("WARNING: Could not signal request failure to client");
            }
            bail!(
                "File '{}' does not exist or cannot be read: {}",
                filename_rust,
                err
            );
        }
    };

    extc::gettimeofday(&mut ping_s, std::ptr::null_mut::<libc::c_void>());
    status = crate::common::full_write(
        session.client_fd,
        b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as u64,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not signal request approval to client");
    }
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.block_size as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read block size");
    }
    parameter.block_size = extc::__bswap_32(parameter.block_size);
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.target_rate as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read target bitrate");
    }
    parameter.target_rate = extc::__bswap_32(parameter.target_rate);
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.error_rate as *mut u32 as *mut libc::c_void,
        4 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read error rate");
    }
    parameter.error_rate = extc::__bswap_32(parameter.error_rate);
    extc::gettimeofday(&mut ping_e, std::ptr::null_mut::<libc::c_void>());
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.slower_num as *mut u16 as *mut libc::c_void,
        2 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read slowdown numerator");
    }
    parameter.slower_num = extc::__bswap_16(parameter.slower_num);
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.slower_den as *mut u16 as *mut libc::c_void,
        2 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read slowdown denominator");
    }
    parameter.slower_den = extc::__bswap_16(parameter.slower_den);
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.faster_num as *mut u16 as *mut libc::c_void,
        2 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read speedup numerator");
    }
    parameter.faster_num = extc::__bswap_16(parameter.faster_num);
    if crate::common::full_read(
        session.client_fd,
        &mut parameter.faster_den as *mut u16 as *mut libc::c_void,
        2 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not read speedup denominator");
    }
    parameter.faster_den = extc::__bswap_16(parameter.faster_den);

    parameter.file_size = file.seek(SeekFrom::End(0))?;
    file.seek(SeekFrom::Start(0))?;

    parameter.block_count = (parameter.file_size / parameter.block_size as u64).wrapping_add(
        (parameter.file_size % parameter.block_size as u64 != 0 as libc::c_int as u64)
            as libc::c_int as u64,
    ) as u32;
    parameter.epoch = extc::time(std::ptr::null_mut::<extc::time_t>());
    file_size = crate::common::htonll(parameter.file_size);
    if crate::common::full_write(
        session.client_fd,
        &mut file_size as *mut u64 as *const libc::c_void,
        8 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not submit file size");
    }
    block_size = extc::__bswap_32(parameter.block_size);
    if crate::common::full_write(
        session.client_fd,
        &mut block_size as *mut u32 as *const libc::c_void,
        4 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not submit block size");
    }
    block_count = extc::__bswap_32(parameter.block_count);
    if crate::common::full_write(
        session.client_fd,
        &mut block_count as *mut u32 as *const libc::c_void,
        4 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not submit block count");
    }
    epoch = extc::__bswap_32(parameter.epoch as u32) as extc::time_t;
    if crate::common::full_write(
        session.client_fd,
        &mut epoch as *mut extc::time_t as *const libc::c_void,
        4 as libc::c_int as u64,
    ) < 0 as libc::c_int as i64
    {
        bail!("Could not submit run epoch");
    }
    parameter.wait_u_sec = (ping_e.tv_sec - ping_s.tv_sec)
        * 1000000 as libc::c_int as extc::__time_t
        + (ping_e.tv_usec - ping_s.tv_usec);
    parameter.wait_u_sec = parameter.wait_u_sec
        + (parameter.wait_u_sec as libc::c_double * 0.1f64) as libc::c_int as libc::c_long;
    parameter.ipd_time = (1000000 as libc::c_longlong
        * 8 as libc::c_int as libc::c_longlong
        * parameter.block_size as libc::c_longlong
        / parameter.target_rate as libc::c_longlong) as u32;
    session.transfer.ipd_current = (parameter.ipd_time * 3 as libc::c_int as u32) as libc::c_double;
    if parameter.transcript_yn {
        crate::common::transcript_warn_error(super::transcript::xscript_open_server(
            session, parameter,
        ));
    }
    Ok(())
}
