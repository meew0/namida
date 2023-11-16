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
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn rindex(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn getsockname(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __len: *mut socklen_t,
    ) -> libc::c_int;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn time(__timer: *mut time_t) -> time_t;
    fn ctime(__timer: *const time_t) -> *mut libc::c_char;
    fn access(__name: *const libc::c_char, __type: libc::c_int) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    static mut stdout: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn xscript_data_log_client(session: *mut ttp_session_t, logline: *const libc::c_char);
    static PROTOCOL_REVISION: u_int32_t;
    static REQUEST_RETRANSMIT: u_int16_t;
    static REQUEST_RESTART: u_int16_t;
    static REQUEST_STOP: u_int16_t;
    static REQUEST_ERROR_RATE: u_int16_t;
    fn get_usec_since(old_time: *mut timeval) -> u_int64_t;
    fn ntohll(value: u_int64_t) -> u_int64_t;
    fn prepare_proof(
        buffer: *mut u_char,
        bytes: size_t,
        secret: *const u_char,
        digest: *mut u_char,
    ) -> *mut u_char;
    fn get_udp_in_errors() -> u_int64_t;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    fn got_block(session: *mut ttp_session_t, blocknr: u_int32_t) -> libc::c_int;
    fn create_udp_socket_client(parameter: *mut ttp_parameter_t) -> libc::c_int;
    fn xscript_open_client(session: *mut ttp_session_t);
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
pub type __socklen_t = libc::c_uint;
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
pub type uint32_t = __uint32_t;
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
pub type uint16_t = __uint16_t;
pub type uint8_t = __uint8_t;
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
#[inline]
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_authenticate_client(
    mut session: *mut ttp_session_t,
    mut secret: *mut u_char,
) -> libc::c_int {
    let mut random: [u_char; 64] = [0; 64];
    let mut digest: [u_char; 16] = [0; 16];
    let mut result: u_char = 0;
    let mut status: libc::c_int = 0;
    status = fread(
        random.as_mut_ptr() as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        64 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 64 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            104 as libc::c_int,
            b"Could not read authentication challenge from server\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    prepare_proof(
        random.as_mut_ptr(),
        64 as libc::c_int as size_t,
        secret,
        digest.as_mut_ptr(),
    );
    while *secret != 0 {
        let fresh0 = secret;
        secret = secret.offset(1);
        *fresh0 = '\0' as i32 as u_char;
    }
    status = fwrite(
        digest.as_mut_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        16 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 16 as libc::c_int || fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            b"Could not send authentication response\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = fread(
        &mut result as *mut u_char as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 1 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            119 as libc::c_int,
            b"Could not read authentication status\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return if result as libc::c_int == 0 as libc::c_int {
        0 as libc::c_int
    } else {
        -(1 as libc::c_int)
    };
}
#[no_mangle]
pub unsafe extern "C" fn ttp_negotiate_client(mut session: *mut ttp_session_t) -> libc::c_int {
    let mut server_revision: u_int32_t = 0;
    let mut client_revision: u_int32_t = __bswap_32(PROTOCOL_REVISION);
    let mut status: libc::c_int = 0;
    status = fwrite(
        &mut client_revision as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 1 as libc::c_int || fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            145 as libc::c_int,
            b"Could not send protocol revision number\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = fread(
        &mut server_revision as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 1 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            150 as libc::c_int,
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
pub unsafe extern "C" fn ttp_open_transfer_client(
    mut session: *mut ttp_session_t,
    mut remote_filename: *const libc::c_char,
    mut local_filename: *const libc::c_char,
) -> libc::c_int {
    let mut result: u_char = 0;
    let mut temp: u_int32_t = 0;
    let mut temp16: u_int16_t = 0;
    let mut status: libc::c_int = 0;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    status = fprintf(
        (*session).server,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        remote_filename,
    );
    if status <= 0 as libc::c_int || fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            180 as libc::c_int,
            b"Could not request file\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = fread(
        &mut result as *mut u_char as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 1 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            185 as libc::c_int,
            b"Could not read response to file request\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if result as libc::c_int != 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            189 as libc::c_int,
            b"Server: File does not exist or cannot be transmitted\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp = __bswap_32((*param).block_size);
    if fwrite(
        &mut temp as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            b"Could not submit block size\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp = __bswap_32((*param).target_rate);
    if fwrite(
        &mut temp as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            193 as libc::c_int,
            b"Could not submit target rate\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp = __bswap_32((*param).error_rate);
    if fwrite(
        &mut temp as *mut u_int32_t as *const libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            194 as libc::c_int,
            b"Could not submit error rate\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            196 as libc::c_int,
            b"Could not flush control channel\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp16 = __bswap_16((*param).slower_num);
    if fwrite(
        &mut temp16 as *mut u_int16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            199 as libc::c_int,
            b"Could not submit slowdown numerator\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp16 = __bswap_16((*param).slower_den);
    if fwrite(
        &mut temp16 as *mut u_int16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            200 as libc::c_int,
            b"Could not submit slowdown denominator\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp16 = __bswap_16((*param).faster_num);
    if fwrite(
        &mut temp16 as *mut u_int16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            201 as libc::c_int,
            b"Could not submit speedup numerator\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    temp16 = __bswap_16((*param).faster_den);
    if fwrite(
        &mut temp16 as *mut u_int16_t as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            202 as libc::c_int,
            b"Could not submit speedup denominator\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            204 as libc::c_int,
            b"Could not flush control channel\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    memset(
        xfer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_transfer_t>() as libc::c_ulong,
    );
    (*xfer).remote_filename = remote_filename;
    (*xfer).local_filename = local_filename;
    if fread(
        &mut (*xfer).file_size as *mut u_int64_t as *mut libc::c_void,
        8 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            212 as libc::c_int,
            b"Could not read file size\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*xfer).file_size = ntohll((*xfer).file_size);
    if fread(
        &mut temp as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            b"Could not read block size\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if __bswap_32(temp) != (*param).block_size {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            213 as libc::c_int,
            b"Block size disagreement\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if fread(
        &mut (*xfer).block_count as *mut u_int32_t as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            b"Could not read number of blocks\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*xfer).block_count = __bswap_32((*xfer).block_count);
    if fread(
        &mut (*xfer).epoch as *mut time_t as *mut libc::c_void,
        4 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) < 1 as libc::c_int as libc::c_ulong
    {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
            b"Could not read run epoch\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    (*xfer).epoch = __bswap_32((*xfer).epoch as __uint32_t) as time_t;
    (*xfer).blocks_left = (*xfer).block_count;
    if access((*xfer).local_filename, 0 as libc::c_int) == 0 {
        printf(
            b"Warning: overwriting existing file '%s'\n\0" as *const u8
                as *const libc::c_char,
            local_filename,
        );
    }
    (*xfer)
        .file = fopen(
        (*xfer).local_filename,
        b"wb\0" as *const u8 as *const libc::c_char,
    );
    if ((*xfer).file).is_null() {
        let mut trimmed: *mut libc::c_char = rindex((*xfer).local_filename, '/' as i32);
        if !trimmed.is_null() && strlen(trimmed) > 1 as libc::c_int as libc::c_ulong {
            printf(
                b"Warning: could not open file %s for writing, trying local directory instead.\n\0"
                    as *const u8 as *const libc::c_char,
                (*xfer).local_filename,
            );
            (*xfer).local_filename = trimmed.offset(1 as libc::c_int as isize);
            if access((*xfer).local_filename, 0 as libc::c_int) == 0 {
                printf(
                    b"Warning: overwriting existing file '%s'\n\0" as *const u8
                        as *const libc::c_char,
                    (*xfer).local_filename,
                );
            }
            (*xfer)
                .file = fopen(
                (*xfer).local_filename,
                b"wb\0" as *const u8 as *const libc::c_char,
            );
        }
        if ((*xfer).file).is_null() {
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                234 as libc::c_int,
                b"Could not open local file for writing\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
        }
    }
    (*xfer)
        .on_wire_estimate = (0.5f64 * (*param).target_rate as libc::c_double
        / (8 as libc::c_int as u_int32_t * (*param).block_size) as libc::c_double)
        as u_int32_t;
    (*xfer)
        .on_wire_estimate = if (*xfer).block_count < (*xfer).on_wire_estimate {
        (*xfer).block_count
    } else {
        (*xfer).on_wire_estimate
    };
    if (*param).transcript_yn != 0 {
        xscript_open_client(session);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_open_port_client(mut session: *mut ttp_session_t) -> libc::c_int {
    let mut udp_address: sockaddr = sockaddr {
        sa_family: 0,
        sa_data: [0; 14],
    };
    let mut udp_length: libc::c_uint = ::core::mem::size_of::<sockaddr>()
        as libc::c_ulong as libc::c_uint;
    let mut status: libc::c_int = 0;
    let mut port: *mut u_int16_t = 0 as *mut u_int16_t;
    (*session).transfer.udp_fd = create_udp_socket_client((*session).parameter);
    if (*session).transfer.udp_fd < 0 as libc::c_int {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            280 as libc::c_int,
            b"Could not create UDP socket\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    memset(
        &mut udp_address as *mut sockaddr as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<sockaddr>() as libc::c_ulong,
    );
    getsockname(
        (*session).transfer.udp_fd,
        __SOCKADDR_ARG {
            __sockaddr__: &mut udp_address as *mut sockaddr,
        },
        &mut udp_length,
    );
    port = if (*(*session).parameter).ipv6_yn as libc::c_int != 0 {
        &mut (*(&mut udp_address as *mut sockaddr as *mut sockaddr_in6)).sin6_port
    } else {
        &mut (*(&mut udp_address as *mut sockaddr as *mut sockaddr_in)).sin_port
    };
    status = fwrite(
        port as *const libc::c_void,
        2 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status < 1 as libc::c_int || fflush((*session).server) != 0 {
        close((*session).transfer.udp_fd);
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            293 as libc::c_int,
            b"Could not send UDP port number\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_repeat_retransmit(
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    let mut retransmission: [retransmission_t; 2048] = [retransmission_t {
        request_type: 0,
        block: 0,
        error_rate: 0,
    }; 2048];
    let mut entry: libc::c_int = 0;
    let mut status: libc::c_int = 0;
    let mut block: libc::c_int = 0;
    let mut count: libc::c_int = 0 as libc::c_int;
    let mut rexmit: *mut retransmit_t = &mut (*session).transfer.retransmit;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    memset(
        retransmission.as_mut_ptr() as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<[retransmission_t; 2048]>() as libc::c_ulong,
    );
    (*xfer).stats.this_retransmits = 0 as libc::c_int as u_int32_t;
    count = 0 as libc::c_int;
    entry = 0 as libc::c_int;
    while (entry as u_int32_t) < (*rexmit).index_max && count < 2048 as libc::c_int {
        block = *((*rexmit).table).offset(entry as isize) as libc::c_int;
        if block != 0 && got_block(session, block as u_int32_t) == 0 {
            *((*rexmit).table).offset(count as isize) = block as u_int32_t;
            retransmission[count as usize].request_type = __bswap_16(REQUEST_RETRANSMIT);
            retransmission[count as usize].block = __bswap_32(block as __uint32_t);
            count += 1;
            count;
        }
        entry += 1;
        entry;
    }
    if count >= 2048 as libc::c_int {
        block = (if (*xfer).block_count
            < ((*xfer).gapless_to_block).wrapping_add(1 as libc::c_int as u_int32_t)
        {
            (*xfer).block_count
        } else {
            ((*xfer).gapless_to_block).wrapping_add(1 as libc::c_int as u_int32_t)
        }) as libc::c_int;
        retransmission[0 as libc::c_int as usize]
            .request_type = __bswap_16(REQUEST_RESTART);
        retransmission[0 as libc::c_int as usize]
            .block = __bswap_32(block as __uint32_t);
        status = fwrite(
            &mut *retransmission.as_mut_ptr().offset(0 as libc::c_int as isize)
                as *mut retransmission_t as *const libc::c_void,
            ::core::mem::size_of::<retransmission_t>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            (*session).server,
        ) as libc::c_int;
        if status <= 0 as libc::c_int {
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                363 as libc::c_int,
                b"Could not send restart-at request\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        (*xfer).restart_pending = 1 as libc::c_int as u_char;
        (*xfer)
            .restart_lastidx = *((*rexmit).table)
            .offset(
                ((*rexmit).index_max).wrapping_sub(1 as libc::c_int as u_int32_t)
                    as isize,
            );
        (*xfer)
            .restart_wireclearidx = if (*xfer).block_count
            < ((*xfer).restart_lastidx).wrapping_add((*xfer).on_wire_estimate)
        {
            (*xfer).block_count
        } else {
            ((*xfer).restart_lastidx).wrapping_add((*xfer).on_wire_estimate)
        };
        (*rexmit).index_max = 0 as libc::c_int as u_int32_t;
        (*xfer).next_block = block as u_int32_t;
        (*xfer).stats.this_retransmits = 2048 as libc::c_int as u_int32_t;
    } else {
        (*rexmit).index_max = count as u_int32_t;
        (*xfer).stats.this_retransmits = count as u_int32_t;
        (*xfer)
            .stats
            .total_retransmits = ((*xfer).stats.total_retransmits)
            .wrapping_add(count as u_int32_t);
        if count > 0 as libc::c_int {
            status = fwrite(
                retransmission.as_mut_ptr() as *const libc::c_void,
                ::core::mem::size_of::<retransmission_t>() as libc::c_ulong,
                count as libc::c_ulong,
                (*session).server,
            ) as libc::c_int;
            if status <= 0 as libc::c_int {
                return error_handler(
                    b"protocol.c\0" as *const u8 as *const libc::c_char,
                    396 as libc::c_int,
                    b"Could not send retransmit requests\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
        }
    }
    if fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            404 as libc::c_int,
            b"Could not flush retransmit requests\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_request_retransmit(
    mut session: *mut ttp_session_t,
    mut block: u_int32_t,
) -> libc::c_int {
    let mut ptr: *mut u_int32_t = 0 as *mut u_int32_t;
    let mut rexmit: *mut retransmit_t = &mut (*session).transfer.retransmit;
    if got_block(session, block) != 0 {
        return 0 as libc::c_int;
    }
    if (*rexmit).index_max >= (*rexmit).table_size {
        if (*rexmit).index_max >= (32 as libc::c_int * 2048 as libc::c_int) as u_int32_t
        {
            return 0 as libc::c_int;
        }
        ptr = realloc(
            (*rexmit).table as *mut libc::c_void,
            (2 as libc::c_int as libc::c_ulong)
                .wrapping_mul(::core::mem::size_of::<u_int32_t>() as libc::c_ulong)
                .wrapping_mul((*rexmit).table_size as libc::c_ulong),
        ) as *mut u_int32_t;
        if ptr.is_null() {
            return error_handler(
                b"protocol.c\0" as *const u8 as *const libc::c_char,
                446 as libc::c_int,
                b"Could not grow retransmission table\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        (*rexmit).table = ptr;
        memset(
            ((*rexmit).table).offset((*rexmit).table_size as isize) as *mut libc::c_void,
            0 as libc::c_int,
            (::core::mem::size_of::<u_int32_t>() as libc::c_ulong)
                .wrapping_mul((*rexmit).table_size as libc::c_ulong),
        );
        (*rexmit).table_size = (*rexmit).table_size * 2 as libc::c_int as u_int32_t;
    }
    *((*rexmit).table).offset((*rexmit).index_max as isize) = block;
    (*rexmit).index_max = ((*rexmit).index_max).wrapping_add(1);
    (*rexmit).index_max;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_request_stop(
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    let mut retransmission: retransmission_t = {
        let mut init = retransmission_t {
            request_type: 0 as libc::c_int as u_int16_t,
            block: 0 as libc::c_int as u_int32_t,
            error_rate: 0 as libc::c_int as u_int32_t,
        };
        init
    };
    let mut status: libc::c_int = 0;
    retransmission.request_type = __bswap_16(REQUEST_STOP);
    status = fwrite(
        &mut retransmission as *mut retransmission_t as *const libc::c_void,
        ::core::mem::size_of::<retransmission_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status <= 0 as libc::c_int || fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            522 as libc::c_int,
            b"Could not request end of transmission\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ttp_update_stats(
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    let mut now_epoch: time_t = time(0 as *mut time_t);
    let mut delta: u_int64_t = 0;
    let mut d_seconds: libc::c_double = 0.;
    let mut delta_total: u_int64_t = 0;
    let mut d_seconds_total: libc::c_double = 0.;
    let mut temp: u_int64_t = 0;
    let mut hours: libc::c_int = 0;
    let mut minutes: libc::c_int = 0;
    let mut seconds: libc::c_int = 0;
    let mut milliseconds: libc::c_int = 0;
    let mut data_total: libc::c_double = 0.;
    let mut data_total_rate: libc::c_double = 0.;
    let mut data_this: libc::c_double = 0.;
    let mut data_this_rexmit: libc::c_double = 0.;
    let mut data_this_goodpt: libc::c_double = 0.;
    let mut retransmits_fraction: libc::c_double = 0.;
    let mut total_retransmits_fraction: libc::c_double = 0.;
    let mut ringfill_fraction: libc::c_double = 0.;
    let mut stats: *mut statistics_t = &mut (*session).transfer.stats;
    let mut retransmission: retransmission_t = retransmission_t {
        request_type: 0,
        block: 0,
        error_rate: 0,
    };
    let mut status: libc::c_int = 0;
    static mut iteration: u_int32_t = 0 as libc::c_int as u_int32_t;
    static mut stats_line: [libc::c_char; 128] = [0; 128];
    static mut stats_flags: [libc::c_char; 8] = [0; 8];
    let mut ff: libc::c_double = 0.;
    let mut fb: libc::c_double = 0.;
    let u_mega: libc::c_double = (1024 as libc::c_int * 1024 as libc::c_int)
        as libc::c_double;
    let u_giga: libc::c_double = (1024 as libc::c_int * 1024 as libc::c_int
        * 1024 as libc::c_int) as libc::c_double;
    delta = get_usec_since(&mut (*stats).this_time);
    temp = get_usec_since(&mut (*stats).start_time);
    delta_total = temp;
    milliseconds = (temp % 1000000 as libc::c_int as u_int64_t
        / 1000 as libc::c_int as u_int64_t) as libc::c_int;
    temp = temp / 1000000 as libc::c_int as u_int64_t;
    seconds = (temp % 60 as libc::c_int as u_int64_t) as libc::c_int;
    temp = temp / 60 as libc::c_int as u_int64_t;
    minutes = (temp % 60 as libc::c_int as u_int64_t) as libc::c_int;
    temp = temp / 60 as libc::c_int as u_int64_t;
    hours = temp as libc::c_int;
    d_seconds = delta as libc::c_double / 1e6f64;
    d_seconds_total = delta_total as libc::c_double / 1e6f64;
    data_total = (*(*session).parameter).block_size as libc::c_double
        * (*stats).total_blocks as libc::c_double;
    data_this = (*(*session).parameter).block_size as libc::c_double
        * ((*stats).total_blocks).wrapping_sub((*stats).this_blocks) as libc::c_double;
    data_this_rexmit = (*(*session).parameter).block_size as libc::c_double
        * (*stats).this_flow_retransmitteds as libc::c_double;
    data_this_goodpt = (*(*session).parameter).block_size as libc::c_double
        * (*stats).this_flow_originals as libc::c_double;
    (*stats).this_udp_errors = get_udp_in_errors();
    retransmits_fraction = (*stats).this_retransmits as libc::c_double
        / (1.0f64 + (*stats).this_retransmits as libc::c_double
            + (*stats).total_blocks as libc::c_double
            - (*stats).this_blocks as libc::c_double);
    ringfill_fraction = ((*(*session).transfer.ring_buffer).count_data
        / 4096 as libc::c_int) as libc::c_double;
    total_retransmits_fraction = ((*stats).total_retransmits
        / ((*stats).total_retransmits).wrapping_add((*stats).total_blocks))
        as libc::c_double;
    (*stats).this_transmit_rate = 8.0f64 * data_this / (d_seconds * u_mega);
    (*stats).this_retransmit_rate = 8.0f64 * data_this_rexmit / (d_seconds * u_mega);
    data_total_rate = 8.0f64 * data_total / (d_seconds_total * u_mega);
    fb = (*(*session).parameter).history as libc::c_int as libc::c_double / 100.0f64;
    ff = 1.0f64 - fb;
    (*stats)
        .transmit_rate = fb * (*stats).transmit_rate + ff * (*stats).this_transmit_rate;
    (*stats)
        .error_rate = fb * (*stats).error_rate
        + ff * 500 as libc::c_int as libc::c_double
            * 100 as libc::c_int as libc::c_double
            * (retransmits_fraction + ringfill_fraction);
    memset(
        &mut retransmission as *mut retransmission_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<retransmission_t>() as libc::c_ulong,
    );
    retransmission.request_type = __bswap_16(REQUEST_ERROR_RATE);
    retransmission
        .error_rate = __bswap_32(
        (*session).transfer.stats.error_rate as u_int64_t as __uint32_t,
    );
    status = fwrite(
        &mut retransmission as *mut retransmission_t as *const libc::c_void,
        ::core::mem::size_of::<retransmission_t>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as libc::c_int;
    if status <= 0 as libc::c_int || fflush((*session).server) != 0 {
        return error_handler(
            b"protocol.c\0" as *const u8 as *const libc::c_char,
            619 as libc::c_int,
            b"Could not send error rate information\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    sprintf(
        stats_flags.as_mut_ptr(),
        b"%c%c\0" as *const u8 as *const libc::c_char,
        if (*session).transfer.restart_pending as libc::c_int != 0 {
            'R' as i32
        } else {
            '-' as i32
        },
        if (*(*session).transfer.ring_buffer).space_ready == 0 {
            'F' as i32
        } else {
            '-' as i32
        },
    );
    sprintf(
        stats_line.as_mut_ptr(),
        b"%02d:%02d:%02d.%03d %4u %6.2fM %6.1fMbps %5.1f%% %7u %6.1fG %6.1fMbps %5.1f%% %5d %5d %7u %8u %8Lu %s\n\0"
            as *const u8 as *const libc::c_char,
        hours,
        minutes,
        seconds,
        milliseconds,
        ((*stats).total_blocks).wrapping_sub((*stats).this_blocks),
        (*stats).this_retransmit_rate,
        (*stats).this_transmit_rate,
        100.0f64 * retransmits_fraction,
        (*session).transfer.stats.total_blocks,
        data_total / u_giga,
        data_total_rate,
        100.0f64 * total_retransmits_fraction,
        (*session).transfer.retransmit.index_max,
        (*(*session).transfer.ring_buffer).count_data,
        (*session).transfer.blocks_left,
        (*stats).this_retransmits,
        ((*stats).this_udp_errors).wrapping_sub((*stats).start_udp_errors) as ull_t,
        stats_flags.as_mut_ptr(),
    );
    if (*(*session).parameter).verbose_yn != 0 {
        if (*(*session).parameter).output_mode as libc::c_int == 0 as libc::c_int {
            printf(b"\x1B[2J\x1B[H\0" as *const u8 as *const libc::c_char);
            printf(
                b"Current time:   %s\n\0" as *const u8 as *const libc::c_char,
                ctime(&mut now_epoch),
            );
            printf(
                b"Elapsed time:   %02d:%02d:%02d.%03d\n\n\0" as *const u8
                    as *const libc::c_char,
                hours,
                minutes,
                seconds,
                milliseconds,
            );
            printf(
                b"Last interval\n--------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(
                b"Blocks count:     %u\n\0" as *const u8 as *const libc::c_char,
                ((*stats).total_blocks).wrapping_sub((*stats).this_blocks),
            );
            printf(
                b"Data transferred: %0.2f GB\n\0" as *const u8 as *const libc::c_char,
                data_this / u_giga,
            );
            printf(
                b"Transfer rate:    %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                (*stats).this_transmit_rate,
            );
            printf(
                b"Retransmissions:  %u (%0.2f%%)\n\n\0" as *const u8
                    as *const libc::c_char,
                (*stats).this_retransmits,
                100.0f64 * retransmits_fraction,
            );
            printf(
                b"Cumulative\n--------------------------------------------------\n\0"
                    as *const u8 as *const libc::c_char,
            );
            printf(
                b"Blocks count:     %u\n\0" as *const u8 as *const libc::c_char,
                (*session).transfer.stats.total_blocks,
            );
            printf(
                b"Data transferred: %0.2f GB\n\0" as *const u8 as *const libc::c_char,
                data_total / u_giga,
            );
            printf(
                b"Transfer rate:    %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                data_total_rate,
            );
            printf(
                b"Retransmissions:  %u (%0.2f%%)\n\0" as *const u8
                    as *const libc::c_char,
                (*stats).total_retransmits,
                100.0f64 * total_retransmits_fraction,
            );
            printf(
                b"Flags          :  %s\n\n\0" as *const u8 as *const libc::c_char,
                stats_flags.as_mut_ptr(),
            );
            printf(
                b"OS UDP rx errors: %llu\n\0" as *const u8 as *const libc::c_char,
                ((*stats).this_udp_errors).wrapping_sub((*stats).start_udp_errors)
                    as ull_t,
            );
        } else {
            let fresh1 = iteration;
            iteration = iteration.wrapping_add(1);
            if fresh1 % 23 as libc::c_int as u_int32_t == 0 {
                printf(
                    b"             last_interval                   transfer_total                   buffers      transfer_remaining  OS UDP\n\0"
                        as *const u8 as *const libc::c_char,
                );
                printf(
                    b"time          blk    data       rate rexmit     blk    data       rate rexmit queue  ring     blk   rt_len      err \n\0"
                        as *const u8 as *const libc::c_char,
                );
            }
            printf(b"%s\0" as *const u8 as *const libc::c_char, stats_line.as_mut_ptr());
        }
        fflush(stdout);
    }
    if (*(*session).parameter).transcript_yn != 0 {
        xscript_data_log_client(session, stats_line.as_mut_ptr());
    }
    (*stats).this_blocks = (*stats).total_blocks;
    (*stats).this_retransmits = 0 as libc::c_int as u_int32_t;
    (*stats).this_flow_originals = 0 as libc::c_int as u_int32_t;
    (*stats).this_flow_retransmitteds = 0 as libc::c_int as u_int32_t;
    gettimeofday(&mut (*stats).this_time, 0 as *mut libc::c_void);
    return 0 as libc::c_int;
}
