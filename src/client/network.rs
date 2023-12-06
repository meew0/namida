use std::{
    io::ErrorKind,
    net::{TcpStream, UdpSocket},
    os::fd::AsRawFd,
};

use ::libc;
use anyhow::bail;
use to_socket_addrs::ToSocketAddrsWithDefaultPort;

use super::get;

/// Establishes a new TCP control session based on the given parameters. The TCP session is
/// connected to the given server; we return the socket on success. Whether the socket is IPv6 or
/// IPv4 is determined by the given address.
///
/// # Errors
/// Returns an error if the socket could not be created or configured correctly.
pub fn create_tcp_socket(server: &str) -> anyhow::Result<TcpStream> {
    let socket_addr = server.with_default_port(super::config::DEFAULT_SERVER_PORT);

    let socket = TcpStream::connect(socket_addr)?;
    // TODO: "make reusable" (SO_REUSEADDR)
    socket.set_nodelay(true)?; // disable Nagle's algorithm

    Ok(socket)
}

/// Establishes a new UDP socket for data transfer, returning the created socket on success. The
/// parameter structure is used for setting the size of the UDP receive buffer. This will be an
/// IPv6 socket if `ipv6` is `true` and an IPv4 socket otherwise. The next available port starting
/// from `parameter.client_port` will be taken.
///
/// # Errors
/// Returns an error if the socket could not be created or configured correctly.
pub fn create_udp_socket(parameter: &get::Parameter, ipv6: bool) -> anyhow::Result<UdpSocket> {
    let catch_all_host = crate::common::catch_all_host(ipv6);
    let port = parameter.client_port.unwrap_or(0);

    let mut socket = match UdpSocket::bind((catch_all_host, port)) {
        Ok(socket) => socket,
        Err(err) => match err.kind() {
            ErrorKind::AddrInUse => {
                bail!("UDP port {port} is in use. Please specify a different port, or make use of auto-discovery by not specifying a port at all.");
            }
            _ => {
                bail!(
                    "Miscellaneous error while trying to create UDP socket: {}",
                    err
                );
            }
        },
    };

    // set the receive buffer size
    if let Err(err) = set_udp_receive_buffer(&mut socket, parameter.udp_buffer) {
        println!("WARNING: {err}");
    };

    println!("Receiving data over UDP at: {}", socket.local_addr()?);

    Ok(socket)
}

/// Sets the transmit buffer of the given UDP socket. Currently only works on Linux.
#[allow(clippy::needless_pass_by_ref_mut)] // since we are theoretically mutating the socket
fn set_udp_receive_buffer(socket: &mut UdpSocket, receive_buffer_size: u32) -> anyhow::Result<()> {
    // TODO: cross platform
    unsafe {
        #[allow(clippy::cast_possible_truncation)]
        let status = libc::setsockopt(
            socket.as_raw_fd(),
            1,
            8,
            std::ptr::addr_of!(receive_buffer_size).cast(),
            ::core::mem::size_of::<u32>() as libc::c_ulong as libc::socklen_t,
        );
        if status < 0 as libc::c_int {
            bail!("Could not resize UDP receive buffer");
        }
    }

    Ok(())
}
