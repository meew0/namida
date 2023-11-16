use ::libc;
extern "C" {
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
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn setsockopt(
        __fd: libc::c_int,
        __level: libc::c_int,
        __optname: libc::c_int,
        __optval: *const libc::c_void,
        __optlen: socklen_t,
    ) -> libc::c_int;
    fn connect(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn socket(
        __domain: libc::c_int,
        __type: libc::c_int,
        __protocol: libc::c_int,
    ) -> libc::c_int;
    fn bind(
        __fd: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __len: socklen_t,
    ) -> libc::c_int;
    fn getaddrinfo(
        __name: *const libc::c_char,
        __service: *const libc::c_char,
        __req: *const addrinfo,
        __pai: *mut *mut addrinfo,
    ) -> libc::c_int;
    fn freeaddrinfo(__ai: *mut addrinfo);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type uint8_t = __uint8_t;
pub type uint16_t = __uint16_t;
pub type uint32_t = __uint32_t;
pub type size_t = libc::c_ulong;
pub type u_char = __u_char;
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
pub union __atomic_wide_counter {
    pub __value64: libc::c_ulonglong,
    pub __value32: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed {
    pub __low: libc::c_uint,
    pub __high: libc::c_uint,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_internal_list {
    pub __prev: *mut __pthread_internal_list,
    pub __next: *mut __pthread_internal_list,
}
pub type __pthread_list_t = __pthread_internal_list;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_mutex_s {
    pub __lock: libc::c_int,
    pub __count: libc::c_uint,
    pub __owner: libc::c_int,
    pub __nusers: libc::c_uint,
    pub __kind: libc::c_int,
    pub __spins: libc::c_short,
    pub __elision: libc::c_short,
    pub __list: __pthread_list_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct __pthread_cond_s {
    pub __wseq: __atomic_wide_counter,
    pub __g1_start: __atomic_wide_counter,
    pub __g_refs: [libc::c_uint; 2],
    pub __g_size: [libc::c_uint; 2],
    pub __g1_orig_size: libc::c_uint,
    pub __wrefs: libc::c_uint,
    pub __g_signals: [libc::c_uint; 2],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutex_t {
    pub __data: __pthread_mutex_s,
    pub __size: [libc::c_char; 40],
    pub __align: libc::c_long,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_cond_t {
    pub __data: __pthread_cond_s,
    pub __size: [libc::c_char; 48],
    pub __align: libc::c_longlong,
}
pub type socklen_t = __socklen_t;
pub type __socket_type = libc::c_uint;
pub const SOCK_NONBLOCK: __socket_type = 2048;
pub const SOCK_CLOEXEC: __socket_type = 524288;
pub const SOCK_PACKET: __socket_type = 10;
pub const SOCK_DCCP: __socket_type = 6;
pub const SOCK_SEQPACKET: __socket_type = 5;
pub const SOCK_RDM: __socket_type = 4;
pub const SOCK_RAW: __socket_type = 3;
pub const SOCK_DGRAM: __socket_type = 2;
pub const SOCK_STREAM: __socket_type = 1;
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
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
    pub __in6_u: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed_0 {
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
pub type C2RustUnnamed_1 = libc::c_uint;
pub const IPPROTO_MAX: C2RustUnnamed_1 = 263;
pub const IPPROTO_MPTCP: C2RustUnnamed_1 = 262;
pub const IPPROTO_RAW: C2RustUnnamed_1 = 255;
pub const IPPROTO_ETHERNET: C2RustUnnamed_1 = 143;
pub const IPPROTO_MPLS: C2RustUnnamed_1 = 137;
pub const IPPROTO_UDPLITE: C2RustUnnamed_1 = 136;
pub const IPPROTO_SCTP: C2RustUnnamed_1 = 132;
pub const IPPROTO_L2TP: C2RustUnnamed_1 = 115;
pub const IPPROTO_COMP: C2RustUnnamed_1 = 108;
pub const IPPROTO_PIM: C2RustUnnamed_1 = 103;
pub const IPPROTO_ENCAP: C2RustUnnamed_1 = 98;
pub const IPPROTO_BEETPH: C2RustUnnamed_1 = 94;
pub const IPPROTO_MTP: C2RustUnnamed_1 = 92;
pub const IPPROTO_AH: C2RustUnnamed_1 = 51;
pub const IPPROTO_ESP: C2RustUnnamed_1 = 50;
pub const IPPROTO_GRE: C2RustUnnamed_1 = 47;
pub const IPPROTO_RSVP: C2RustUnnamed_1 = 46;
pub const IPPROTO_IPV6: C2RustUnnamed_1 = 41;
pub const IPPROTO_DCCP: C2RustUnnamed_1 = 33;
pub const IPPROTO_TP: C2RustUnnamed_1 = 29;
pub const IPPROTO_IDP: C2RustUnnamed_1 = 22;
pub const IPPROTO_UDP: C2RustUnnamed_1 = 17;
pub const IPPROTO_PUP: C2RustUnnamed_1 = 12;
pub const IPPROTO_EGP: C2RustUnnamed_1 = 8;
pub const IPPROTO_TCP: C2RustUnnamed_1 = 6;
pub const IPPROTO_IPIP: C2RustUnnamed_1 = 4;
pub const IPPROTO_IGMP: C2RustUnnamed_1 = 2;
pub const IPPROTO_ICMP: C2RustUnnamed_1 = 1;
pub const IPPROTO_IP: C2RustUnnamed_1 = 0;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub struct statistics_t {
    pub start_time: timeval,
    pub stop_time: timeval,
    pub this_time: timeval,
    pub this_blocks: u_int32_t,
    pub this_retransmits: u_int32_t,
    pub total_blocks: u_int32_t,
    pub total_retransmits: u_int32_t,
    pub total_recvd_retransmits: u_int32_t,
    pub total_lost: u_int32_t,
    pub this_flow_originals: u_int32_t,
    pub this_flow_retransmitteds: u_int32_t,
    pub this_transmit_rate: libc::c_double,
    pub transmit_rate: libc::c_double,
    pub this_retransmit_rate: libc::c_double,
    pub error_rate: libc::c_double,
    pub start_udp_errors: u_int64_t,
    pub this_udp_errors: u_int64_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct retransmit_t {
    pub table: *mut u_int32_t,
    pub table_size: u_int32_t,
    pub index_max: u_int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ring_buffer_t {
    pub datagrams: *mut u_char,
    pub datagram_size: libc::c_int,
    pub base_data: libc::c_int,
    pub count_data: libc::c_int,
    pub count_reserved: libc::c_int,
    pub mutex: pthread_mutex_t,
    pub data_ready_cond: pthread_cond_t,
    pub data_ready: libc::c_int,
    pub space_ready_cond: pthread_cond_t,
    pub space_ready: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_parameter_t {
    pub server_name: *mut libc::c_char,
    pub server_port: u_int16_t,
    pub client_port: u_int16_t,
    pub udp_buffer: u_int32_t,
    pub verbose_yn: u_char,
    pub transcript_yn: u_char,
    pub ipv6_yn: u_char,
    pub output_mode: u_char,
    pub block_size: u_int32_t,
    pub target_rate: u_int32_t,
    pub rate_adjust: u_char,
    pub error_rate: u_int32_t,
    pub slower_num: u_int16_t,
    pub slower_den: u_int16_t,
    pub faster_num: u_int16_t,
    pub faster_den: u_int16_t,
    pub history: u_int16_t,
    pub lossless: u_char,
    pub losswindow_ms: u_int32_t,
    pub blockdump: u_char,
    pub passphrase: *mut libc::c_char,
    pub ringbuf: *mut libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_transfer_t {
    pub epoch: time_t,
    pub remote_filename: *const libc::c_char,
    pub local_filename: *const libc::c_char,
    pub file: *mut FILE,
    pub vsib: *mut FILE,
    pub transcript: *mut FILE,
    pub udp_fd: libc::c_int,
    pub file_size: u_int64_t,
    pub block_count: u_int32_t,
    pub next_block: u_int32_t,
    pub gapless_to_block: u_int32_t,
    pub retransmit: retransmit_t,
    pub stats: statistics_t,
    pub ring_buffer: *mut ring_buffer_t,
    pub received: *mut u_char,
    pub blocks_left: u_int32_t,
    pub restart_pending: u_char,
    pub restart_lastidx: u_int32_t,
    pub restart_wireclearidx: u_int32_t,
    pub on_wire_estimate: u_int32_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ttp_session_t {
    pub parameter: *mut ttp_parameter_t,
    pub transfer: ttp_transfer_t,
    pub server: *mut FILE,
    pub server_address: *mut sockaddr,
    pub server_address_length: socklen_t,
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
#[no_mangle]
pub unsafe extern "C" fn create_tcp_socket_client(
    mut session: *mut ttp_session_t,
    mut server_name: *const libc::c_char,
    mut server_port: u_int16_t,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    let mut info_save: *mut addrinfo = 0 as *mut addrinfo;
    let mut buffer: [libc::c_char; 10] = [0; 10];
    let mut socket_fd: libc::c_int = 0;
    let mut yes: libc::c_int = 1 as libc::c_int;
    let mut status: libc::c_int = 0;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints
        .ai_family = if (*(*session).parameter).ipv6_yn as libc::c_int != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    hints.ai_socktype = SOCK_STREAM as libc::c_int;
    sprintf(
        buffer.as_mut_ptr(),
        b"%d\0" as *const u8 as *const libc::c_char,
        server_port as libc::c_int,
    );
    status = getaddrinfo(server_name, buffer.as_mut_ptr(), &mut hints, &mut info);
    if status != 0 {
        return error_handler(
            b"network.c\0" as *const u8 as *const libc::c_char,
            106 as libc::c_int,
            b"Error in getting address information for server\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    info_save = info;
    loop {
        socket_fd = socket((*info).ai_family, (*info).ai_socktype, (*info).ai_protocol);
        if socket_fd < 0 as libc::c_int {
            error_handler(
                b"network.c\0" as *const u8 as *const libc::c_char,
                115 as libc::c_int,
                b"Could not create socket\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            status = setsockopt(
                socket_fd,
                1 as libc::c_int,
                2 as libc::c_int,
                &mut yes as *mut libc::c_int as *const libc::c_void,
                ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
            );
            if status < 0 as libc::c_int {
                error_handler(
                    b"network.c\0" as *const u8 as *const libc::c_char,
                    122 as libc::c_int,
                    b"Could not make socket reusable\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                );
                close(socket_fd);
            } else {
                status = setsockopt(
                    socket_fd,
                    IPPROTO_TCP as libc::c_int,
                    1 as libc::c_int,
                    &mut yes as *mut libc::c_int as *const libc::c_void,
                    ::core::mem::size_of::<libc::c_int>() as libc::c_ulong as socklen_t,
                );
                if status < 0 as libc::c_int {
                    error_handler(
                        b"network.c\0" as *const u8 as *const libc::c_char,
                        130 as libc::c_int,
                        b"Could not disable Nagle's algorithm\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                    );
                    close(socket_fd);
                } else {
                    status = connect(
                        socket_fd,
                        __CONST_SOCKADDR_ARG {
                            __sockaddr__: (*info).ai_addr,
                        },
                        (*info).ai_addrlen,
                    );
                    if status == 0 as libc::c_int {
                        (*session)
                            .server_address = malloc((*info).ai_addrlen as libc::c_ulong)
                            as *mut sockaddr;
                        (*session).server_address_length = (*info).ai_addrlen;
                        if ((*session).server_address).is_null() {
                            error_handler(
                                b"network.c\0" as *const u8 as *const libc::c_char,
                                143 as libc::c_int,
                                b"Could not allocate space for server address\0"
                                    as *const u8 as *const libc::c_char,
                                1 as libc::c_int,
                            );
                        }
                        memcpy(
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
    freeaddrinfo(info_save);
    if info.is_null() {
        return error_handler(
            b"network.c\0" as *const u8 as *const libc::c_char,
            155 as libc::c_int,
            b"Error in connecting to Tsunami server\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return socket_fd;
}
#[no_mangle]
pub unsafe extern "C" fn create_udp_socket_client(
    mut parameter: *mut ttp_parameter_t,
) -> libc::c_int {
    let mut hints: addrinfo = addrinfo {
        ai_flags: 0,
        ai_family: 0,
        ai_socktype: 0,
        ai_protocol: 0,
        ai_addrlen: 0,
        ai_addr: 0 as *mut sockaddr,
        ai_canonname: 0 as *mut libc::c_char,
        ai_next: 0 as *mut addrinfo,
    };
    let mut info: *mut addrinfo = 0 as *mut addrinfo;
    let mut info_save: *mut addrinfo = 0 as *mut addrinfo;
    let mut buffer: [libc::c_char; 10] = [0; 10];
    let mut socket_fd: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut higher_port_attempt: libc::c_int = 0 as libc::c_int;
    memset(
        &mut hints as *mut addrinfo as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<addrinfo>() as libc::c_ulong,
    );
    hints.ai_flags = 0x1 as libc::c_int;
    hints
        .ai_family = if (*parameter).ipv6_yn as libc::c_int != 0 {
        10 as libc::c_int
    } else {
        2 as libc::c_int
    };
    hints.ai_socktype = SOCK_DGRAM as libc::c_int;
    loop {
        sprintf(
            buffer.as_mut_ptr(),
            b"%d\0" as *const u8 as *const libc::c_char,
            (*parameter).client_port as libc::c_int + higher_port_attempt,
        );
        status = getaddrinfo(
            0 as *const libc::c_char,
            buffer.as_mut_ptr(),
            &mut hints,
            &mut info,
        );
        if status != 0 {
            return error_handler(
                b"network.c\0" as *const u8 as *const libc::c_char,
                195 as libc::c_int,
                b"Error in getting address information\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        info_save = info;
        loop {
            socket_fd = socket(
                (*info).ai_family,
                (*info).ai_socktype,
                (*info).ai_protocol,
            );
            if !(socket_fd < 0 as libc::c_int) {
                status = setsockopt(
                    socket_fd,
                    1 as libc::c_int,
                    8 as libc::c_int,
                    &mut (*parameter).udp_buffer as *mut u_int32_t
                        as *const libc::c_void,
                    ::core::mem::size_of::<u_int32_t>() as libc::c_ulong as socklen_t,
                );
                if status < 0 as libc::c_int {
                    error_handler(
                        b"network.c\0" as *const u8 as *const libc::c_char,
                        211 as libc::c_int,
                        b"Error in resizing UDP receive buffer\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                    );
                }
                status = bind(
                    socket_fd,
                    __CONST_SOCKADDR_ARG {
                        __sockaddr__: (*info).ai_addr,
                    },
                    (*info).ai_addrlen,
                );
                if status == 0 as libc::c_int {
                    (*parameter)
                        .client_port = __bswap_16(
                        (*((*info).ai_addr as *mut sockaddr_in)).sin_port,
                    );
                    fprintf(
                        stderr,
                        b"Receiving data on UDP port %d\n\0" as *const u8
                            as *const libc::c_char,
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
        freeaddrinfo(info_save);
        higher_port_attempt += 1;
        if !(higher_port_attempt < 256 as libc::c_int && info.is_null()) {
            break;
        }
    }
    if higher_port_attempt > 1 as libc::c_int {
        fprintf(
            stderr,
            b"Warning: there are %d other Tsunami clients running\n\0" as *const u8
                as *const libc::c_char,
            higher_port_attempt - 1 as libc::c_int,
        );
    }
    if info.is_null() {
        return error_handler(
            b"network.c\0" as *const u8 as *const libc::c_char,
            235 as libc::c_int,
            b"Error in creating UDP socket\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return socket_fd;
}
