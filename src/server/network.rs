use std::{
    net::{TcpListener, ToSocketAddrs, UdpSocket},
    os::fd::AsRawFd,
};

use ::libc;
use anyhow::bail;

use super::Parameter;

/// Establishes a new TCP listener by binding to the `bind` address specified in the
/// `parameter`s.
///
/// # Errors
/// Returns an error if binding was unsuccessful.
pub fn create_tcp_socket(parameter: &Parameter) -> anyhow::Result<TcpListener> {
    let listener = TcpListener::bind(&parameter.bind)?;
    Ok(listener)
}

/// Establishes a new UDP server socket. It will be an IPv6 socket if `parameter.bind` specifies an
/// IPv6 address, and an IPv4 address otherwise.
///
/// # Errors
/// Returns an error if binding was unsuccessful.
///
/// # Panics
/// Panics if we were unable to get IPv6 or IPv4 information from `parameter.bind`.
pub fn create_udp_socket(parameter: &Parameter) -> anyhow::Result<UdpSocket> {
    let ipv6 = parameter
        .bind
        .to_socket_addrs()?
        .next()
        .expect("parameter.bind should resolve to something valid")
        .is_ipv6();
    let catch_all_host = crate::common::catch_all_host(ipv6);

    let mut socket = UdpSocket::bind((catch_all_host, 0))?;

    // TODO: “make reusable”

    set_udp_transmit_buffer(&mut socket, parameter.udp_buffer)?;

    Ok(socket)
}

/// Sets the transmit buffer of the given UDP socket. Currently only works on Linux.
fn set_udp_transmit_buffer(
    socket: &mut UdpSocket,
    transmit_buffer_size: u32,
) -> anyhow::Result<()> {
    // TODO: cross platform
    unsafe {
        #[allow(clippy::cast_possible_truncation)]
        let status = libc::setsockopt(
            socket.as_raw_fd(),
            1,
            7,
            std::ptr::addr_of!(transmit_buffer_size).cast(),
            ::core::mem::size_of::<u32>() as libc::c_ulong as libc::socklen_t,
        );
        if status < 0 as libc::c_int {
            bail!("Could not resize UDP transmit buffer");
        }
    }

    Ok(())
}
