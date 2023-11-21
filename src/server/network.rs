use crate::extc;
use ::libc;
use anyhow::bail;

pub unsafe fn create_tcp_socket_server(
    mut parameter: *mut super::ttp_parameter_t,
) -> anyhow::Result<i32> {
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
    let mut yes: libc::c_int = 1 as libc::c_int;
    let mut status: libc::c_int = 0;
    extc::memset(
        &mut hints as *mut extc::addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<extc::addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x1 as libc::c_int;
    hints.ai_family = if (*parameter).ipv6_yn as libc::c_int != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    hints.ai_socktype = extc::SOCK_STREAM as libc::c_int;
    extc::sprintf(
        buffer.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        (*parameter).tcp_port as libc::c_int,
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
                2 as libc::c_int,
                &mut yes as *mut libc::c_int as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as extc::socklen_t,
            );
            if status >= 0 as libc::c_int {
                status = extc::bind(
                    socket_fd,
                    extc::__CONST_SOCKADDR_ARG {
                        __sockaddr__: (*info).ai_addr,
                    },
                    (*info).ai_addrlen,
                );
                if status == 0 as libc::c_int {
                    break;
                }
            }
        }
        info = (*info).ai_next;
        if info.is_null() {
            break;
        }
    }
    extc::freeaddrinfo(info_save);
    if info.is_null() {
        bail!("Error in creating TCP server socket");
    }
    status = extc::listen(socket_fd, 4096 as libc::c_int);
    if status < 0 as libc::c_int {
        bail!("Error in listening on TCP server socket");
    }
    Ok(socket_fd)
}
pub unsafe fn create_udp_socket_server(
    mut parameter: *mut super::ttp_parameter_t,
) -> anyhow::Result<i32> {
    let mut socket_fd: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut yes: libc::c_int = 1 as libc::c_int;
    socket_fd = extc::socket(
        if (*parameter).ipv6_yn as libc::c_int != 0 {
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
        &mut (*parameter).udp_buffer as *mut u32 as *const libc::c_void,
        ::core::mem::size_of::<u32>() as libc::c_ulong as extc::socklen_t,
    );
    if status < 0 as libc::c_int {
        println!("WARNING: Error in resizing UDP transmit buffer");
    }
    Ok(socket_fd)
}
