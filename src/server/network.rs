use std::net::{TcpListener, ToSocketAddrs};

use crate::extc;
use ::libc;
use anyhow::bail;

use super::Parameter;

pub fn create_tcp_socket_server(parameter: &Parameter) -> anyhow::Result<TcpListener> {
    let listener = TcpListener::bind(&parameter.bind)?;
    Ok(listener)
}

pub unsafe fn create_udp_socket_server(parameter: &Parameter) -> anyhow::Result<i32> {
    let ipv6 = parameter.bind.to_socket_addrs()?.next().unwrap().is_ipv6();

    let mut socket_fd: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut yes: libc::c_int = 1 as libc::c_int;
    socket_fd = extc::socket(
        if ipv6 {
            10 as libc::c_int
        } else {
            2 as libc::c_int
        },
        extc::SOCK_DGRAM as libc::c_int,
        0 as libc::c_int,
    );
    if socket_fd < 0 as libc::c_int {
        bail!("Error in creating UDP socket");
    }
    status = extc::setsockopt(
        socket_fd,
        1 as libc::c_int,
        2 as libc::c_int,
        &mut yes as *mut libc::c_int as *const libc::c_void,
        ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as extc::socklen_t,
    );
    if status < 0 as libc::c_int {
        extc::close(socket_fd);
        bail!("Error in configuring UDP socket");
    }
    status = extc::setsockopt(
        socket_fd,
        1 as libc::c_int,
        7 as libc::c_int,
        &parameter.udp_buffer as *const u32 as *const libc::c_void,
        ::core::mem::size_of::<u32>() as libc::c_ulong as extc::socklen_t,
    );
    if status < 0 as libc::c_int {
        println!("WARNING: Error in resizing UDP transmit buffer");
    }
    Ok(socket_fd)
}
