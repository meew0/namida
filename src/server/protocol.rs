use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    pub type sockaddr_x25;
    pub type sockaddr_un;
    pub type sockaddr_ns;
    pub type sockaddr_iso;
    pub type sockaddr_ipx;
    pub type sockaddr_inarp;
    pub type sockaddr_eon;
    pub type sockaddr_dl;
    pub type sockaddr_ax25;
    pub type sockaddr_at;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn fgets(
        __s: *mut libc::c_char,
        __n: libc::c_int,
        __stream: *mut FILE,
    ) -> *mut libc::c_char;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    fn ftello(__stream: *mut FILE) -> __off64_t;
    fn pclose(__stream: *mut FILE) -> libc::c_int;
    fn popen(__command: *const libc::c_char, __modes: *const libc::c_char) -> *mut FILE;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn getpeername(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn gai_strerror(__ecode: libc::c_int) -> *const libc::c_char;
    fn time(__timer: *mut time_t) -> time_t;
    static PROTOCOL_REVISION: u_int32_t;
    static REQUEST_RETRANSMIT: u_int16_t;
    static REQUEST_RESTART: u_int16_t;
    static REQUEST_ERROR_RATE: u_int16_t;
    static mut g_error: [libc::c_char; 0];
    fn get_random_data(buffer: *mut u_char, bytes: size_t) -> libc::c_int;
    fn htonll(value: u_int64_t) -> u_int64_t;
    fn prepare_proof(
        buffer: *mut u_char,
        bytes: size_t,
        secret: *const u_char,
        digest: *mut u_char,
    ) -> *mut u_char;
    fn read_line(
        fd: libc::c_int,
        buffer: *mut libc::c_char,
        buffer_length: size_t,
    ) -> libc::c_int;
    fn full_write(_: libc::c_int, _: *const libc::c_void, _: size_t) -> ssize_t;
    fn full_read(_: libc::c_int, _: *mut libc::c_void, _: size_t) -> ssize_t;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    fn xscript_data_log(session: *mut ttp_session_t, logline: *const libc::c_char);
    fn create_udp_socket(parameter: *mut ttp_parameter_t) -> libc::c_int;
    fn build_datagram(
        session: *mut ttp_session_t,
        block_index: u_int32_t,
        block_type: u_int16_t,
        datagram: *mut u_char,
    ) -> libc::c_int;
    fn xscript_open(session: *mut ttp_session_t);
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type u_char = __u_char;
pub type ssize_t = __ssize_t;
pub type time_t = __time_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct _IO_FILE {
    pub _flags: libc::c_int,
    pub _IO_read_ptr: *mut libc::c_char,
    pub _IO_read_end: *mut libc::c_char,
    pub _IO_read_base: *mut libc::c_char,
    pub _IO_write_base: *mut libc::c_char,
    pub _IO_write_ptr: *mut libc::c_char,
    pub _IO_write_end: *mut libc::c_char,
    pub _IO_buf_base: *mut libc::c_char,
    pub _IO_buf_end: *mut libc::c_char,
    pub _IO_save_base: *mut libc::c_char,
    pub _IO_backup_base: *mut libc::c_char,
    pub _IO_save_end: *mut libc::c_char,
    pub _markers: *mut _IO_marker,
    pub _chain: *mut _IO_FILE,
    pub _fileno: libc::c_int,
    pub _flags2: libc::c_int,
    pub _old_offset: __off_t,
    pub _cur_column: libc::c_ushort,
    pub _vtable_offset: libc::c_schar,
    pub _shortbuf: [libc::c_char; 1],
    pub _lock: *mut libc::c_void,
    pub _offset: __off64_t,
    pub _codecvt: *mut _IO_codecvt,
    pub _wide_data: *mut _IO_wide_data,
    pub _freeres_list: *mut _IO_FILE,
    pub _freeres_buf: *mut libc::c_void,
    pub __pad5: size_t,
    pub _mode: libc::c_int,
    pub _unused2: [libc::c_char; 20],
}
pub type _IO_lock_t = ();
pub type FILE = _IO_FILE;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type socklen_t = __socklen_t;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union __SOCKADDR_ARG {
    pub __sockaddr__: *mut sockaddr,
    pub __sockaddr_at__: *mut sockaddr_at,
    pub __sockaddr_ax25__: *mut sockaddr_ax25,
    pub __sockaddr_dl__: *mut sockaddr_dl,
    pub __sockaddr_eon__: *mut sockaddr_eon,
    pub __sockaddr_in__: *mut sockaddr_in,
    pub __sockaddr_in6__: *mut sockaddr_in6,
    pub __sockaddr_inarp__: *mut sockaddr_inarp,
    pub __sockaddr_ipx__: *mut sockaddr_ipx,
    pub __sockaddr_iso__: *mut sockaddr_iso,
    pub __sockaddr_ns__: *mut sockaddr_ns,
    pub __sockaddr_un__: *mut sockaddr_un,
    pub __sockaddr_x25__: *mut sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in6 {
    pub sin6_family: sa_family_t,
    pub sin6_port: in_port_t,
    pub sin6_flowinfo: uint32_t,
    pub sin6_addr: in6_addr,
    pub sin6_scope_id: uint32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in6_addr {
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
    pub __u6_addr8: [uint8_t; 16],
    pub __u6_addr16: [uint16_t; 8],
    pub __u6_addr32: [uint32_t; 4],
}
pub type in_port_t = uint16_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr_in {
    pub sin_family: sa_family_t,
    pub sin_port: in_port_t,
    pub sin_addr: in_addr,
    pub sin_zero: [libc::c_uchar; 8],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct in_addr {
    pub s_addr: in_addr_t,
}
pub type in_addr_t = uint32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub union __CONST_SOCKADDR_ARG {
    pub __sockaddr__: *const sockaddr,
    pub __sockaddr_at__: *const sockaddr_at,
    pub __sockaddr_ax25__: *const sockaddr_ax25,
    pub __sockaddr_dl__: *const sockaddr_dl,
    pub __sockaddr_eon__: *const sockaddr_eon,
    pub __sockaddr_in__: *const sockaddr_in,
    pub __sockaddr_in6__: *const sockaddr_in6,
    pub __sockaddr_inarp__: *const sockaddr_inarp,
    pub __sockaddr_ipx__: *const sockaddr_ipx,
    pub __sockaddr_iso__: *const sockaddr_iso,
    pub __sockaddr_ns__: *const sockaddr_ns,
    pub __sockaddr_un__: *const sockaddr_un,
    pub __sockaddr_x25__: *const sockaddr_x25,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct addrinfo {
    pub ai_flags: libc::c_int,
    pub ai_family: libc::c_int,
    pub ai_socktype: libc::c_int,
    pub ai_protocol: libc::c_int,
    pub ai_addrlen: socklen_t,
    pub ai_addr: *mut sockaddr,
    pub ai_canonname: *mut libc::c_char,
    pub ai_next: *mut addrinfo,
}
pub type ull_t = libc::c_ulonglong;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct retransmission_t {
    pub request_type: u_int16_t,
    pub block: u_int32_t,
    pub error_rate: u_int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_parameter_t {
    pub epoch: time_t,
    pub verbose_yn: u_char,
    pub transcript_yn: u_char,
    pub ipv6_yn: u_char,
    pub tcp_port: u_int16_t,
    pub udp_buffer: u_int32_t,
    pub hb_timeout: u_int16_t,
    pub secret: *const u_char,
    pub client: *const libc::c_char,
    pub finishhook: *const u_char,
    pub allhook: *const u_char,
    pub block_size: u_int32_t,
    pub file_size: u_int64_t,
    pub block_count: u_int32_t,
    pub target_rate: u_int32_t,
    pub error_rate: u_int32_t,
    pub ipd_time: u_int32_t,
    pub slower_num: u_int16_t,
    pub slower_den: u_int16_t,
    pub faster_num: u_int16_t,
    pub faster_den: u_int16_t,
    pub ringbuf: *mut libc::c_char,
    pub fileout: u_int16_t,
    pub slotnumber: libc::c_int,
    pub totalslots: libc::c_int,
    pub samplerate: libc::c_int,
    pub file_names: *mut *mut libc::c_char,
    pub file_sizes: *mut size_t,
    pub file_name_size: u_int16_t,
    pub total_files: u_int16_t,
    pub wait_u_sec: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_transfer_t {
    pub parameter: *mut ttp_parameter_t,
    pub filename: *mut libc::c_char,
    pub file: *mut FILE,
    pub vsib: *mut FILE,
    pub transcript: *mut FILE,
    pub udp_fd: libc::c_int,
    pub udp_address: *mut sockaddr,
    pub udp_length: socklen_t,
    pub ipd_current: libc::c_double,
    pub block: u_int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_session_t {
    pub parameter: *mut ttp_parameter_t,
    pub transfer: ttp_transfer_t,
    pub client_fd: libc::c_int,
    pub session_id: libc::c_int,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_accept_retransmit(
    mut session: *mut ttp_session_t,
    mut retransmission: *mut retransmission_t,
    mut datagram: *mut u_char,
) -> libc::c_int {
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    static mut iteration: libc::c_int = 0 as libc::c_int;
    static mut stats_line: [libc::c_char; 80] = [0; 80];
    let mut status: libc::c_int = 0;
    let mut type_0: u_int16_t = 0;
    (*retransmission).block = __bswap_32((*retransmission).block);
    (*retransmission).error_rate = __bswap_32((*retransmission).error_rate);
    type_0 = __bswap_16((*retransmission).request_type);
    if type_0 as libc::c_int == REQUEST_ERROR_RATE as libc::c_int {
        if (*retransmission).error_rate > (*param).error_rate {
            let mut factor1: libc::c_double = 1.0f64
                * (*param).slower_num as libc::c_int as libc::c_double
                / (*param).slower_den as libc::c_int as libc::c_double - 1.0f64;
            let mut factor2: libc::c_double = (1.0f64
                + (*retransmission).error_rate as libc::c_double
                - (*param).error_rate as libc::c_double)
                / (100000.0f64 - (*param).error_rate as libc::c_double);
            (*xfer).ipd_current *= 1.0f64 + factor1 * factor2;
        } else {
            (*xfer).ipd_current
                *= (*param).faster_num as libc::c_double
                    / (*param).faster_den as libc::c_int as libc::c_double;
        }
        (*xfer)
            .ipd_current = if (if (*xfer).ipd_current < 10000.0f64 {
            (*xfer).ipd_current
        } else {
            10000.0f64
        }) > (*param).ipd_time as libc::c_double
        {
            if (*xfer).ipd_current < 10000.0f64 {
                (*xfer).ipd_current
            } else {
                10000.0f64
            }
        } else {
            (*param).ipd_time as libc::c_double
        };
        sprintf(
            stats_line.as_mut_ptr(),
            b"%6u %3.2fus %5uus %7u %6.2f %3u\n\0" as *const u8 as *const libc::c_char,
            (*retransmission).error_rate,
            (*xfer).ipd_current as libc::c_float as libc::c_double,
            (*param).ipd_time,
            (*xfer).block,
            100.0f64 * (*xfer).block as libc::c_double
                / (*param).block_count as libc::c_double,
            (*session).session_id,
        );
        let fresh0 = iteration;
        iteration = iteration + 1;
        if fresh0 % 23 as libc::c_int == 0 {
            printf(
                b" erate     ipd  target   block   %%done srvNr\n\0" as *const u8
                    as *const libc::c_char,
            );
        }
        printf(b"%s\0" as *const u8 as *const libc::c_char, stats_line.as_mut_ptr());
        if (*param).transcript_yn != 0 {
            xscript_data_log(session, stats_line.as_mut_ptr());
        }
    } else if type_0 as libc::c_int == REQUEST_RESTART as libc::c_int {
        if (*retransmission).block == 0 as libc::c_int as u_int32_t
            || (*retransmission).block > (*param).block_count
        {
            sprintf(
                g_error.as_mut_ptr(),
                b"Attempt to restart at illegal block %u\0" as *const u8
                    as *const libc::c_char,
                (*retransmission).block,
            );
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                151 as libc::c_int,
                g_error.as_mut_ptr(),
                0 as libc::c_int,
            );
        } else {
            (*xfer).block = (*retransmission).block;
        }
    } else if type_0 as libc::c_int == REQUEST_RETRANSMIT as libc::c_int {
        status = build_datagram(
            session,
            (*retransmission).block,
            'R' as i32 as u_int16_t,
            datagram,
        );
        if status < 0 as libc::c_int {
            sprintf(
                g_error.as_mut_ptr(),
                b"Could not build retransmission for block %u\0" as *const u8
                    as *const libc::c_char,
                (*retransmission).block,
            );
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                162 as libc::c_int,
                g_error.as_mut_ptr(),
                0 as libc::c_int,
            );
        }
        status = sendto(
            (*xfer).udp_fd,
            datagram as *const libc::c_void,
            (6 as libc::c_int as u_int32_t).wrapping_add((*param).block_size) as size_t,
            0 as libc::c_int,
            __CONST_SOCKADDR_ARG {
                __sockaddr__: (*xfer).udp_address,
            },
            (*xfer).udp_length,
        ) as libc::c_int;
        if status < 0 as libc::c_int {
            sprintf(
                g_error.as_mut_ptr(),
                b"Could not retransmit block %u\0" as *const u8 as *const libc::c_char,
                (*retransmission).block,
            );
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                169 as libc::c_int,
                g_error.as_mut_ptr(),
                0 as libc::c_int,
            );
        }
    } else {
        sprintf(
            g_error.as_mut_ptr(),
            b"Received unknown retransmission request of type %u\0" as *const u8
                as *const libc::c_char,
            __bswap_16((*retransmission).request_type) as libc::c_int,
        );
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            175 as libc::c_int,
            g_error.as_mut_ptr(),
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_authenticate(
    mut session: *mut ttp_session_t,
    mut secret: *const u_char,
) -> libc::c_int {
    let mut random: [u_char; 64] = [0; 64];
    let mut server_digest: [u_char; 16] = [0; 16];
    let mut client_digest: [u_char; 16] = [0; 16];
    let mut i: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    status = get_random_data(random.as_mut_ptr(), 64 as libc::c_int as size_t);
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            b"Access to random data is broken\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = full_write(
        (*session).client_fd,
        random.as_mut_ptr() as *const libc::c_void,
        64 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            219 as libc::c_int,
            b"Could not send authentication challenge to client\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = full_read(
        (*session).client_fd,
        client_digest.as_mut_ptr() as *mut libc::c_void,
        16 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            b"Could not read authentication response from client\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    prepare_proof(
        random.as_mut_ptr(),
        64 as libc::c_int as size_t,
        secret,
        server_digest.as_mut_ptr(),
    );
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        if client_digest[i as usize] as libc::c_int
            != server_digest[i as usize] as libc::c_int
        {
            full_write(
                (*session).client_fd,
                b"\x01\0" as *const u8 as *const libc::c_char as *const libc::c_void,
                1 as libc::c_int as size_t,
            );
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                231 as libc::c_int,
                b"Authentication failed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        i += 1;
        i;
    }
    status = full_write(
        (*session).client_fd,
        b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
            b"Could not send authentication confirmation to client\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_negotiate(mut session: *mut ttp_session_t) -> libc::c_int {
    let mut server_revision: u_int32_t = __bswap_32(PROTOCOL_REVISION);
    let mut client_revision: u_int32_t = 0;
    let mut status: libc::c_int = 0;
    status = full_write(
        (*session).client_fd,
        &mut server_revision as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            263 as libc::c_int,
            b"Could not send protocol revision number\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = full_read(
        (*session).client_fd,
        &mut client_revision as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            268 as libc::c_int,
            b"Could not read protocol revision number\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return if client_revision == server_revision {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ttp_open_port(mut session: *mut ttp_session_t) -> libc::c_int {
    let mut address: *mut sockaddr = 0 as *mut sockaddr;
    let mut status: libc::c_int = 0;
    let mut port: u_int16_t = 0;
    let mut ipv6_yn: u_char = (*(*session).parameter).ipv6_yn;
    if ((*(*session).parameter).client).is_null() {
        (*session)
            .transfer
            .udp_length = (if ipv6_yn as libc::c_int != 0 {
            ::core::mem::size_of::<sockaddr_in6>() as libc::c_ulong
        } else {
            ::core::mem::size_of::<sockaddr_in>() as libc::c_ulong
        }) as socklen_t;
        address = malloc((*session).transfer.udp_length as libc::c_ulong)
            as *mut sockaddr;
        if address.is_null() {
            error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                296 as libc::c_int,
                b"Could not allocate space for UDP socket address\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        getpeername(
            (*session).client_fd,
            __SOCKADDR_ARG {
                __sockaddr__: address,
            },
            &mut (*session).transfer.udp_length,
        );
    } else {
        let mut result: *mut addrinfo = 0 as *mut addrinfo;
        let mut errmsg: [libc::c_char; 256] = [0; 256];
        let mut status_0: libc::c_int = getaddrinfo(
            (*(*session).parameter).client,
            0 as *const libc::c_char,
            0 as *const addrinfo,
            &mut result,
        );
        if status_0 != 0 {
            sprintf(
                errmsg.as_mut_ptr(),
                b"error in getaddrinfo: %s\n\0" as *const u8 as *const libc::c_char,
                gai_strerror(status_0),
            );
            error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                309 as libc::c_int,
                errmsg.as_mut_ptr(),
                1 as libc::c_int,
            );
            return 1 as libc::c_int;
        }
        if (*result).ai_family == 10 as libc::c_int {
            ipv6_yn = 1 as libc::c_int as u_char;
        } else {
            ipv6_yn = 0 as libc::c_int as u_char;
        }
        (*(*session).parameter).ipv6_yn = ipv6_yn;
        (*session).transfer.udp_length = (*result).ai_addrlen;
        address = malloc((*result).ai_addrlen as libc::c_ulong) as *mut sockaddr;
        if address.is_null() {
            error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                323 as libc::c_int,
                b"Could not allocate space for UDP socket address\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        memcpy(
            address as *mut libc::c_void,
            (*result).ai_addr as *const libc::c_void,
            (*result).ai_addrlen as libc::c_ulong,
        );
        if !((*result).ai_canonname).is_null() {
            printf(
                b"Sending data to: %s\n\0" as *const u8 as *const libc::c_char,
                (*result).ai_canonname,
            );
        }
        freeaddrinfo(result);
    }
    status = full_read(
        (*session).client_fd,
        &mut port as *mut u_int16_t as *mut libc::c_void,
        2 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            335 as libc::c_int,
            b"Could not read UDP port number\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if ipv6_yn != 0 {
        (*(address as *mut sockaddr_in6)).sin6_port = port;
    } else {
        (*(address as *mut sockaddr_in)).sin_port = port;
    }
    if (*(*session).parameter).verbose_yn != 0 {
        printf(
            b"Sending to client port %d\n\0" as *const u8 as *const libc::c_char,
            __bswap_16(port) as libc::c_int,
        );
    }
    (*session).transfer.udp_fd = create_udp_socket((*session).parameter);
    if (*session).transfer.udp_fd < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            348 as libc::c_int,
            b"Could not create UDP socket\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*session).transfer.udp_address = address;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_open_transfer(
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    let mut filename: [libc::c_char; 1024] = [0; 1024];
    let mut file_size: u_int64_t = 0;
    let mut block_size: u_int32_t = 0;
    let mut block_count: u_int32_t = 0;
    let mut epoch: time_t = 0;
    let mut status: libc::c_int = 0;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    let mut size: [libc::c_char; 10] = [0; 10];
    let mut file_no: [libc::c_char; 10] = [0; 10];
    let mut message: [libc::c_char; 20] = [0; 20];
    let mut i: u_int16_t = 0;
    let mut ping_s: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut ping_e: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    memset(
        xfer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_transfer_t>() as libc::c_ulong,
    );
    status = read_line(
        (*session).client_fd,
        filename.as_mut_ptr(),
        1024 as libc::c_int as size_t,
    );
    if status < 0 as libc::c_int {
        error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            398 as libc::c_int,
            b"Could not read filename from client\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    filename[(1024 as libc::c_int - 1 as libc::c_int)
        as usize] = '\0' as i32 as libc::c_char;
    if strcmp(filename.as_mut_ptr(), b"!#DIR??\0" as *const u8 as *const libc::c_char)
        == 0
    {
        snprintf(
            file_no.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            b"%u\0" as *const u8 as *const libc::c_char,
            (*param).total_files as libc::c_int,
        );
        full_write(
            (*session).client_fd,
            file_no.as_mut_ptr() as *const libc::c_void,
            (strlen(file_no.as_mut_ptr()))
                .wrapping_add(1 as libc::c_int as libc::c_ulong),
        );
        i = 0 as libc::c_int as u_int16_t;
        while (i as libc::c_int) < (*param).total_files as libc::c_int {
            full_write(
                (*session).client_fd,
                *((*param).file_names).offset(i as isize) as *const libc::c_void,
                (strlen(*((*param).file_names).offset(i as isize)))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            snprintf(
                message.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                b"%Lu\0" as *const u8 as *const libc::c_char,
                *((*param).file_sizes).offset(i as isize) as ull_t,
            );
            full_write(
                (*session).client_fd,
                message.as_mut_ptr() as *const libc::c_void,
                (strlen(message.as_mut_ptr()))
                    .wrapping_add(1 as libc::c_int as libc::c_ulong),
            );
            i = i.wrapping_add(1);
            i;
        }
        full_read(
            (*session).client_fd,
            message.as_mut_ptr() as *mut libc::c_void,
            1 as libc::c_int as size_t,
        );
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            414 as libc::c_int,
            b"File list sent!\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    } else if strcmp(filename.as_mut_ptr(), b"*\0" as *const u8 as *const libc::c_char)
        == 0
    {
        if !((*param).allhook).is_null() {
            let MaxFileListLength: libc::c_int = 32768 as libc::c_int;
            let vla = MaxFileListLength as usize;
            let mut fileList: Vec::<libc::c_char> = ::std::vec::from_elem(0, vla);
            let mut fl: *const libc::c_char = 0 as *const libc::c_char;
            let mut nFile: libc::c_int = 0 as libc::c_int;
            let mut length: libc::c_int = 0 as libc::c_int;
            let mut l: libc::c_int = 0;
            let mut p: *mut FILE = 0 as *mut FILE;
            fprintf(
                stderr,
                b"Using allhook program: %s\n\0" as *const u8 as *const libc::c_char,
                (*param).allhook,
            );
            p = popen(
                (*param).allhook as *mut libc::c_char,
                b"r\0" as *const u8 as *const libc::c_char,
            );
            if !p.is_null() {
                memset(
                    fileList.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    MaxFileListLength as libc::c_ulong,
                );
                while !(fgets(
                    message.as_mut_ptr(),
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong
                        as libc::c_int,
                    p,
                ))
                    .is_null()
                {
                    l = 0 as libc::c_int;
                    while message[l as usize] as libc::c_int >= ' ' as i32 {
                        l += 1;
                        l;
                    }
                    message[l as usize] = 0 as libc::c_int as libc::c_char;
                    fprintf(
                        stdout,
                        b"  '%s'\n\0" as *const u8 as *const libc::c_char,
                        message.as_mut_ptr(),
                    );
                    if l + length >= MaxFileListLength {
                        break;
                    }
                    strncpy(
                        fileList.as_mut_ptr().offset(length as isize),
                        message.as_mut_ptr(),
                        l as libc::c_ulong,
                    );
                    length += l + 1 as libc::c_int;
                    nFile += 1;
                    nFile;
                }
            }
            pclose(p);
            memset(
                size.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            snprintf(
                size.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                length,
            );
            full_write(
                (*session).client_fd,
                size.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as size_t,
            );
            memset(
                file_no.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            snprintf(
                file_no.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                nFile,
            );
            full_write(
                (*session).client_fd,
                file_no.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as size_t,
            );
            printf(
                b"\nSent multi-GET filename count and array size to client\n\0"
                    as *const u8 as *const libc::c_char,
            );
            memset(
                message.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
            full_read(
                (*session).client_fd,
                message.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as size_t,
            );
            printf(
                b"Client response: %s\n\0" as *const u8 as *const libc::c_char,
                message.as_mut_ptr(),
            );
            fl = fileList.as_mut_ptr();
            if nFile > 0 as libc::c_int {
                i = 0 as libc::c_int as u_int16_t;
                while (i as libc::c_int) < nFile {
                    l = strlen(fl) as libc::c_int;
                    full_write(
                        (*session).client_fd,
                        fl as *const libc::c_void,
                        (l + 1 as libc::c_int) as size_t,
                    );
                    fl = fl.offset((l + 1 as libc::c_int) as isize);
                    i = i.wrapping_add(1);
                    i;
                }
                memset(
                    message.as_mut_ptr() as *mut libc::c_void,
                    0 as libc::c_int,
                    ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
                );
                full_read(
                    (*session).client_fd,
                    message.as_mut_ptr() as *mut libc::c_void,
                    8 as libc::c_int as size_t,
                );
                printf(
                    b"Sent file list, client response: %s\n\0" as *const u8
                        as *const libc::c_char,
                    message.as_mut_ptr(),
                );
                status = read_line(
                    (*session).client_fd,
                    filename.as_mut_ptr(),
                    1024 as libc::c_int as size_t,
                );
                if status < 0 as libc::c_int {
                    error_handler(
                        b"protocol.c\0" as *const u8 as *const libc::c_char,
                        489 as libc::c_int,
                        b"Could not read filename from client\0" as *const u8
                            as *const libc::c_char,
                        1 as libc::c_int,
                    );
                }
            }
        } else {
            memset(
                size.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            snprintf(
                size.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                (*param).file_name_size as libc::c_int,
            );
            full_write(
                (*session).client_fd,
                size.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as size_t,
            );
            memset(
                file_no.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
            );
            snprintf(
                file_no.as_mut_ptr(),
                ::core::mem::size_of::<[libc::c_char; 10]>() as libc::c_ulong,
                b"%u\0" as *const u8 as *const libc::c_char,
                (*param).total_files as libc::c_int,
            );
            full_write(
                (*session).client_fd,
                file_no.as_mut_ptr() as *const libc::c_void,
                10 as libc::c_int as size_t,
            );
            printf(
                b"\nSent multi-GET filename count and array size to client\n\0"
                    as *const u8 as *const libc::c_char,
            );
            memset(
                message.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
            full_read(
                (*session).client_fd,
                message.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as size_t,
            );
            printf(
                b"Client response: %s\n\0" as *const u8 as *const libc::c_char,
                message.as_mut_ptr(),
            );
            i = 0 as libc::c_int as u_int16_t;
            while (i as libc::c_int) < (*param).total_files as libc::c_int {
                full_write(
                    (*session).client_fd,
                    *((*param).file_names).offset(i as isize) as *const libc::c_void,
                    (strlen(*((*param).file_names).offset(i as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                );
                i = i.wrapping_add(1);
                i;
            }
            memset(
                message.as_mut_ptr() as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<[libc::c_char; 20]>() as libc::c_ulong,
            );
            full_read(
                (*session).client_fd,
                message.as_mut_ptr() as *mut libc::c_void,
                8 as libc::c_int as size_t,
            );
            printf(
                b"Sent file list, client response: %s\n\0" as *const u8
                    as *const libc::c_char,
                message.as_mut_ptr(),
            );
            status = read_line(
                (*session).client_fd,
                filename.as_mut_ptr(),
                1024 as libc::c_int as size_t,
            );
            if status < 0 as libc::c_int {
                error_handler(
                    b"protocol.c\0" as *const u8 as *const libc::c_char,
                    520 as libc::c_int,
                    b"Could not read filename from client\0" as *const u8
                        as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
        }
    }
    (*xfer).filename = strdup(filename.as_mut_ptr());
    if ((*xfer).filename).is_null() {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            527 as libc::c_int,
            b"Memory allocation error\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if (*param).verbose_yn != 0 {
        printf(
            b"Request for file: '%s'\n\0" as *const u8 as *const libc::c_char,
            filename.as_mut_ptr(),
        );
    }
    (*xfer)
        .file = fopen(filename.as_mut_ptr(), b"r\0" as *const u8 as *const libc::c_char);
    if ((*xfer).file).is_null() {
        sprintf(
            g_error.as_mut_ptr(),
            b"File '%s' does not exist or cannot be read\0" as *const u8
                as *const libc::c_char,
            filename.as_mut_ptr(),
        );
        status = full_write(
            (*session).client_fd,
            b"\x08\0" as *const u8 as *const libc::c_char as *const libc::c_void,
            1 as libc::c_int as size_t,
        ) as libc::c_int;
        if status < 0 as libc::c_int {
            error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                542 as libc::c_int,
                b"Could not signal request failure to client\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            543 as libc::c_int,
            g_error.as_mut_ptr(),
            0 as libc::c_int,
        );
    }
    gettimeofday(&mut ping_s, 0 as *mut libc::c_void);
    status = full_write(
        (*session).client_fd,
        b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as size_t,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            630 as libc::c_int,
            b"Could not signal request approval to client\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if full_read(
        (*session).client_fd,
        &mut (*param).block_size as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            633 as libc::c_int,
            b"Could not read block size\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).block_size = __bswap_32((*param).block_size);
    if full_read(
        (*session).client_fd,
        &mut (*param).target_rate as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            634 as libc::c_int,
            b"Could not read target bitrate\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).target_rate = __bswap_32((*param).target_rate);
    if full_read(
        (*session).client_fd,
        &mut (*param).error_rate as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            635 as libc::c_int,
            b"Could not read error rate\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).error_rate = __bswap_32((*param).error_rate);
    gettimeofday(&mut ping_e, 0 as *mut libc::c_void);
    if full_read(
        (*session).client_fd,
        &mut (*param).slower_num as *mut u_int16_t as *mut libc::c_void,
        2 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            641 as libc::c_int,
            b"Could not read slowdown numerator\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).slower_num = __bswap_16((*param).slower_num);
    if full_read(
        (*session).client_fd,
        &mut (*param).slower_den as *mut u_int16_t as *mut libc::c_void,
        2 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            642 as libc::c_int,
            b"Could not read slowdown denominator\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).slower_den = __bswap_16((*param).slower_den);
    if full_read(
        (*session).client_fd,
        &mut (*param).faster_num as *mut u_int16_t as *mut libc::c_void,
        2 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            643 as libc::c_int,
            b"Could not read speedup numerator\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).faster_num = __bswap_16((*param).faster_num);
    if full_read(
        (*session).client_fd,
        &mut (*param).faster_den as *mut u_int16_t as *mut libc::c_void,
        2 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            644 as libc::c_int,
            b"Could not read speedup denominator\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*param).faster_den = __bswap_16((*param).faster_den);
    fseeko((*xfer).file, 0 as libc::c_int as __off64_t, 2 as libc::c_int);
    (*param).file_size = ftello((*xfer).file) as u_int64_t;
    fseeko((*xfer).file, 0 as libc::c_int as __off64_t, 0 as libc::c_int);
    (*param)
        .block_count = ((*param).file_size / (*param).block_size as u_int64_t)
        .wrapping_add(
            ((*param).file_size % (*param).block_size as u_int64_t
                != 0 as libc::c_int as u_int64_t) as libc::c_int as u_int64_t,
        ) as u_int32_t;
    (*param).epoch = time(0 as *mut time_t);
    file_size = htonll((*param).file_size);
    if full_write(
        (*session).client_fd,
        &mut file_size as *mut u_int64_t as *const libc::c_void,
        8 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            667 as libc::c_int,
            b"Could not submit file size\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    block_size = __bswap_32((*param).block_size);
    if full_write(
        (*session).client_fd,
        &mut block_size as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            668 as libc::c_int,
            b"Could not submit block size\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    block_count = __bswap_32((*param).block_count);
    if full_write(
        (*session).client_fd,
        &mut block_count as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            669 as libc::c_int,
            b"Could not submit block count\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    epoch = __bswap_32((*param).epoch as __uint32_t) as time_t;
    if full_write(
        (*session).client_fd,
        &mut epoch as *mut time_t as *const libc::c_void,
        4 as libc::c_int as size_t,
    ) < 0 as libc::c_int as ssize_t
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            670 as libc::c_int,
            b"Could not submit run epoch\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*(*session).parameter)
        .wait_u_sec = (ping_e.tv_sec - ping_s.tv_sec)
        * 1000000 as libc::c_int as __time_t + (ping_e.tv_usec - ping_s.tv_usec);
    (*(*session).parameter)
        .wait_u_sec = (*(*session).parameter).wait_u_sec
        + ((*(*session).parameter).wait_u_sec as libc::c_double * 0.1f64) as libc::c_int
            as libc::c_long;
    (*param)
        .ipd_time = (1000000 as libc::c_longlong * 8 as libc::c_int as libc::c_longlong
        * (*param).block_size as libc::c_longlong
        / (*param).target_rate as libc::c_longlong) as u_int32_t;
    (*xfer)
        .ipd_current = ((*param).ipd_time * 3 as libc::c_int as u_int32_t)
        as libc::c_double;
    if (*param).transcript_yn != 0 {
        xscript_open(session);
    }
    return 0 as libc::c_int;
}
