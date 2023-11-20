use crate::extc;
use ::libc;

use super::{ttp_parameter_t, ttp_session_t};

#[no_mangle]
pub unsafe extern "C" fn create_tcp_socket_client(
    mut session: *mut ttp_session_t,
    mut server_name: *const libc::c_char,
    mut server_port: u16,
) -> libc::c_int {
    let mut hints: extc::addrinfo = extc::addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut extc::sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut extc::addrinfo,
    };
    let mut info: *mut extc::addrinfo = 0 as *mut extc::addrinfo;
    let mut info_save: *mut extc::addrinfo = 0 as *mut extc::addrinfo;
    let mut buffer: [libc::c_char; 10] = [0; 10];
    let mut socket_fd: libc::c_int = 0;
    let mut yes: libc::c_int = 1 as libc::c_int;
    let mut status: libc::c_int = 0;
    extc::memset(
        &mut hints as *mut extc::addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<extc::addrinfo>() as libc::c_ulong,
    );
    hints.ai_family = if (*(*session).parameter).ipv6_yn as libc::c_int != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    hints.ai_socktype = extc::SOCK_STREAM as libc::c_int;
    extc::sprintf(
        buffer.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        server_port as libc::c_int,
    );
    status = extc::getaddrinfo(server_name, buffer.as_mut_ptr(), &mut hints, &mut info);
    if status != 0 {
        return crate::common::error::error_handler(
            b"network.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            b"Error in getting address information for server\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    info_save = info;
    loop {
        socket_fd = extc::socket((*info).ai_family, (*info).ai_socktype, (*info).ai_protocol);
        if socket_fd < 0 as libc::c_int {
            crate::common::error::error_handler(
                b"network.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                b"Could not create socket\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            status = extc::setsockopt(
                socket_fd,
                1 as libc::c_int,
                2 as libc::c_int,
                &mut yes as *mut libc::c_int as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as extc::socklen_t,
            );
            if status < 0 as libc::c_int {
                crate::common::error::error_handler(
                    b"network.c\0" as *const u8 as *const libc::c_char,
                    122 as libc::c_int,
                    b"Could not make socket reusable\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                extc::close(socket_fd);
            } else {
                status = extc::setsockopt(
                    socket_fd,
                    extc::IPPROTO_TCP as libc::c_int,
                    1 as libc::c_int,
                    &mut yes as *mut libc::c_int as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as extc::socklen_t,
                );
                if status < 0 as libc::c_int {
                    crate::common::error::error_handler(
                        b"network.c\0" as *const u8 as *const libc::c_char,
                        130 as libc::c_int,
                        b"Could not disable Nagle's algorithm\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                    );
                    extc::close(socket_fd);
                } else {
                    status = extc::connect(
                        socket_fd,
                        extc::__CONST_SOCKADDR_ARG {
                            __sockaddr__: (*info).ai_addr,
                        },
                        (*info).ai_addrlen,
                    );
                    if status == 0 as libc::c_int {
                        (*session).server_address =
                            extc::malloc((*info).ai_addrlen as libc::c_ulong)
                                as *mut extc::sockaddr;
                        (*session).server_address_length = (*info).ai_addrlen;
                        if ((*session).server_address).is_null() {
                            crate::common::error::error_handler(
                                b"network.c\0" as *const u8 as *const libc::c_char,
                                143 as libc::c_int,
                                b"Could not allocate space for server address\0" as *const u8
                                    as *const libc::c_char,
                                1 as libc::c_int,
                            );
                        }
                        extc::memcpy(
                            (*session).server_address as *mut libc::c_void,
                            (*info).ai_addr as *const libc::c_void,
                            (*info).ai_addrlen as libc::c_ulong,
                        );
                        break;
                    }
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
        return crate::common::error::error_handler(
            b"network.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            b"Error in connecting to Tsunami server\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return socket_fd;
}
#[no_mangle]
pub unsafe extern "C" fn create_udp_socket_client(
    mut parameter: *mut ttp_parameter_t,
) -> libc::c_int {
    let mut hints: extc::addrinfo = extc::addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut extc::sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut extc::addrinfo,
    };
    let mut info: *mut extc::addrinfo = 0 as *mut extc::addrinfo;
    let mut info_save: *mut extc::addrinfo = 0 as *mut extc::addrinfo;
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
    hints.ai_family = if (*parameter).ipv6_yn as libc::c_int != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    hints.ai_socktype = extc::SOCK_DGRAM as libc::c_int;
    loop {
        extc::sprintf(
            buffer.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*parameter).client_port as libc::c_int + higher_port_attempt,
        );
        status = extc::getaddrinfo(
            0 as *const libc::c_char,
            buffer.as_mut_ptr(),
            &mut hints,
            &mut info,
        );
        if status != 0 {
            return crate::common::error::error_handler(
                b"network.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int,
                b"Error in getting address information\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        info_save = info;
        loop {
            socket_fd = extc::socket((*info).ai_family, (*info).ai_socktype, (*info).ai_protocol);
            if !(socket_fd < 0 as libc::c_int) {
                status = extc::setsockopt(
                    socket_fd,
                    1 as libc::c_int,
                    8 as libc::c_int,
                    &mut (*parameter).udp_buffer as *mut u32 as *const libc::c_void,
                    ::core::mem::size_of::<u32>() as libc::c_ulong as extc::socklen_t,
                );
                if status < 0 as libc::c_int {
                    crate::common::error::error_handler(
                        b"network.c\0" as *const u8 as *const libc::c_char,
                        211 as libc::c_int,
                        b"Error in resizing UDP receive buffer\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                    );
                }
                status = extc::bind(
                    socket_fd,
                    extc::__CONST_SOCKADDR_ARG {
                        __sockaddr__: (*info).ai_addr,
                    },
                    (*info).ai_addrlen,
                );
                if status == 0 as libc::c_int {
                    (*parameter).client_port =
                        extc::__bswap_16((*((*info).ai_addr as *mut extc::sockaddr_in)).sin_port);
                    extc::fprintf(
                        extc::stderr,
                        b"Receiving data on UDP port %d\n\0" as *const u8 as *const libc::c_char,
                        (*parameter).client_port as libc::c_int,
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
        return crate::common::error::error_handler(
            b"network.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int,
            b"Error in creating UDP socket\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return socket_fd;
}
