use std::net::TcpStream;

use crate::extc;
use ::libc;
use anyhow::bail;
use to_socket_addrs::ToSocketAddrsWithDefaultPort;

use super::{Parameter, Session};

pub unsafe fn create_tcp_socket_client(
    session: &mut Session,
    parameter: &Parameter,
) -> anyhow::Result<TcpStream> {
    let socket_addr = parameter
        .server
        .as_str()
        .with_default_port(super::config::DEFAULT_SERVER_PORT);

    let mut socket = TcpStream::connect(socket_addr)?;
    // TODO: "make reusable" (SO_REUSEADDR)
    socket.set_nodelay(true)?;

    Ok(socket)
}

pub unsafe fn create_udp_socket_client(parameter: &mut Parameter) -> anyhow::Result<i32> {
    let mut hints: extc::addrinfo = extc::addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: std::ptr::null_mut::<extc::sockaddr>(),
        ai_canonname: std::ptr::null_mut::<libc::c_char>(),
        ai_next: std::ptr::null_mut::<extc::addrinfo>(),
    };
    let mut info: *mut extc::addrinfo = std::ptr::null_mut::<extc::addrinfo>();
    let mut info_save: *mut extc::addrinfo = std::ptr::null_mut::<extc::addrinfo>();
    let mut buffer: [libc::c_char; 10] = [0; 10];
    let mut socket_fd: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut higher_port_attempt: libc::c_int = 0 as libc::c_int;
    extc::memset(
        &mut hints as *mut extc::addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<extc::addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x1 as libc::c_int;
    hints.ai_family = if parameter.ipv6_yn as libc::c_int != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    hints.ai_socktype = extc::SOCK_DGRAM as libc::c_int;
    loop {
        extc::sprintf(
            buffer.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            parameter.client_port as libc::c_int + higher_port_attempt,
        );
        status = extc::getaddrinfo(
            std::ptr::null::<libc::c_char>(),
            buffer.as_mut_ptr(),
            &hints,
            &mut info,
        );
        if status != 0 {
            bail!("Error in getting address information");
        }
        info_save = info;
        loop {
            socket_fd = extc::socket((*info).ai_family, (*info).ai_socktype, (*info).ai_protocol);
            if socket_fd >= 0 as libc::c_int {
                status = extc::setsockopt(
                    socket_fd,
                    1 as libc::c_int,
                    8 as libc::c_int,
                    &parameter.udp_buffer as *const u32 as *const libc::c_void,
                    ::core::mem::size_of::<u32>() as libc::c_ulong as extc::socklen_t,
                );
                if status < 0 as libc::c_int {
                    println!("WARNING: Error in resizing UDP receive buffer");
                }
                status = extc::bind(
                    socket_fd,
                    extc::__CONST_SOCKADDR_ARG {
                        __sockaddr__: (*info).ai_addr,
                    },
                    (*info).ai_addrlen,
                );
                if status == 0 as libc::c_int {
                    parameter.client_port =
                        extc::__bswap_16((*((*info).ai_addr as *mut extc::sockaddr_in)).sin_port);
                    extc::fprintf(
                        extc::stderr,
                        b"Receiving data on UDP port %d\n\0" as *const u8 as *const libc::c_char,
                        parameter.client_port as libc::c_int,
                    );
                    break;
                }
            }
            info = (*info).ai_next;
            if info.is_null() {
                break;
            }
        }
        extc::freeaddrinfo(info_save);
        higher_port_attempt += 1;
        if !(higher_port_attempt < 256 as libc::c_int && info.is_null()) {
            break;
        }
    }
    if higher_port_attempt > 1 as libc::c_int {
        extc::fprintf(
            extc::stderr,
            b"Warning: there are %d other Tsunami clients running\n\0" as *const u8
                as *const libc::c_char,
            higher_port_attempt - 1 as libc::c_int,
        );
    }
    if info.is_null() {
        bail!("Error in creating UDP socket");
    }
    Ok(socket_fd)
}
