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
    fn pthread_create(
        __newthread: *mut pthread_t,
        __attr: *const pthread_attr_t,
        __start_routine: Option<unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void>,
        __arg: *mut libc::c_void,
    ) -> libc::c_int;
    fn pthread_join(__th: pthread_t, __thread_return: *mut *mut libc::c_void) -> libc::c_int;
    fn strtod(_: *const libc::c_char, _: *mut *mut libc::c_char) -> libc::c_double;
    fn strtol(_: *const libc::c_char, _: *mut *mut libc::c_char, _: libc::c_int) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn exit(_: libc::c_int) -> !;
    fn memcpy(_: *mut libc::c_void, _: *const libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    fn recvfrom(
        __fd: libc::c_int,
        __buf: *mut libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> ssize_t;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn __ctype_toupper_loc() -> *mut *const __int32_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fdopen(__fd: libc::c_int, __modes: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
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
    fn ttp_open_transfer_client(
        session: *mut ttp_session_t,
        remote_filename: *const libc::c_char,
        local_filename: *const libc::c_char,
    ) -> libc::c_int;
    fn ttp_open_port_client(session: *mut ttp_session_t) -> libc::c_int;
    fn ring_create(session: *mut ttp_session_t) -> *mut ring_buffer_t;
    fn ring_peek(ring: *mut ring_buffer_t) -> *mut u_char;
    fn accept_block(
        session: *mut ttp_session_t,
        block_index: u_int32_t,
        block: *mut u_char,
    ) -> libc::c_int;
    fn ring_pop(ring: *mut ring_buffer_t) -> libc::c_int;
    fn xscript_data_start_client(session: *mut ttp_session_t, epoch: *const timeval);
    fn ring_full(ring: *mut ring_buffer_t) -> libc::c_int;
    static mut g_error: [libc::c_char; 0];
    fn get_usec_since(old_time: *mut timeval) -> u_int64_t;
    fn fread_line(f: *mut FILE, buffer: *mut libc::c_char, buffer_length: size_t) -> libc::c_int;
    fn get_udp_in_errors() -> u_int64_t;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    fn ttp_request_retransmit(session: *mut ttp_session_t, block: u_int32_t) -> libc::c_int;
    fn ttp_authenticate_client(session: *mut ttp_session_t, secret: *mut u_char) -> libc::c_int;
    fn ttp_negotiate_client(session: *mut ttp_session_t) -> libc::c_int;
    fn create_tcp_socket_client(
        session: *mut ttp_session_t,
        server_name: *const libc::c_char,
        server_port: u_int16_t,
    ) -> libc::c_int;
    fn ttp_repeat_retransmit(session: *mut ttp_session_t) -> libc::c_int;
    fn ring_destroy(ring: *mut ring_buffer_t) -> libc::c_int;
    fn ttp_update_stats(session: *mut ttp_session_t) -> libc::c_int;
    fn xscript_close_client(session: *mut ttp_session_t, delta: u_int64_t);
    fn xscript_data_stop_client(session: *mut ttp_session_t, epoch: *const timeval);
    fn ttp_request_stop(session: *mut ttp_session_t) -> libc::c_int;
    fn ring_confirm(ring: *mut ring_buffer_t) -> libc::c_int;
    fn ring_reserve(ring: *mut ring_buffer_t) -> *mut u_char;
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __int32_t = libc::c_int;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type time_t = __time_t;
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
pub type pthread_t = libc::c_ulong;
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_attr_t {
    pub __size: [libc::c_char; 56],
    pub __align: libc::c_long,
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
pub type u_char = __u_char;
pub type ssize_t = __ssize_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
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
pub struct command_t {
    pub count: u_char,
    pub text: [*const libc::c_char; 10],
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn atol(mut __nptr: *const libc::c_char) -> libc::c_long {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    );
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
#[inline]
unsafe extern "C" fn atof(mut __nptr: *const libc::c_char) -> libc::c_double {
    return strtod(__nptr, 0 as *mut libc::c_void as *mut *mut libc::c_char);
}
#[inline]
unsafe extern "C" fn toupper(mut __c: libc::c_int) -> libc::c_int {
    return if __c >= -(128 as libc::c_int) && __c < 256 as libc::c_int {
        *(*__ctype_toupper_loc()).offset(__c as isize)
    } else {
        __c
    };
}
#[no_mangle]
pub unsafe extern "C" fn command_close(
    mut command: *mut command_t,
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    if session.is_null() || ((*session).server).is_null() {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            97 as libc::c_int,
            b"Tsunami session was not active\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    fclose((*session).server);
    (*session).server = 0 as *mut FILE;
    if (*(*session).parameter).verbose_yn != 0 {
        printf(b"Connection closed.\n\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn command_connect(
    mut command: *mut command_t,
    mut parameter: *mut ttp_parameter_t,
) -> *mut ttp_session_t {
    let mut server_fd: libc::c_int = 0;
    let mut session: *mut ttp_session_t = 0 as *mut ttp_session_t;
    let mut secret: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*command).count as libc::c_int > 1 as libc::c_int {
        if !((*parameter).server_name).is_null() {
            free((*parameter).server_name as *mut libc::c_void);
        }
        (*parameter).server_name = strdup((*command).text[1 as libc::c_int as usize]);
        if ((*parameter).server_name).is_null() {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                132 as libc::c_int,
                b"Could not update server name\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            return 0 as *mut ttp_session_t;
        }
    }
    if (*command).count as libc::c_int > 2 as libc::c_int {
        (*parameter).server_port = atoi((*command).text[2 as libc::c_int as usize]) as u_int16_t;
    }
    session = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ttp_session_t>() as libc::c_ulong,
    ) as *mut ttp_session_t;
    if session.is_null() {
        error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            144 as libc::c_int,
            b"Could not allocate session object\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*session).parameter = parameter;
    server_fd =
        create_tcp_socket_client(session, (*parameter).server_name, (*parameter).server_port);
    if server_fd < 0 as libc::c_int {
        sprintf(
            g_error.as_mut_ptr(),
            b"Could not connect to %s:%d.\0" as *const u8 as *const libc::c_char,
            (*parameter).server_name,
            (*parameter).server_port as libc::c_int,
        );
        error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            151 as libc::c_int,
            g_error.as_mut_ptr(),
            0 as libc::c_int,
        );
        return 0 as *mut ttp_session_t;
    }
    (*session).server = fdopen(server_fd, b"w+\0" as *const u8 as *const libc::c_char);
    if ((*session).server).is_null() {
        error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            b"Could not convert control channel into a stream\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
        close(server_fd);
        free(session as *mut libc::c_void);
        return 0 as *mut ttp_session_t;
    }
    if ttp_negotiate_client(session) < 0 as libc::c_int {
        error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            166 as libc::c_int,
            b"Protocol negotiation failed\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fclose((*session).server);
        free(session as *mut libc::c_void);
        return 0 as *mut ttp_session_t;
    }
    if ((*parameter).passphrase).is_null() {
        secret = strdup(b"kitten\0" as *const u8 as *const libc::c_char);
    } else {
        secret = strdup((*parameter).passphrase);
    }
    if ttp_authenticate_client(session, secret as *mut u_char) < 0 as libc::c_int {
        error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            182 as libc::c_int,
            b"Authentication failed\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        fclose((*session).server);
        free(secret as *mut libc::c_void);
        free(session as *mut libc::c_void);
        return 0 as *mut ttp_session_t;
    }
    if (*(*session).parameter).verbose_yn != 0 {
        printf(b"Connected.\n\n\0" as *const u8 as *const libc::c_char);
    }
    free(secret as *mut libc::c_void);
    return session;
}
#[no_mangle]
pub unsafe extern "C" fn command_dir(
    mut command: *mut command_t,
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    let mut result: u_char = 0;
    let mut read_str: [libc::c_char; 2048] = [0; 2048];
    let mut num_files: u_int16_t = 0;
    let mut i: u_int16_t = 0;
    let mut filelen: size_t = 0;
    let mut status: u_int16_t = 0 as libc::c_int as u_int16_t;
    if session.is_null() || ((*session).server).is_null() {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            215 as libc::c_int,
            b"Not connected to a Tsunami server\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    fprintf(
        (*session).server,
        b"%s\n\0" as *const u8 as *const libc::c_char,
        b"!#DIR??\0" as *const u8 as *const libc::c_char,
    );
    status = fread(
        &mut result as *mut u_char as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    ) as u_int16_t;
    if (status as libc::c_int) < 1 as libc::c_int {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            222 as libc::c_int,
            b"Could not read response to directory request\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if result as libc::c_int == 8 as libc::c_int {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            224 as libc::c_int,
            b"Server does no support listing of shared files\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    read_str[0 as libc::c_int as usize] = result as libc::c_char;
    fread_line(
        (*session).server,
        &mut *read_str.as_mut_ptr().offset(1 as libc::c_int as isize),
        (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
            .wrapping_sub(2 as libc::c_int as libc::c_ulong),
    );
    num_files = atoi(read_str.as_mut_ptr()) as u_int16_t;
    fprintf(
        stderr,
        b"Remote file list:\n\0" as *const u8 as *const libc::c_char,
    );
    i = 0 as libc::c_int as u_int16_t;
    while (i as libc::c_int) < num_files as libc::c_int {
        fread_line(
            (*session).server,
            read_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        fprintf(
            stderr,
            b" %2d) %-64s\0" as *const u8 as *const libc::c_char,
            i as libc::c_int + 1 as libc::c_int,
            read_str.as_mut_ptr(),
        );
        fread_line(
            (*session).server,
            read_str.as_mut_ptr(),
            (::core::mem::size_of::<[libc::c_char; 2048]>() as libc::c_ulong)
                .wrapping_sub(1 as libc::c_int as libc::c_ulong),
        );
        filelen = atol(read_str.as_mut_ptr()) as size_t;
        fprintf(
            stderr,
            b"%8Lu bytes\n\0" as *const u8 as *const libc::c_char,
            filelen as ull_t,
        );
        i = i.wrapping_add(1);
        i;
    }
    fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
    fwrite(
        b"\0\0" as *const u8 as *const libc::c_char as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
        (*session).server,
    );
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn command_get(
    mut command: *mut command_t,
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    let mut current_block: u64;
    let mut datagram: *mut u_char = 0 as *mut u_char;
    let mut local_datagram: *mut u_char = 0 as *mut u_char;
    let mut this_block: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut this_type: u_int16_t = 0 as libc::c_int as u_int16_t;
    let mut delta: u_int64_t = 0 as libc::c_int as u_int64_t;
    let mut block: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut dumpcount: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut mbit_thru: libc::c_double = 0.;
    let mut mbit_good: libc::c_double = 0.;
    let mut mbit_file: libc::c_double = 0.;
    let mut time_secs: libc::c_double = 0.;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut rexmit: *mut retransmit_t = &mut (*session).transfer.retransmit;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut disk_thread_id: pthread_t = 0 as libc::c_int as pthread_t;
    let mut multimode: libc::c_int = 0 as libc::c_int;
    let mut file_names: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    let mut f_counter: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut f_total: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut f_arrsize: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut ping_s: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut ping_e: timeval = timeval {
        tv_sec: 0,
        tv_usec: 0,
    };
    let mut wait_u_sec: libc::c_long = 1 as libc::c_int as libc::c_long;
    if ((*command).count as libc::c_int) < 2 as libc::c_int {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            284 as libc::c_int,
            b"Invalid command syntax (use 'help get' for details)\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    if session.is_null() || ((*session).server).is_null() {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            288 as libc::c_int,
            b"Not connected to a Tsunami server\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    memset(
        xfer as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_transfer_t>() as libc::c_ulong,
    );
    if strcmp(
        b"*\0" as *const u8 as *const libc::c_char,
        (*command).text[1 as libc::c_int as usize],
    ) == 0
    {
        let mut filearray_size: [libc::c_char; 10] = [0; 10];
        let mut file_count: [libc::c_char; 10] = [0; 10];
        multimode = 1 as libc::c_int;
        printf(b"Requesting all available files\n\0" as *const u8 as *const libc::c_char);
        gettimeofday(&mut ping_s, 0 as *mut libc::c_void);
        status = fprintf(
            (*session).server,
            b"%s\n\0" as *const u8 as *const libc::c_char,
            (*command).text[1 as libc::c_int as usize],
        );
        status = fread(
            filearray_size.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            10 as libc::c_int as libc::c_ulong,
            (*session).server,
        ) as libc::c_int;
        gettimeofday(&mut ping_e, 0 as *mut libc::c_void);
        status = fread(
            file_count.as_mut_ptr() as *mut libc::c_void,
            ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
            10 as libc::c_int as libc::c_ulong,
            (*session).server,
        ) as libc::c_int;
        fprintf(
            (*session).server,
            b"got size\0" as *const u8 as *const libc::c_char,
        );
        if status <= 0 as libc::c_int || fflush((*session).server) != 0 {
            return error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                311 as libc::c_int,
                b"Could not request file\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        if status < 1 as libc::c_int {
            return error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                315 as libc::c_int,
                b"Could not read response to file request\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        wait_u_sec = (ping_e.tv_sec - ping_s.tv_sec) * 1000000 as libc::c_int as __time_t
            + (ping_e.tv_usec - ping_s.tv_usec);
        wait_u_sec =
            wait_u_sec + (wait_u_sec as libc::c_double * 0.1f64) as libc::c_int as libc::c_long;
        sscanf(
            filearray_size.as_mut_ptr(),
            b"%u\0" as *const u8 as *const libc::c_char,
            &mut f_arrsize as *mut u_int32_t,
        );
        sscanf(
            file_count.as_mut_ptr(),
            b"%u\0" as *const u8 as *const libc::c_char,
            &mut f_total as *mut u_int32_t,
        );
        if f_total <= 0 as libc::c_int as u_int32_t {
            let mut dummy: [libc::c_char; 1] = [0; 1];
            status = fread(
                dummy.as_mut_ptr() as *mut libc::c_void,
                ::core::mem::size_of::<libc::c_char>() as libc::c_ulong,
                1 as libc::c_int as libc::c_ulong,
                (*session).server,
            ) as libc::c_int;
            return error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                330 as libc::c_int,
                b"Server advertised no files to get\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            printf(
                b"\nServer is sharing %u files\n\0" as *const u8 as *const libc::c_char,
                f_total,
            );
            file_names = malloc(
                (f_total as libc::c_ulong)
                    .wrapping_mul(::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong),
            ) as *mut *mut libc::c_char;
            if file_names.is_null() {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    339 as libc::c_int,
                    b"Could not allocate memory\n\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            printf(
                b"Multi-GET of %d files:\n\0" as *const u8 as *const libc::c_char,
                f_total,
            );
            f_counter = 0 as libc::c_int as u_int32_t;
            while f_counter < f_total {
                let mut tmpname: [libc::c_char; 1024] = [0; 1024];
                fread_line(
                    (*session).server,
                    tmpname.as_mut_ptr(),
                    1024 as libc::c_int as size_t,
                );
                let ref mut fresh0 = *file_names.offset(f_counter as isize);
                *fresh0 = strdup(tmpname.as_mut_ptr());
                printf(
                    b"%s \0" as *const u8 as *const libc::c_char,
                    *file_names.offset(f_counter as isize),
                );
                f_counter = f_counter.wrapping_add(1);
                f_counter;
            }
            fprintf(
                (*session).server,
                b"got list\0" as *const u8 as *const libc::c_char,
            );
            printf(b"\n\0" as *const u8 as *const libc::c_char);
        }
    } else {
        f_total = 1 as libc::c_int as u_int32_t;
    }
    f_counter = 0 as libc::c_int as u_int32_t;
    's_202: loop {
        if multimode == 0 {
            (*xfer).remote_filename = (*command).text[1 as libc::c_int as usize];
        } else {
            (*xfer).remote_filename = *file_names.offset(f_counter as isize);
        }
        if multimode == 0 {
            if (*command).count as libc::c_int >= 3 as libc::c_int {
                (*xfer).local_filename = (*command).text[2 as libc::c_int as usize];
            } else {
                (*xfer).local_filename =
                    strrchr((*command).text[1 as libc::c_int as usize], '/' as i32);
                if ((*xfer).local_filename).is_null() {
                    (*xfer).local_filename = (*command).text[1 as libc::c_int as usize];
                } else {
                    (*xfer).local_filename = ((*xfer).local_filename).offset(1);
                    (*xfer).local_filename;
                }
            }
        } else {
            (*xfer).local_filename = *file_names.offset(f_counter as isize);
            printf(
                b"GET *: now requesting file '%s'\n\0" as *const u8 as *const libc::c_char,
                (*xfer).local_filename,
            );
        }
        if ttp_open_transfer_client(session, (*xfer).remote_filename, (*xfer).local_filename)
            < 0 as libc::c_int
        {
            return error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                387 as libc::c_int,
                b"File transfer request failed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        if ttp_open_port_client(session) < 0 as libc::c_int {
            return error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                391 as libc::c_int,
                b"Creation of data socket failed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        }
        (*rexmit).table = calloc(
            super::config::DEFAULT_TABLE_SIZE as libc::c_ulong,
            ::core::mem::size_of::<u_int32_t>() as libc::c_ulong,
        ) as *mut u_int32_t;
        if ((*rexmit).table).is_null() {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                396 as libc::c_int,
                b"Could not allocate retransmission table\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        (*xfer).received = calloc(
            ((*xfer).block_count / 8 as libc::c_int as u_int32_t)
                .wrapping_add(2 as libc::c_int as u_int32_t) as libc::c_ulong,
            ::core::mem::size_of::<u_char>() as libc::c_ulong,
        ) as *mut u_char;
        if ((*xfer).received).is_null() {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                401 as libc::c_int,
                b"Could not allocate received-data bitfield\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        (*xfer).ring_buffer = ring_create(session);
        local_datagram = calloc(
            (6 as libc::c_int as u_int32_t).wrapping_add((*(*session).parameter).block_size)
                as libc::c_ulong,
            ::core::mem::size_of::<u_char>() as libc::c_ulong,
        ) as *mut u_char;
        if local_datagram.is_null() {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                409 as libc::c_int,
                b"Could not allocate fast local datagram buffer in command_get()\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        status = pthread_create(
            &mut disk_thread_id,
            0 as *const pthread_attr_t,
            Some(disk_thread as unsafe extern "C" fn(*mut libc::c_void) -> *mut libc::c_void),
            session as *mut libc::c_void,
        );
        if status != 0 as libc::c_int {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                414 as libc::c_int,
                b"Could not create I/O thread\0" as *const u8 as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        (*rexmit).table_size = super::config::DEFAULT_TABLE_SIZE as u_int32_t;
        (*rexmit).index_max = 0 as libc::c_int as u_int32_t;
        (*xfer).next_block = 1 as libc::c_int as u_int32_t;
        (*xfer).gapless_to_block = 0 as libc::c_int as u_int32_t;
        memset(
            &mut (*xfer).stats as *mut statistics_t as *mut libc::c_void,
            0 as libc::c_int,
            ::core::mem::size_of::<statistics_t>() as libc::c_ulong,
        );
        (*xfer).stats.start_udp_errors = get_udp_in_errors();
        (*xfer).stats.this_udp_errors = (*xfer).stats.start_udp_errors;
        gettimeofday(&mut (*xfer).stats.start_time, 0 as *mut libc::c_void);
        gettimeofday(&mut (*xfer).stats.this_time, 0 as *mut libc::c_void);
        if (*(*session).parameter).transcript_yn != 0 {
            xscript_data_start_client(session, &mut (*xfer).stats.start_time);
        }
        loop {
            status = recvfrom(
                (*xfer).udp_fd,
                local_datagram as *mut libc::c_void,
                (6 as libc::c_int as u_int32_t).wrapping_add((*(*session).parameter).block_size)
                    as size_t,
                0 as libc::c_int,
                __SOCKADDR_ARG {
                    __sockaddr__: 0 as *mut libc::c_void as *mut sockaddr,
                },
                0 as *mut socklen_t,
            ) as libc::c_int;
            if status < 0 as libc::c_int {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    442 as libc::c_int,
                    b"UDP data transmission error\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                printf(
                    b"Apparently frozen transfer, trying to do retransmit request\n\0" as *const u8
                        as *const libc::c_char,
                );
                if ttp_repeat_retransmit(session) < 0 as libc::c_int {
                    error_handler(
                        b"command.c\0" as *const u8 as *const libc::c_char,
                        445 as libc::c_int,
                        b"Repeat of retransmission requests failed\0" as *const u8
                            as *const libc::c_char,
                        0 as libc::c_int,
                    );
                    current_block = 78252603380123710;
                    break 's_202;
                }
            }
            this_block = __bswap_32(*(local_datagram as *mut u_int32_t));
            this_type =
                __bswap_16(*(local_datagram.offset(4 as libc::c_int as isize) as *mut u_int16_t));
            (*xfer).stats.total_blocks = ((*xfer).stats.total_blocks).wrapping_add(1);
            (*xfer).stats.total_blocks;
            if this_type as libc::c_int != 'R' as i32 {
                (*xfer).stats.this_flow_originals =
                    ((*xfer).stats.this_flow_originals).wrapping_add(1);
                (*xfer).stats.this_flow_originals;
            } else {
                (*xfer).stats.this_flow_retransmitteds =
                    ((*xfer).stats.this_flow_retransmitteds).wrapping_add(1);
                (*xfer).stats.this_flow_retransmitteds;
                (*xfer).stats.total_recvd_retransmits =
                    ((*xfer).stats.total_recvd_retransmits).wrapping_add(1);
                (*xfer).stats.total_recvd_retransmits;
            }
            if ring_full((*xfer).ring_buffer) == 0 {
                if got_block(session, this_block) == 0
                    || this_type as libc::c_int == 'X' as i32
                    || (*xfer).restart_pending as libc::c_int != 0
                {
                    if got_block(session, this_block) == 0 {
                        datagram = ring_reserve((*xfer).ring_buffer);
                        memcpy(
                            datagram as *mut libc::c_void,
                            local_datagram as *const libc::c_void,
                            (6 as libc::c_int as u_int32_t)
                                .wrapping_add((*(*session).parameter).block_size)
                                as libc::c_ulong,
                        );
                        if ring_confirm((*xfer).ring_buffer) < 0 as libc::c_int {
                            error_handler(
                                b"command.c\0" as *const u8 as *const libc::c_char,
                                475 as libc::c_int,
                                b"Error in accepting block\0" as *const u8 as *const libc::c_char,
                                0 as libc::c_int,
                            );
                            current_block = 78252603380123710;
                            break 's_202;
                        } else {
                            let ref mut fresh1 = *((*xfer).received)
                                .offset((this_block / 8 as libc::c_int as u_int32_t) as isize);
                            *fresh1 = (*fresh1 as libc::c_int
                                | (1 as libc::c_int) << this_block % 8 as libc::c_int as u_int32_t)
                                as u_char;
                            if (*xfer).blocks_left > 0 as libc::c_int as u_int32_t {
                                (*xfer).blocks_left = ((*xfer).blocks_left).wrapping_sub(1);
                                (*xfer).blocks_left;
                            } else {
                                printf(
                                    b"Oops! Negative-going blocks_left count at block: type=%c this=%u final=%u left=%u\n\0"
                                        as *const u8 as *const libc::c_char,
                                    this_type as libc::c_int,
                                    this_block,
                                    (*xfer).block_count,
                                    (*xfer).blocks_left,
                                );
                            }
                        }
                    }
                    if (*xfer).restart_pending as libc::c_int != 0
                        && this_type as libc::c_int != 'X' as i32
                    {
                        if this_block > (*xfer).restart_lastidx
                            && this_block <= (*xfer).restart_wireclearidx
                        {
                            current_block = 13361531435213260772;
                        } else {
                            current_block = 8937240710477387595;
                        }
                    } else {
                        current_block = 8937240710477387595;
                    }
                    match current_block {
                        13361531435213260772 => {}
                        _ => {
                            if this_block > (*xfer).next_block {
                                if (*(*session).parameter).lossless == 0 {
                                    if (*(*session).parameter).losswindow_ms
                                        == 0 as libc::c_int as u_int32_t
                                    {
                                        (*xfer).gapless_to_block = this_block;
                                    } else {
                                        let mut path_capability: libc::c_double = 0.;
                                        path_capability = 0.8f64
                                            * ((*xfer).stats.this_transmit_rate
                                                + (*xfer).stats.this_retransmit_rate);
                                        path_capability *= 0.001f64
                                            * (*(*session).parameter).losswindow_ms
                                                as libc::c_double;
                                        let mut earliest_block: u_int32_t = (this_block
                                            as libc::c_double
                                            - (if ((1024 as libc::c_int * 1024 as libc::c_int)
                                                as libc::c_double
                                                * path_capability
                                                / (8 as libc::c_int as u_int32_t
                                                    * (*(*session).parameter).block_size)
                                                    as libc::c_double)
                                                < this_block.wrapping_sub((*xfer).gapless_to_block)
                                                    as libc::c_double
                                            {
                                                (1024 as libc::c_int * 1024 as libc::c_int)
                                                    as libc::c_double
                                                    * path_capability
                                                    / (8 as libc::c_int as u_int32_t
                                                        * (*(*session).parameter).block_size)
                                                        as libc::c_double
                                            } else {
                                                this_block.wrapping_sub((*xfer).gapless_to_block)
                                                    as libc::c_double
                                            }))
                                            as u_int32_t;
                                        block = earliest_block;
                                        while block < this_block {
                                            if ttp_request_retransmit(session, block)
                                                < 0 as libc::c_int
                                            {
                                                error_handler(
                                                    b"command.c\0" as *const u8
                                                        as *const libc::c_char,
                                                    515 as libc::c_int,
                                                    b"Retransmission request failed\0" as *const u8
                                                        as *const libc::c_char,
                                                    0 as libc::c_int,
                                                );
                                                current_block = 78252603380123710;
                                                break 's_202;
                                            } else {
                                                block = block.wrapping_add(1);
                                                block;
                                            }
                                        }
                                        (*xfer).next_block = earliest_block;
                                        (*xfer).gapless_to_block = earliest_block;
                                    }
                                } else {
                                    block = (*xfer).next_block;
                                    while block < this_block {
                                        if ttp_request_retransmit(session, block) < 0 as libc::c_int
                                        {
                                            error_handler(
                                                b"command.c\0" as *const u8 as *const libc::c_char,
                                                528 as libc::c_int,
                                                b"Retransmission request failed\0" as *const u8
                                                    as *const libc::c_char,
                                                0 as libc::c_int,
                                            );
                                            current_block = 78252603380123710;
                                            break 's_202;
                                        } else {
                                            block = block.wrapping_add(1);
                                            block;
                                        }
                                    }
                                }
                            }
                            while got_block(
                                session,
                                ((*xfer).gapless_to_block)
                                    .wrapping_add(1 as libc::c_int as u_int32_t),
                            ) != 0
                                && (*xfer).gapless_to_block < (*xfer).block_count
                            {
                                (*xfer).gapless_to_block =
                                    ((*xfer).gapless_to_block).wrapping_add(1);
                                (*xfer).gapless_to_block;
                            }
                            if this_type as libc::c_int == 'O' as i32 {
                                (*xfer).next_block =
                                    this_block.wrapping_add(1 as libc::c_int as u_int32_t);
                            }
                            if (*xfer).restart_pending as libc::c_int != 0
                                && (*xfer).next_block >= (*xfer).restart_lastidx
                            {
                                (*xfer).restart_pending = 0 as libc::c_int as u_char;
                            }
                            if this_type as libc::c_int == 'X' as i32 {
                                if (*xfer).blocks_left == 0 as libc::c_int as u_int32_t {
                                    break;
                                }
                                if (*(*session).parameter).lossless == 0 {
                                    if (*rexmit).index_max == 0 as libc::c_int as u_int32_t
                                        && (*xfer).restart_pending == 0
                                    {
                                        break;
                                    }
                                }
                                block = ((*xfer).gapless_to_block)
                                    .wrapping_add(1 as libc::c_int as u_int32_t);
                                while block < (*xfer).block_count {
                                    if ttp_request_retransmit(session, block) < 0 as libc::c_int {
                                        error_handler(
                                            b"command.c\0" as *const u8 as *const libc::c_char,
                                            571 as libc::c_int,
                                            b"Retransmission request failed\0" as *const u8
                                                as *const libc::c_char,
                                            0 as libc::c_int,
                                        );
                                        current_block = 78252603380123710;
                                        break 's_202;
                                    } else {
                                        block = block.wrapping_add(1);
                                        block;
                                    }
                                }
                                ttp_repeat_retransmit(session);
                            }
                        }
                    }
                }
            }
            if !((*xfer).stats.total_blocks % 50 as libc::c_int as u_int32_t == 0) {
                continue;
            }
            if !(get_usec_since(&mut (*xfer).stats.this_time) as libc::c_ulonglong
                > 350000 as libc::c_longlong as libc::c_ulonglong)
            {
                continue;
            }
            if ttp_repeat_retransmit(session) < 0 as libc::c_int {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    592 as libc::c_int,
                    b"Repeat of retransmission requests failed\0" as *const u8
                        as *const libc::c_char,
                    0 as libc::c_int,
                );
                current_block = 78252603380123710;
                break 's_202;
            } else {
                ttp_update_stats(session);
                if (*(*session).parameter).blockdump != 0 {
                    let mut postfix: [libc::c_char; 64] = [0; 64];
                    let fresh2 = dumpcount;
                    dumpcount = dumpcount.wrapping_add(1);
                    snprintf(
                        postfix.as_mut_ptr(),
                        63 as libc::c_int as libc::c_ulong,
                        b".bmap%u\0" as *const u8 as *const libc::c_char,
                        fresh2,
                    );
                    dump_blockmap(postfix.as_mut_ptr(), xfer);
                }
            }
        }
        printf(
            b"Transfer complete. Flushing to disk and signaling server to stop...\n\0" as *const u8
                as *const libc::c_char,
        );
        close((*xfer).udp_fd);
        if ttp_request_stop(session) < 0 as libc::c_int {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                619 as libc::c_int,
                b"Could not request end of transfer\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            current_block = 78252603380123710;
            break;
        } else {
            datagram = ring_reserve((*xfer).ring_buffer);
            *(datagram as *mut u_int32_t) = 0 as libc::c_int as u_int32_t;
            if ring_confirm((*xfer).ring_buffer) < 0 as libc::c_int {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    627 as libc::c_int,
                    b"Error in terminating disk thread\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            if pthread_join(disk_thread_id, 0 as *mut *mut libc::c_void) < 0 as libc::c_int {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    631 as libc::c_int,
                    b"Disk thread terminated with error\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
            }
            gettimeofday(&mut (*xfer).stats.stop_time, 0 as *mut libc::c_void);
            delta = get_usec_since(&mut (*xfer).stats.start_time);
            (*xfer).stats.total_lost = 0 as libc::c_int as u_int32_t;
            block = 1 as libc::c_int as u_int32_t;
            while block <= (*xfer).block_count {
                if got_block(session, block) == 0 {
                    (*xfer).stats.total_lost = ((*xfer).stats.total_lost).wrapping_add(1);
                    (*xfer).stats.total_lost;
                }
                block = block.wrapping_add(1);
                block;
            }
            mbit_thru = 8.0f64
                * (*xfer).stats.total_blocks as libc::c_double
                * (*(*session).parameter).block_size as libc::c_double;
            mbit_good = mbit_thru
                - 8.0f64
                    * (*xfer).stats.total_recvd_retransmits as libc::c_double
                    * (*(*session).parameter).block_size as libc::c_double;
            mbit_file = 8.0f64 * (*xfer).file_size as libc::c_double;
            mbit_thru /= 1024.0f64 * 1024.0f64;
            mbit_good /= 1024.0f64 * 1024.0f64;
            mbit_file /= 1024.0f64 * 1024.0f64;
            time_secs = delta as libc::c_double / 1e6f64;
            printf(
                b"PC performance figure : %llu packets dropped (if high this indicates receiving PC overload)\n\0"
                    as *const u8 as *const libc::c_char,
                ((*xfer).stats.this_udp_errors)
                    .wrapping_sub((*xfer).stats.start_udp_errors) as ull_t,
            );
            printf(
                b"Transfer duration     : %0.2f seconds\n\0" as *const u8 as *const libc::c_char,
                time_secs,
            );
            printf(
                b"Total packet data     : %0.2f Mbit\n\0" as *const u8 as *const libc::c_char,
                mbit_thru,
            );
            printf(
                b"Goodput data          : %0.2f Mbit\n\0" as *const u8 as *const libc::c_char,
                mbit_good,
            );
            printf(
                b"File data             : %0.2f Mbit\n\0" as *const u8 as *const libc::c_char,
                mbit_file,
            );
            printf(
                b"Throughput            : %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                mbit_thru / time_secs,
            );
            printf(
                b"Goodput w/ restarts   : %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                mbit_good / time_secs,
            );
            printf(
                b"Final file rate       : %0.2f Mbps\n\0" as *const u8 as *const libc::c_char,
                mbit_file / time_secs,
            );
            printf(b"Transfer mode         : \0" as *const u8 as *const libc::c_char);
            if (*(*session).parameter).lossless != 0 {
                if (*xfer).stats.total_lost == 0 as libc::c_int as u_int32_t {
                    printf(b"lossless\n\0" as *const u8 as *const libc::c_char);
                } else {
                    printf(
                        b"lossless mode - but lost count=%u > 0, please file a bug report!!\n\0"
                            as *const u8 as *const libc::c_char,
                        (*xfer).stats.total_lost,
                    );
                }
            } else {
                if (*(*session).parameter).losswindow_ms == 0 as libc::c_int as u_int32_t {
                    printf(b"lossy\n\0" as *const u8 as *const libc::c_char);
                } else {
                    printf(
                        b"semi-lossy, time window %d ms\n\0" as *const u8 as *const libc::c_char,
                        (*(*session).parameter).losswindow_ms,
                    );
                }
                printf(
                    b"Data blocks lost      : %llu (%.2f%% of data) per user-specified time window constraint\n\0"
                        as *const u8 as *const libc::c_char,
                    (*xfer).stats.total_lost as ull_t,
                    100.0f64 * (*xfer).stats.total_lost as libc::c_double
                        / (*xfer).block_count as libc::c_double,
                );
            }
            printf(b"\n\0" as *const u8 as *const libc::c_char);
            if (*(*session).parameter).transcript_yn != 0 {
                xscript_data_stop_client(session, &mut (*xfer).stats.stop_time);
                xscript_close_client(session, delta);
            }
            if (*(*session).parameter).blockdump != 0 {
                dump_blockmap(b".blockmap\0" as *const u8 as *const libc::c_char, xfer);
            }
            if !((*xfer).file).is_null() {
                fclose((*xfer).file);
                (*xfer).file = 0 as *mut FILE;
            }
            ring_destroy((*xfer).ring_buffer);
            if !((*rexmit).table).is_null() {
                free((*rexmit).table as *mut libc::c_void);
                (*rexmit).table = 0 as *mut u_int32_t;
            }
            if !((*xfer).received).is_null() {
                free((*xfer).received as *mut libc::c_void);
                (*xfer).received = 0 as *mut u_char;
            }
            if !local_datagram.is_null() {
                free(local_datagram as *mut libc::c_void);
                local_datagram = 0 as *mut u_char;
            }
            if (*(*session).parameter).rate_adjust != 0 {
                (*(*session).parameter).target_rate =
                    (1.15f64 * 1e6f64 * (mbit_file / time_secs)) as u_int32_t;
                printf(
                    b"Adjusting target rate to %d Mbps for next transfer.\n\0" as *const u8
                        as *const libc::c_char,
                    ((*(*session).parameter).target_rate as libc::c_double / 1e6f64) as libc::c_int,
                );
            }
            f_counter = f_counter.wrapping_add(1);
            if !(f_counter < f_total) {
                current_block = 6000599718051633247;
                break;
            }
        }
    }
    match current_block {
        78252603380123710 => {
            fprintf(
                stderr,
                b"Transfer not successful.  (WARNING: You may need to reconnect.)\n\n\0"
                    as *const u8 as *const libc::c_char,
            );
            close((*xfer).udp_fd);
            ring_destroy((*xfer).ring_buffer);
            if !((*xfer).file).is_null() {
                fclose((*xfer).file);
                (*xfer).file = 0 as *mut FILE;
            }
            if !((*rexmit).table).is_null() {
                free((*rexmit).table as *mut libc::c_void);
                (*rexmit).table = 0 as *mut u_int32_t;
            }
            if !((*xfer).received).is_null() {
                free((*xfer).received as *mut libc::c_void);
                (*xfer).received = 0 as *mut u_char;
            }
            if !local_datagram.is_null() {
                free(local_datagram as *mut libc::c_void);
                local_datagram = 0 as *mut u_char;
            }
            return -(1 as libc::c_int);
        }
        _ => {
            if multimode != 0 {
                f_counter = 0 as libc::c_int as u_int32_t;
                while f_counter < f_total {
                    free(*file_names.offset(f_counter as isize) as *mut libc::c_void);
                    f_counter = f_counter.wrapping_add(1);
                    f_counter;
                }
                free(file_names as *mut libc::c_void);
            }
            return 0 as libc::c_int;
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn command_help(
    mut command: *mut command_t,
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    if ((*command).count as libc::c_int) < 2 as libc::c_int {
        printf(
            b"Help is available for the following commands:\n\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"    close    connect    get    dir    help    quit    set\n\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"Use 'help <command>' for help on an individual command.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"close\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(b"Usage: close\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Closes the current connection to a remote Tsunami server.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"connect\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(b"Usage: connect\n\0" as *const u8 as *const libc::c_char);
        printf(b"       connect <remote-host>\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"       connect <remote-host> <remote-port>\n\n\0" as *const u8 as *const libc::c_char,
        );
        printf(
            b"Opens a connection to a remote Tsunami server.  If the host and port\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"are not specified, default values are used.  (Use the 'set' command to\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(b"modify these values.)\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"After connecting, you will be prompted to enter a shared secret for\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(b"authentication.\n\n\0" as *const u8 as *const libc::c_char);
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"get\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(b"Usage: get <remote-file>\n\0" as *const u8 as *const libc::c_char);
        printf(b"       get <remote-file> <local-file>\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Attempts to retrieve the remote file with the given name using the\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"Tsunami file transfer protocol.  If the local filename is not\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"specified, the final part of the remote filename (after the last path\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(b"separator) will be used.\n\n\0" as *const u8 as *const libc::c_char);
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"dir\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(b"Usage: dir\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Attempts to list the available remote files.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"help\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(
            b"Come on.  You know what that command does.\n\n\0" as *const u8 as *const libc::c_char,
        );
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"quit\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(b"Usage: quit\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Closes any open connection to a remote Tsunami server and exits the\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(b"Tsunami client.\n\n\0" as *const u8 as *const libc::c_char);
    } else if strcasecmp(
        (*command).text[1 as libc::c_int as usize],
        b"set\0" as *const u8 as *const libc::c_char,
    ) == 0
    {
        printf(b"Usage: set\n\0" as *const u8 as *const libc::c_char);
        printf(b"       set <field>\n\0" as *const u8 as *const libc::c_char);
        printf(b"       set <field> <value>\n\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"Sets one of the defaults to the given value.  If the value is omitted,\n\0"
                as *const u8 as *const libc::c_char,
        );
        printf(
            b"the current value of the field is returned.  If the field is also\n\0" as *const u8
                as *const libc::c_char,
        );
        printf(
            b"omitted, the current values of all defaults are returned.\n\n\0" as *const u8
                as *const libc::c_char,
        );
    } else {
        printf(
            b"'%s' is not a recognized command.\n\0" as *const u8 as *const libc::c_char,
            (*command).text[1 as libc::c_int as usize],
        );
        printf(b"Use 'help' for a list of commands.\n\n\0" as *const u8 as *const libc::c_char);
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn command_quit(
    mut command: *mut command_t,
    mut session: *mut ttp_session_t,
) -> libc::c_int {
    if !session.is_null() && !((*session).server).is_null() {
        fclose((*session).server);
    }
    printf(b"Thank you for using Tsunami.\n\0" as *const u8 as *const libc::c_char);
    printf(
        b"The ANML web site can be found at:    http://www.anml.iu.edu/\n\0" as *const u8
            as *const libc::c_char,
    );
    printf(
        b"The SourceForge site can be found at: http://tsunami-udp.sf.net/\n\n\0" as *const u8
            as *const libc::c_char,
    );
    exit(1 as libc::c_int);
}
#[no_mangle]
pub unsafe extern "C" fn command_set(
    mut command: *mut command_t,
    mut parameter: *mut ttp_parameter_t,
) -> libc::c_int {
    let mut do_all: libc::c_int =
        ((*command).count as libc::c_int == 1 as libc::c_int) as libc::c_int;
    if (*command).count as libc::c_int == 3 as libc::c_int {
        if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"server\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !((*parameter).server_name).is_null() {
                free((*parameter).server_name as *mut libc::c_void);
            }
            (*parameter).server_name = strdup((*command).text[2 as libc::c_int as usize]);
            if ((*parameter).server_name).is_null() {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    851 as libc::c_int,
                    b"Could not update server name\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"port\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).server_port =
                atoi((*command).text[2 as libc::c_int as usize]) as u_int16_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"udpport\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).client_port =
                atoi((*command).text[2 as libc::c_int as usize]) as u_int16_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"buffer\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).udp_buffer = atol((*command).text[2 as libc::c_int as usize]) as u_int32_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"blocksize\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).block_size = atol((*command).text[2 as libc::c_int as usize]) as u_int32_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"verbose\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).verbose_yn = (strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"transcript\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).transcript_yn = (strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int
                as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"ip\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).ipv6_yn = (strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"v6\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"output\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).output_mode = (if strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"screen\0" as *const u8 as *const libc::c_char,
            ) != 0
            {
                1 as libc::c_int
            } else {
                0 as libc::c_int
            }) as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"rateadjust\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).rate_adjust = (strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"rate\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            let mut multiplier: libc::c_long = 1 as libc::c_int as libc::c_long;
            let mut cmd: *mut libc::c_char =
                (*command).text[2 as libc::c_int as usize] as *mut libc::c_char;
            let mut cpy: [libc::c_char; 256] = [0; 256];
            let mut l: libc::c_int = strlen(cmd) as libc::c_int;
            strcpy(cpy.as_mut_ptr(), cmd);
            if l > 1 as libc::c_int
                && ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int =
                                cpy[(l - 1 as libc::c_int) as usize] as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int as isize);
                    }
                    __res
                }) == 'M' as i32
            {
                multiplier = 1000000 as libc::c_int as libc::c_long;
                cpy[(l - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            } else if l > 1 as libc::c_int
                && ({
                    let mut __res: libc::c_int = 0;
                    if ::core::mem::size_of::<libc::c_char>() as libc::c_ulong
                        > 1 as libc::c_int as libc::c_ulong
                    {
                        if 0 != 0 {
                            let mut __c: libc::c_int =
                                cpy[(l - 1 as libc::c_int) as usize] as libc::c_int;
                            __res = (if __c < -(128 as libc::c_int) || __c > 255 as libc::c_int {
                                __c
                            } else {
                                *(*__ctype_toupper_loc()).offset(__c as isize)
                            });
                        } else {
                            __res = toupper(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int);
                        }
                    } else {
                        __res = *(*__ctype_toupper_loc())
                            .offset(cpy[(l - 1 as libc::c_int) as usize] as libc::c_int as isize);
                    }
                    __res
                }) == 'G' as i32
            {
                multiplier = 1000000000 as libc::c_int as libc::c_long;
                cpy[(l - 1 as libc::c_int) as usize] = '\0' as i32 as libc::c_char;
            }
            (*parameter).target_rate = (multiplier * atol(cpy.as_mut_ptr())) as u_int32_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"error\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).error_rate =
                (atof((*command).text[2 as libc::c_int as usize]) * 1000.0f64) as u_int32_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"slowdown\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parse_fraction(
                (*command).text[2 as libc::c_int as usize],
                &mut (*parameter).slower_num,
                &mut (*parameter).slower_den,
            );
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"speedup\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            parse_fraction(
                (*command).text[2 as libc::c_int as usize],
                &mut (*parameter).faster_num,
                &mut (*parameter).faster_den,
            );
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"history\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).history = atoi((*command).text[2 as libc::c_int as usize]) as u_int16_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"lossless\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).lossless = (strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"losswindow\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).losswindow_ms =
                atol((*command).text[2 as libc::c_int as usize]) as u_int32_t;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"blockdump\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            (*parameter).blockdump = (strcmp(
                (*command).text[2 as libc::c_int as usize],
                b"yes\0" as *const u8 as *const libc::c_char,
            ) == 0 as libc::c_int) as libc::c_int as u_char;
        } else if strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"passphrase\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            if !((*parameter).passphrase).is_null() {
                free((*parameter).passphrase as *mut libc::c_void);
            }
            (*parameter).passphrase = strdup((*command).text[2 as libc::c_int as usize]);
            if ((*parameter).passphrase).is_null() {
                error_handler(
                    b"command.c\0" as *const u8 as *const libc::c_char,
                    884 as libc::c_int,
                    b"Could not update passphrase\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
        }
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"server\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"server = %s\n\0" as *const u8 as *const libc::c_char,
            (*parameter).server_name,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"port\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"port = %u\n\0" as *const u8 as *const libc::c_char,
            (*parameter).server_port as libc::c_int,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"udpport\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"udpport = %u\n\0" as *const u8 as *const libc::c_char,
            (*parameter).client_port as libc::c_int,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"buffer\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"buffer = %u\n\0" as *const u8 as *const libc::c_char,
            (*parameter).udp_buffer,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"blocksize\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"blocksize = %u\n\0" as *const u8 as *const libc::c_char,
            (*parameter).block_size,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"verbose\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"verbose = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).verbose_yn as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"transcript\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"transcript = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).transcript_yn as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"ip\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"ip = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).ipv6_yn as libc::c_int != 0 {
                b"v6\0" as *const u8 as *const libc::c_char
            } else {
                b"v4\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"output\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"output = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).output_mode as libc::c_int == 0 as libc::c_int {
                b"screen\0" as *const u8 as *const libc::c_char
            } else {
                b"line\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"rate\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"rate = %u\n\0" as *const u8 as *const libc::c_char,
            (*parameter).target_rate,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"rateadjust\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"rateadjust = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).rate_adjust as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"error\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"error = %0.2f%%\n\0" as *const u8 as *const libc::c_char,
            (*parameter).error_rate as libc::c_double / 1000.0f64,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"slowdown\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"slowdown = %d/%d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).slower_num as libc::c_int,
            (*parameter).slower_den as libc::c_int,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"speedup\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"speedup = %d/%d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).faster_num as libc::c_int,
            (*parameter).faster_den as libc::c_int,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"history\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"history = %d%%\n\0" as *const u8 as *const libc::c_char,
            (*parameter).history as libc::c_int,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"lossless\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"lossless = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).lossless as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"losswindow\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"losswindow = %d msec\n\0" as *const u8 as *const libc::c_char,
            (*parameter).losswindow_ms,
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"blockdump\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"blockdump = %s\n\0" as *const u8 as *const libc::c_char,
            if (*parameter).blockdump as libc::c_int != 0 {
                b"yes\0" as *const u8 as *const libc::c_char
            } else {
                b"no\0" as *const u8 as *const libc::c_char
            },
        );
    }
    if do_all != 0
        || strcasecmp(
            (*command).text[1 as libc::c_int as usize],
            b"passphrase\0" as *const u8 as *const libc::c_char,
        ) == 0
    {
        printf(
            b"passphrase = %s\n\0" as *const u8 as *const libc::c_char,
            if ((*parameter).passphrase).is_null() {
                b"default\0" as *const u8 as *const libc::c_char
            } else {
                b"<user-specified>\0" as *const u8 as *const libc::c_char
            },
        );
    }
    printf(b"\n\0" as *const u8 as *const libc::c_char);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn disk_thread(mut arg: *mut libc::c_void) -> *mut libc::c_void {
    let mut session: *mut ttp_session_t = arg as *mut ttp_session_t;
    let mut datagram: *mut u_char = 0 as *mut u_char;
    let mut status: libc::c_int = 0;
    let mut block_index: u_int32_t = 0;
    let mut block_type: u_int16_t = 0;
    loop {
        datagram = ring_peek((*session).transfer.ring_buffer);
        block_index = __bswap_32(*(datagram as *mut u_int32_t));
        block_type = __bswap_16(*(datagram.offset(4 as libc::c_int as isize) as *mut u_int16_t));
        if block_index == 0 as libc::c_int as u_int32_t {
            printf(b"!!!!\n\0" as *const u8 as *const libc::c_char);
            return 0 as *mut libc::c_void;
        }
        status = accept_block(
            session,
            block_index,
            datagram.offset(6 as libc::c_int as isize),
        );
        if status < 0 as libc::c_int {
            error_handler(
                b"command.c\0" as *const u8 as *const libc::c_char,
                947 as libc::c_int,
                b"Block accept failed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            return 0 as *mut libc::c_void;
        }
        ring_pop((*session).transfer.ring_buffer);
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_fraction(
    mut fraction: *const libc::c_char,
    mut num: *mut u_int16_t,
    mut den: *mut u_int16_t,
) -> libc::c_int {
    let mut slash: *const libc::c_char = 0 as *const libc::c_char;
    slash = strchr(fraction, '/' as i32);
    if slash.is_null() {
        return error_handler(
            b"command.c\0" as *const u8 as *const libc::c_char,
            972 as libc::c_int,
            b"Value is not a fraction\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    *num = atoi(fraction) as u_int16_t;
    *den = atoi(slash.offset(1 as libc::c_int as isize)) as u_int16_t;
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn got_block(
    mut session: *mut ttp_session_t,
    mut blocknr: u_int32_t,
) -> libc::c_int {
    if blocknr > (*session).transfer.block_count {
        return 1 as libc::c_int;
    }
    return *((*session).transfer.received)
        .offset((blocknr / 8 as libc::c_int as u_int32_t) as isize) as libc::c_int
        & (1 as libc::c_int) << blocknr % 8 as libc::c_int as u_int32_t;
}
#[no_mangle]
pub unsafe extern "C" fn dump_blockmap(
    mut postfix: *const libc::c_char,
    mut xfer: *const ttp_transfer_t,
) {
    let mut fbits: *mut FILE = 0 as *mut FILE;
    let mut fname: *mut libc::c_char = 0 as *mut libc::c_char;
    fname = calloc(
        (strlen((*xfer).local_filename))
            .wrapping_add(strlen(postfix))
            .wrapping_add(1 as libc::c_int as libc::c_ulong),
        ::core::mem::size_of::<u_char>() as libc::c_ulong,
    ) as *mut libc::c_char;
    strcpy(fname, (*xfer).local_filename);
    strcat(fname, postfix);
    fbits = fopen(fname, b"wb\0" as *const u8 as *const libc::c_char);
    if !fbits.is_null() {
        fwrite(
            &(*xfer).block_count as *const u_int32_t as *const libc::c_void,
            ::core::mem::size_of::<u_int32_t>() as libc::c_ulong,
            1 as libc::c_int as libc::c_ulong,
            fbits,
        );
        fwrite(
            (*xfer).received as *const libc::c_void,
            ::core::mem::size_of::<u_char>() as libc::c_ulong,
            ((*xfer).block_count / 8 as libc::c_int as u_int32_t)
                .wrapping_add(1 as libc::c_int as u_int32_t) as libc::c_ulong,
            fbits,
        );
        fclose(fbits);
    } else {
        fprintf(
            stderr,
            b"Could not create a file for the blockmap dump\0" as *const u8 as *const libc::c_char,
        );
    }
    free(fname as *mut libc::c_void);
}
