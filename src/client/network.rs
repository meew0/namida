use std::{
    io::ErrorKind,
    net::{TcpStream, UdpSocket},
    os::fd::AsRawFd,
};

use ::libc;
use anyhow::{anyhow, bail};
use to_socket_addrs::ToSocketAddrsWithDefaultPort;

use super::Parameter;

pub fn create_tcp_socket_client(parameter: &Parameter) -> anyhow::Result<TcpStream> {
    let socket_addr = parameter
        .server
        .as_str()
        .with_default_port(super::config::DEFAULT_SERVER_PORT);

    let mut socket = TcpStream::connect(socket_addr)?;
    // TODO: "make reusable" (SO_REUSEADDR)
    socket.set_nodelay(true)?;

    Ok(socket)
}

pub fn create_udp_socket_client(parameter: &Parameter, ipv6: bool) -> anyhow::Result<UdpSocket> {
    let catch_all_host = crate::common::catch_all_host(ipv6);
    let mut higher_port_attempt = 0;
    let mut socket_result = None;

    while higher_port_attempt < 256 {
        let port = parameter.client_port + higher_port_attempt;

        let mut socket = match UdpSocket::bind((catch_all_host, port)) {
            Ok(socket) => socket,
            Err(err) => match err.kind() {
                ErrorKind::AddrInUse => {
                    higher_port_attempt += 1;
                    continue;
                }
                _ => {
                    bail!("Error while trying to create UDP socket: {}", err);
                }
            },
        };

        if let Err(err) = set_udp_receive_buffer(&mut socket, parameter.udp_buffer) {
            println!("WARNING: {}", err);
        };

        println!("Receiving data over UDP at: {}", socket.local_addr()?);
        socket_result = Some(socket);
        break;
    }

    if higher_port_attempt > 0 {
        println!(
            "Warning: ports {} to {} are in use",
            parameter.client_port,
            parameter.client_port + higher_port_attempt - 1,
        );
    }

    socket_result.ok_or(anyhow!("Error in creating UDP socket"))
}

pub fn set_udp_receive_buffer(
    socket: &mut UdpSocket,
    receive_buffer_size: u32,
) -> anyhow::Result<()> {
    // TODO: cross platform
    unsafe {
        let status = libc::setsockopt(
            socket.as_raw_fd(),
            1,
            8,
            &receive_buffer_size as *const u32 as *const libc::c_void,
            ::core::mem::size_of::<u32>() as libc::c_ulong as libc::socklen_t,
        );
        if status < 0 as libc::c_int {
            bail!("Could not resize UDP receive buffer");
        }
    }

    Ok(())
}
