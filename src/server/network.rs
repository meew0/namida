use std::{
    net::{TcpListener, ToSocketAddrs, UdpSocket},
    os::fd::AsRawFd,
};

use ::libc;
use anyhow::bail;

use super::Parameter;

pub fn create_tcp_socket_server(parameter: &Parameter) -> anyhow::Result<TcpListener> {
    let listener = TcpListener::bind(&parameter.bind)?;
    Ok(listener)
}

pub unsafe fn create_udp_socket_server(parameter: &Parameter) -> anyhow::Result<UdpSocket> {
    let ipv6 = parameter.bind.to_socket_addrs()?.next().unwrap().is_ipv6();
    let catch_all_host = crate::common::catch_all_host(ipv6);

    let mut socket = UdpSocket::bind((catch_all_host, 0))?;

    // TODO: “make reusable”

    set_udp_transmit_buffer(&mut socket, parameter.udp_buffer)?;

    Ok(socket)
}

pub fn set_udp_transmit_buffer(
    socket: &mut UdpSocket,
    transmit_buffer_size: u32,
) -> anyhow::Result<()> {
    // TODO: cross platform
    unsafe {
        let status = libc::setsockopt(
            socket.as_raw_fd(),
            1,
            7,
            &transmit_buffer_size as *const u32 as *const libc::c_void,
            ::core::mem::size_of::<u32>() as libc::c_ulong as libc::socklen_t,
        );
        if status < 0 as libc::c_int {
            bail!("Could not resize UDP transmit buffer");
        }
    }

    Ok(())
}
