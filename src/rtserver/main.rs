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
    fn __errno_location() -> *mut libc::c_int;
    fn fcntl(__fd: libc::c_int, __cmd: libc::c_int, _: ...) -> libc::c_int;
    static mut optarg: *mut libc::c_char;
    static mut optind: libc::c_int;
    fn getopt_long(
        ___argc: libc::c_int,
        ___argv: *const *mut libc::c_char,
        __shortopts: *const libc::c_char,
        __longopts: *const option,
        __longind: *mut libc::c_int,
    ) -> libc::c_int;
    fn close(__fd: libc::c_int) -> libc::c_int;
    fn read(__fd: libc::c_int, __buf: *mut libc::c_void, __nbytes: size_t) -> ssize_t;
    fn signal(__sig: libc::c_int, __handler: __sighandler_t) -> __sighandler_t;
    fn strtol(
        _: *const libc::c_char,
        _: *mut *mut libc::c_char,
        _: libc::c_int,
    ) -> libc::c_long;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn exit(_: libc::c_int) -> !;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn sendto(
        __fd: libc::c_int,
        __buf: *const libc::c_void,
        __n: size_t,
        __flags: libc::c_int,
        __addr: __CONST_SOCKADDR_ARG,
        __addr_len: socklen_t,
    ) -> ssize_t;
    fn accept(
        __fd: libc::c_int,
        __addr: __SOCKADDR_ARG,
        __addr_len: *mut socklen_t,
    ) -> libc::c_int;
    fn stat(__file: *const libc::c_char, __buf: *mut stat) -> libc::c_int;
    fn inet_ntoa(__in: in_addr) -> *mut libc::c_char;
    fn waitpid(
        __pid: __pid_t,
        __stat_loc: *mut libc::c_int,
        __options: libc::c_int,
    ) -> __pid_t;
    static mut stderr: *mut FILE;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    fn xscript_data_stop(session: *mut ttp_session_t, epoch: *const timeval);
    fn xscript_data_start(session: *mut ttp_session_t, epoch: *const timeval);
    fn xscript_data_log(session: *mut ttp_session_t, logline: *const libc::c_char);
    fn xscript_close(session: *mut ttp_session_t, delta: u_int64_t);
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    static PROTOCOL_REVISION: u_int32_t;
    static REQUEST_STOP: u_int16_t;
    static mut g_error: [libc::c_char; 0];
    fn get_usec_since(old_time: *mut timeval) -> u_int64_t;
    fn usleep_that_works(usec: u_int64_t);
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    static DEFAULT_TCP_PORT: u_int16_t;
    static DEFAULT_UDP_BUFFER: u_int32_t;
    static DEFAULT_VERBOSE_YN: u_char;
    static DEFAULT_TRANSCRIPT_YN: u_char;
    static DEFAULT_IPV6_YN: u_char;
    static DEFAULT_HEARTBEAT_TIMEOUT: u_int16_t;
    fn reset_server(parameter: *mut ttp_parameter_t);
    fn build_datagram(
        session: *mut ttp_session_t,
        block_index: u_int32_t,
        block_type: u_int16_t,
        datagram: *mut u_char,
    ) -> libc::c_int;
    fn stop_vsib(session: *mut ttp_session_t);
    fn create_tcp_socket(parameter: *mut ttp_parameter_t) -> libc::c_int;
    fn ttp_accept_retransmit(
        session: *mut ttp_session_t,
        retransmission: *mut retransmission_t,
        datagram: *mut u_char,
    ) -> libc::c_int;
    fn ttp_authenticate(
        session: *mut ttp_session_t,
        secret: *const u_char,
    ) -> libc::c_int;
    fn ttp_negotiate(session: *mut ttp_session_t) -> libc::c_int;
    fn ttp_open_port(session: *mut ttp_session_t) -> libc::c_int;
    fn ttp_open_transfer(session: *mut ttp_session_t) -> libc::c_int;
    static mut vsib_mode: libc::c_int;
    static mut vsib_mode_gigabit: libc::c_int;
    static mut vsib_mode_embed_1pps_markers: libc::c_int;
    static mut vsib_mode_skip_samples: libc::c_int;
}
pub type __u_char = libc::c_uchar;
pub type __uint8_t = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __int64_t = libc::c_long;
pub type __uint64_t = libc::c_ulong;
pub type __dev_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __ino_t = libc::c_ulong;
pub type __mode_t = libc::c_uint;
pub type __nlink_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __blksize_t = libc::c_long;
pub type __blkcnt_t = libc::c_long;
pub type __ssize_t = libc::c_long;
pub type __syscall_slong_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type pid_t = __pid_t;
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct stat {
    pub st_dev: __dev_t,
    pub st_ino: __ino_t,
    pub st_nlink: __nlink_t,
    pub st_mode: __mode_t,
    pub st_uid: __uid_t,
    pub st_gid: __gid_t,
    pub __pad0: libc::c_int,
    pub st_rdev: __dev_t,
    pub st_size: __off_t,
    pub st_blksize: __blksize_t,
    pub st_blocks: __blkcnt_t,
    pub st_atim: timespec,
    pub st_mtim: timespec,
    pub st_ctim: timespec,
    pub __glibc_reserved: [__syscall_slong_t; 3],
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct option {
    pub name: *const libc::c_char,
    pub has_arg: libc::c_int,
    pub flag: *mut libc::c_int,
    pub val: libc::c_int,
}
pub type __sighandler_t = Option::<unsafe extern "C" fn(libc::c_int) -> ()>;
pub type ssize_t = __ssize_t;
pub type socklen_t = __socklen_t;
pub type u_char = __u_char;
pub type int64_t = __int64_t;
pub type u_int16_t = __uint16_t;
pub type u_int32_t = __uint32_t;
pub type u_int64_t = __uint64_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
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
    pub __in6_u: C2RustUnnamed,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union C2RustUnnamed {
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
unsafe extern "C" fn atoi(mut __nptr: *const libc::c_char) -> libc::c_int {
    return strtol(
        __nptr,
        0 as *mut libc::c_void as *mut *mut libc::c_char,
        10 as libc::c_int,
    ) as libc::c_int;
}
#[inline]
unsafe extern "C" fn __bswap_16(mut __bsx: __uint16_t) -> __uint16_t {
    return (__bsx as libc::c_int >> 8 as libc::c_int & 0xff as libc::c_int
        | (__bsx as libc::c_int & 0xff as libc::c_int) << 8 as libc::c_int)
        as __uint16_t;
}
unsafe fn main_0(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
) -> libc::c_int {
    let mut server_fd: libc::c_int = 0;
    let mut client_fd: libc::c_int = 0;
    let mut remote_address: sockaddr_in = sockaddr_in {
        sin_family: 0,
        sin_port: 0,
        sin_addr: in_addr { s_addr: 0 },
        sin_zero: [0; 8],
    };
    let mut remote_length: socklen_t = ::core::mem::size_of::<sockaddr_in>()
        as libc::c_ulong as socklen_t;
    let mut parameter: ttp_parameter_t = ttp_parameter_t {
        epoch: 0,
        verbose_yn: 0,
        transcript_yn: 0,
        ipv6_yn: 0,
        tcp_port: 0,
        udp_buffer: 0,
        hb_timeout: 0,
        secret: 0 as *const u_char,
        client: 0 as *const libc::c_char,
        finishhook: 0 as *const u_char,
        allhook: 0 as *const u_char,
        block_size: 0,
        file_size: 0,
        block_count: 0,
        target_rate: 0,
        error_rate: 0,
        ipd_time: 0,
        slower_num: 0,
        slower_den: 0,
        faster_num: 0,
        faster_den: 0,
        ringbuf: 0 as *mut libc::c_char,
        fileout: 0,
        slotnumber: 0,
        totalslots: 0,
        samplerate: 0,
        file_names: 0 as *mut *mut libc::c_char,
        file_sizes: 0 as *mut size_t,
        file_name_size: 0,
        total_files: 0,
        wait_u_sec: 0,
    };
    let mut session: ttp_session_t = ttp_session_t {
        parameter: 0 as *mut ttp_parameter_t,
        transfer: ttp_transfer_t {
            parameter: 0 as *mut ttp_parameter_t,
            filename: 0 as *mut libc::c_char,
            file: 0 as *mut FILE,
            vsib: 0 as *mut FILE,
            transcript: 0 as *mut FILE,
            udp_fd: 0,
            udp_address: 0 as *mut sockaddr,
            udp_length: 0,
            ipd_current: 0.,
            block: 0,
        },
        client_fd: 0,
        session_id: 0,
    };
    let mut child_pid: pid_t = 0;
    memset(
        &mut session as *mut ttp_session_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_session_t>() as libc::c_ulong,
    );
    reset_server(&mut parameter);
    process_options(argc, argv, &mut parameter);
    server_fd = create_tcp_socket(&mut parameter);
    if server_fd < 0 as libc::c_int {
        sprintf(
            g_error.as_mut_ptr(),
            b"Could not create server socket on port %d\0" as *const u8
                as *const libc::c_char,
            parameter.tcp_port as libc::c_int,
        );
        return error_handler(
            b"main.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            g_error.as_mut_ptr(),
            1 as libc::c_int,
        );
    }
    fprintf(
        stderr,
        b"Tsunami Realtime Server for protocol rev %X\nRevision: %s\nCompiled: %s %s\n   /dev/vsib VSIB accesses mode=%d, sample skip=%d, gigabit=%d, 1pps embed=%d\nWaiting for clients to connect.\n\0"
            as *const u8 as *const libc::c_char,
        PROTOCOL_REVISION,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
        b"Nov 16 2023\0" as *const u8 as *const libc::c_char,
        b"21:24:20\0" as *const u8 as *const libc::c_char,
        vsib_mode,
        vsib_mode_skip_samples,
        vsib_mode_gigabit,
        vsib_mode_embed_1pps_markers,
    );
    loop {
        client_fd = accept(
            server_fd,
            __SOCKADDR_ARG {
                __sockaddr__: &mut remote_address as *mut sockaddr_in as *mut sockaddr,
            },
            &mut remote_length,
        );
        if client_fd < 0 as libc::c_int {
            error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                144 as libc::c_int,
                b"Could not accept client connection\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            fprintf(
                stderr,
                b"New client connecting from %s...\n\0" as *const u8
                    as *const libc::c_char,
                inet_ntoa(remote_address.sin_addr),
            );
            session.session_id += 1;
            session.session_id;
            session.client_fd = client_fd;
            session.parameter = &mut parameter;
            memset(
                &mut session.transfer as *mut ttp_transfer_t as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<ttp_transfer_t>() as libc::c_ulong,
            );
            session.transfer.ipd_current = 0.0f64;
            client_handler(&mut session);
            close(client_fd);
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn client_handler(mut session: *mut ttp_session_t) {
    let mut retransmission: retransmission_t = retransmission_t {
        request_type: 0,
        block: 0,
        error_rate: 0,
    };
    let mut start: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut stop: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut prevpacketT: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut currpacketT: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut lastfeedback: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut lasthblostreport: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut deadconnection_counter: u_int32_t = 0;
    let mut retransmitlen: libc::c_int = 0 as libc::c_int;
    let mut datagram: [u_char; 65536] = [0; 65536];
    let mut ipd_time: int64_t = 0;
    let mut ipd_usleep_diff: int64_t = 0;
    let mut ipd_time_max: int64_t = 0;
    let mut status: libc::c_int = 0;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    let mut delta: u_int64_t = 0;
    let mut block_type: u_char = 0;
    status = ttp_negotiate(session);
    if status < 0 as libc::c_int {
        error_handler(
            b"main.c\0" as *const u8 as *const libc::c_char,
            232 as libc::c_int,
            b"Protocol revision number mismatch\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = ttp_authenticate(session, (*(*session).parameter).secret);
    if status < 0 as libc::c_int {
        error_handler(
            b"main.c\0" as *const u8 as *const libc::c_char,
            237 as libc::c_int,
            b"Client authentication failure\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if 1 as libc::c_int == (*param).verbose_yn as libc::c_int {
        fprintf(
            stderr,
            b"Client authenticated. Negotiated parameters are:\n\0" as *const u8
                as *const libc::c_char,
        );
        fprintf(
            stderr,
            b"Block size: %d\n\0" as *const u8 as *const libc::c_char,
            (*param).block_size,
        );
        fprintf(
            stderr,
            b"Buffer size: %d\n\0" as *const u8 as *const libc::c_char,
            (*param).udp_buffer,
        );
        fprintf(
            stderr,
            b"Port: %d\n\0" as *const u8 as *const libc::c_char,
            (*param).tcp_port as libc::c_int,
        );
    }
    loop {
        status = fcntl((*session).client_fd, 4 as libc::c_int, 0 as libc::c_int);
        if status < 0 as libc::c_int {
            error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                252 as libc::c_int,
                b"Could not make client socket blocking\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
            );
        }
        status = ttp_open_transfer(session);
        if status < 0 as libc::c_int {
            error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                257 as libc::c_int,
                b"Invalid file request\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
            return;
        }
        status = ttp_open_port(session);
        if status < 0 as libc::c_int {
            error_handler(
                b"main.c\0" as *const u8 as *const libc::c_char,
                268 as libc::c_int,
                b"UDP socket creation failed\0" as *const u8 as *const libc::c_char,
                0 as libc::c_int,
            );
        } else {
            status = fcntl(
                (*session).client_fd,
                4 as libc::c_int,
                0o4000 as libc::c_int,
            );
            if status < 0 as libc::c_int {
                error_handler(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    275 as libc::c_int,
                    b"Could not make client socket non-blocking\0" as *const u8
                        as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
            gettimeofday(&mut start, 0 as *mut libc::c_void);
            if (*param).transcript_yn != 0 {
                xscript_data_start(session, &mut start);
            }
            lasthblostreport = start;
            lastfeedback = start;
            prevpacketT = start;
            deadconnection_counter = 0 as libc::c_int as u_int32_t;
            ipd_time = 0 as libc::c_int as int64_t;
            ipd_time_max = 0 as libc::c_int as int64_t;
            ipd_usleep_diff = 0 as libc::c_int as int64_t;
            retransmitlen = 0 as libc::c_int;
            (*xfer).block = 0 as libc::c_int as u_int32_t;
            while (*xfer).block <= (*param).block_count {
                block_type = 'R' as i32 as u_char;
                gettimeofday(&mut currpacketT, 0 as *mut libc::c_void);
                ipd_usleep_diff = ((*xfer).ipd_current
                    + ((prevpacketT.tv_sec - currpacketT.tv_sec) as libc::c_double
                        * 1e6f64
                        + (prevpacketT.tv_usec - currpacketT.tv_usec) as libc::c_double))
                    as int64_t;
                prevpacketT = currpacketT;
                if ipd_usleep_diff > 0 as libc::c_int as int64_t
                    || ipd_time > 0 as libc::c_int as int64_t
                {
                    ipd_time += ipd_usleep_diff;
                }
                ipd_time_max = if ipd_time > ipd_time_max {
                    ipd_time
                } else {
                    ipd_time_max
                };
                status = read(
                    (*session).client_fd,
                    (&mut retransmission as *mut retransmission_t as *mut libc::c_char)
                        .offset(retransmitlen as isize) as *mut libc::c_void,
                    (::core::mem::size_of::<retransmission_t>() as libc::c_ulong)
                        .wrapping_sub(retransmitlen as libc::c_ulong),
                ) as libc::c_int;
                if status <= 0 as libc::c_int && *__errno_location() != 11 as libc::c_int
                    && (*(*session).parameter).fileout == 0
                {
                    error_handler(
                        b"main.c\0" as *const u8 as *const libc::c_char,
                        316 as libc::c_int,
                        b"Retransmission read failed and not writing local backup file\0"
                            as *const u8 as *const libc::c_char,
                        1 as libc::c_int,
                    );
                }
                if status > 0 as libc::c_int {
                    retransmitlen += status;
                }
                if retransmitlen as libc::c_ulong
                    == ::core::mem::size_of::<retransmission_t>() as libc::c_ulong
                {
                    lastfeedback = currpacketT;
                    lasthblostreport = currpacketT;
                    deadconnection_counter = 0 as libc::c_int as u_int32_t;
                    if __bswap_16(retransmission.request_type) as libc::c_int
                        == REQUEST_STOP as libc::c_int
                    {
                        fprintf(
                            stderr,
                            b"Transmission complete.\n\0" as *const u8
                                as *const libc::c_char,
                        );
                        break;
                    } else {
                        status = ttp_accept_retransmit(
                            session,
                            &mut retransmission,
                            datagram.as_mut_ptr(),
                        );
                        if status < 0 as libc::c_int {
                            error_handler(
                                b"main.c\0" as *const u8 as *const libc::c_char,
                                339 as libc::c_int,
                                b"Retransmission error\0" as *const u8
                                    as *const libc::c_char,
                                0 as libc::c_int,
                            );
                        }
                        retransmitlen = 0 as libc::c_int;
                    }
                } else if (retransmitlen as libc::c_ulong)
                    < ::core::mem::size_of::<retransmission_t>() as libc::c_ulong
                {
                    (*xfer)
                        .block = if ((*xfer).block)
                        .wrapping_add(1 as libc::c_int as u_int32_t)
                        < (*param).block_count
                    {
                        ((*xfer).block).wrapping_add(1 as libc::c_int as u_int32_t)
                    } else {
                        (*param).block_count
                    };
                    block_type = (if (*xfer).block == (*param).block_count {
                        'X' as i32
                    } else {
                        'O' as i32
                    }) as u_char;
                    status = build_datagram(
                        session,
                        (*xfer).block,
                        block_type as u_int16_t,
                        datagram.as_mut_ptr(),
                    );
                    if status < 0 as libc::c_int {
                        sprintf(
                            g_error.as_mut_ptr(),
                            b"Could not read block #%u\0" as *const u8
                                as *const libc::c_char,
                            (*xfer).block,
                        );
                        error_handler(
                            b"main.c\0" as *const u8 as *const libc::c_char,
                            351 as libc::c_int,
                            g_error.as_mut_ptr(),
                            1 as libc::c_int,
                        );
                    }
                    status = sendto(
                        (*xfer).udp_fd,
                        datagram.as_mut_ptr() as *const libc::c_void,
                        (6 as libc::c_int as u_int32_t).wrapping_add((*param).block_size)
                            as size_t,
                        0 as libc::c_int,
                        __CONST_SOCKADDR_ARG {
                            __sockaddr__: (*xfer).udp_address,
                        },
                        (*xfer).udp_length,
                    ) as libc::c_int;
                    if status < 0 as libc::c_int {
                        sprintf(
                            g_error.as_mut_ptr(),
                            b"Could not transmit block #%u\0" as *const u8
                                as *const libc::c_char,
                            (*xfer).block,
                        );
                        error_handler(
                            b"main.c\0" as *const u8 as *const libc::c_char,
                            358 as libc::c_int,
                            g_error.as_mut_ptr(),
                            0 as libc::c_int,
                        );
                        continue;
                    }
                } else if retransmitlen as libc::c_ulong
                    > ::core::mem::size_of::<retransmission_t>() as libc::c_ulong
                {
                    fprintf(
                        stderr,
                        b"warn: retransmitlen > %d\n\0" as *const u8
                            as *const libc::c_char,
                        ::core::mem::size_of::<retransmission_t>() as libc::c_ulong
                            as libc::c_int,
                    );
                    retransmitlen = 0 as libc::c_int;
                }
                let fresh0 = deadconnection_counter;
                deadconnection_counter = deadconnection_counter.wrapping_add(1);
                if fresh0 > 2048 as libc::c_int as u_int32_t {
                    let mut stats_line: [libc::c_char; 160] = [0; 160];
                    deadconnection_counter = 0 as libc::c_int as u_int32_t;
                    if (get_usec_since(&mut lasthblostreport) as libc::c_double)
                        < 500000.0f64
                    {
                        continue;
                    }
                    gettimeofday(&mut lasthblostreport, 0 as *mut libc::c_void);
                    delta = get_usec_since(&mut lastfeedback);
                    snprintf(
                        stats_line.as_mut_ptr(),
                        (::core::mem::size_of::<[libc::c_char; 160]>() as libc::c_ulong)
                            .wrapping_sub(1 as libc::c_int as libc::c_ulong),
                        b"   n/a     n/a     n/a %7u %6.2f %3u -- no heartbeat since %3.2fs\n\0"
                            as *const u8 as *const libc::c_char,
                        (*xfer).block,
                        100.0f64 * (*xfer).block as libc::c_double
                            / (*param).block_count as libc::c_double,
                        (*session).session_id,
                        1e-6f64 * delta as libc::c_double,
                    );
                    if (*param).transcript_yn != 0 {
                        xscript_data_log(session, stats_line.as_mut_ptr());
                    }
                    fprintf(
                        stderr,
                        b"%s\0" as *const u8 as *const libc::c_char,
                        stats_line.as_mut_ptr(),
                    );
                    if 1e-6f64 * delta as libc::c_double
                        > (*param).hb_timeout as libc::c_int as libc::c_double
                    {
                        if (*(*session).parameter).fileout as libc::c_int != 0
                            && block_type as libc::c_int == 'X' as i32
                        {
                            fprintf(
                                stderr,
                                b"Reached the Terminate block and timed out, terminating transfer.\n\0"
                                    as *const u8 as *const libc::c_char,
                            );
                            break;
                        } else if (*(*session).parameter).fileout == 0 {
                            fprintf(
                                stderr,
                                b"Heartbeat timeout of %d seconds reached and not doing local backup, terminating transfer now.\n\0"
                                    as *const u8 as *const libc::c_char,
                                (*param).hb_timeout as libc::c_int,
                            );
                            break;
                        } else {
                            lastfeedback = currpacketT;
                        }
                    }
                }
                if block_type as libc::c_int == 'X' as i32 {
                    usleep_that_works(
                        (10 as libc::c_int as int64_t * ipd_time_max) as u_int64_t,
                    );
                }
                if ipd_time > 0 as libc::c_int as int64_t {
                    usleep_that_works(ipd_time as u_int64_t);
                }
            }
            gettimeofday(&mut stop, 0 as *mut libc::c_void);
            if (*param).transcript_yn != 0 {
                xscript_data_stop(session, &mut stop);
            }
            delta = (1000000 as libc::c_longlong
                * (stop.tv_sec - start.tv_sec) as libc::c_longlong
                + stop.tv_usec as libc::c_longlong - start.tv_usec as libc::c_longlong)
                as u_int64_t;
            if (*param).verbose_yn != 0 {
                fprintf(
                    stderr,
                    b"Server %d transferred %llu bytes in %0.2f seconds (%0.1f Mbps)\n\0"
                        as *const u8 as *const libc::c_char,
                    (*session).session_id,
                    (*param).file_size as ull_t,
                    delta as libc::c_double / 1000000.0f64,
                    8.0f64 * (*param).file_size as libc::c_double
                        / (delta as libc::c_double * 1e-6f64
                            * 1024 as libc::c_int as libc::c_double
                            * 1024 as libc::c_int as libc::c_double),
                );
            }
            if (*param).transcript_yn != 0 {
                xscript_close(session, delta);
            }
            if (*param).fileout != 0 {
                fclose((*xfer).file);
            }
            stop_vsib(session);
            fclose((*xfer).vsib);
            close((*xfer).udp_fd);
            memset(
                xfer as *mut libc::c_void,
                0 as libc::c_int,
                ::core::mem::size_of::<ttp_transfer_t>() as libc::c_ulong,
            );
        }
    };
}
#[no_mangle]
pub unsafe extern "C" fn process_options(
    mut argc: libc::c_int,
    mut argv: *mut *mut libc::c_char,
    mut parameter: *mut ttp_parameter_t,
) {
    let mut long_options: [option; 11] = [
        {
            let mut init = option {
                name: b"verbose\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"transcript\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 't' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"v6\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: '6' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"port\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'p' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"secret\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 's' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"buffer\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'b' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"hbtimeout\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'h' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"v\0" as *const u8 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'v' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsibmode\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'M' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: b"vsibskip\0" as *const u8 as *const libc::c_char,
                has_arg: 1 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 'S' as i32,
            };
            init
        },
        {
            let mut init = option {
                name: 0 as *const libc::c_char,
                has_arg: 0 as libc::c_int,
                flag: 0 as *mut libc::c_int,
                val: 0 as libc::c_int,
            };
            init
        },
    ];
    let mut filestat: stat = stat {
        st_dev: 0,
        st_ino: 0,
        st_nlink: 0,
        st_mode: 0,
        st_uid: 0,
        st_gid: 0,
        __pad0: 0,
        st_rdev: 0,
        st_size: 0,
        st_blksize: 0,
        st_blocks: 0,
        st_atim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_mtim: timespec { tv_sec: 0, tv_nsec: 0 },
        st_ctim: timespec { tv_sec: 0, tv_nsec: 0 },
        __glibc_reserved: [0; 3],
    };
    let mut which: libc::c_int = 0;
    loop {
        which = getopt_long(
            argc,
            argv as *const *mut libc::c_char,
            b"+\0" as *const u8 as *const libc::c_char,
            long_options.as_mut_ptr(),
            0 as *mut libc::c_int,
        );
        if !(which > 0 as libc::c_int) {
            break;
        }
        match which {
            118 => {
                (*parameter).verbose_yn = 1 as libc::c_int as u_char;
            }
            116 => {
                (*parameter).transcript_yn = 1 as libc::c_int as u_char;
            }
            54 => {
                (*parameter).ipv6_yn = 1 as libc::c_int as u_char;
            }
            112 => {
                (*parameter).tcp_port = atoi(optarg) as u_int16_t;
            }
            115 => {
                (*parameter).secret = optarg as *mut libc::c_uchar;
            }
            98 => {
                (*parameter).udp_buffer = atoi(optarg) as u_int32_t;
            }
            104 => {
                (*parameter).hb_timeout = atoi(optarg) as u_int16_t;
            }
            77 => {
                vsib_mode = atoi(optarg);
            }
            83 => {
                vsib_mode_skip_samples = atoi(optarg);
            }
            _ => {
                fprintf(
                    stderr,
                    b"Usage: tsunamid [--verbose] [--transcript] [--v6] [--port=n] [--buffer=bytes]\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"                [--hbtimeout=seconds] \0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"[--vsibmode=mode] [--vsibskip=skip] [filename1 filename2 ...]\n\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"verbose or v : turns on verbose output mode\n\0" as *const u8
                        as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"transcript   : turns on transcript mode for statistics recording\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"v6           : operates using IPv6 instead of (not in addition to!) IPv4\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"port         : specifies which TCP port on which to listen to incoming connections\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"secret       : specifies the shared secret for the client and server\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"buffer       : specifies the desired size for UDP socket send buffer (in bytes)\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"hbtimeout    : specifies the timeout in seconds for disconnect after client heartbeat lost\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"vsibmode     : specifies the VSIB mode to use (see VSIB documentation for modes)\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"vsibskip     : a value N other than 0 will skip N samples after every 1 sample\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(
                    stderr,
                    b"filenames    : list of files to share for downloaded via a client 'GET *'\n\0"
                        as *const u8 as *const libc::c_char,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                fprintf(
                    stderr,
                    b"Defaults: verbose    = %d\n\0" as *const u8 as *const libc::c_char,
                    DEFAULT_VERBOSE_YN as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"          transcript = %d\n\0" as *const u8 as *const libc::c_char,
                    DEFAULT_TRANSCRIPT_YN as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"          v6         = %d\n\0" as *const u8 as *const libc::c_char,
                    DEFAULT_IPV6_YN as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"          port       = %d\n\0" as *const u8 as *const libc::c_char,
                    DEFAULT_TCP_PORT as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"          buffer     = %d bytes\n\0" as *const u8
                        as *const libc::c_char,
                    DEFAULT_UDP_BUFFER,
                );
                fprintf(
                    stderr,
                    b"          hbtimeout  = %d seconds\n\0" as *const u8
                        as *const libc::c_char,
                    DEFAULT_HEARTBEAT_TIMEOUT as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"          vsibmode   = %d\n\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                fprintf(
                    stderr,
                    b"          vsibskip   = %d\n\0" as *const u8 as *const libc::c_char,
                    0 as libc::c_int,
                );
                fprintf(stderr, b"\n\0" as *const u8 as *const libc::c_char);
                exit(1 as libc::c_int);
            }
        }
    }
    if argc > optind {
        let mut counter: libc::c_int = 0;
        (*parameter).file_names = argv.offset(optind as isize);
        (*parameter).file_name_size = 0 as libc::c_int as u_int16_t;
        (*parameter).total_files = (argc - optind) as u_int16_t;
        (*parameter)
            .file_sizes = malloc(
            (::core::mem::size_of::<size_t>() as libc::c_ulong)
                .wrapping_mul((*parameter).total_files as libc::c_ulong),
        ) as *mut size_t;
        fprintf(
            stderr,
            b"\nThe specified %d files will be listed on GET *:\n\0" as *const u8
                as *const libc::c_char,
            (*parameter).total_files as libc::c_int,
        );
        counter = 0 as libc::c_int;
        while counter < argc - optind {
            stat(*((*parameter).file_names).offset(counter as isize), &mut filestat);
            *((*parameter).file_sizes)
                .offset(counter as isize) = filestat.st_size as size_t;
            (*parameter)
                .file_name_size = ((*parameter).file_name_size as libc::c_ulong)
                .wrapping_add(
                    (strlen(*((*parameter).file_names).offset(counter as isize)))
                        .wrapping_add(1 as libc::c_int as libc::c_ulong),
                ) as u_int16_t as u_int16_t;
            fprintf(
                stderr,
                b" %3d)   %-20s  %llu bytes\n\0" as *const u8 as *const libc::c_char,
                counter + 1 as libc::c_int,
                *((*parameter).file_names).offset(counter as isize),
                *((*parameter).file_sizes).offset(counter as isize) as ull_t,
            );
            counter += 1;
            counter;
        }
        fprintf(
            stderr,
            b"total characters %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).file_name_size as libc::c_int,
        );
    }
    if 1 as libc::c_int == (*parameter).verbose_yn as libc::c_int {
        fprintf(
            stderr,
            b"Block size: %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).block_size,
        );
        fprintf(
            stderr,
            b"Buffer size: %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).udp_buffer,
        );
        fprintf(
            stderr,
            b"Port: %d\n\0" as *const u8 as *const libc::c_char,
            (*parameter).tcp_port as libc::c_int,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn reap(mut signum: libc::c_int) {
    let mut status: libc::c_int = 0;
    while waitpid(-(1 as libc::c_int), &mut status, 1 as libc::c_int) > 0 as libc::c_int
    {
        fprintf(
            stderr,
            b"Child server process terminated with status code 0x%X\n\0" as *const u8
                as *const libc::c_char,
            status,
        );
    }
    signal(17 as libc::c_int, Some(reap as unsafe extern "C" fn(libc::c_int) -> ()));
}
pub fn main() {
    let mut args: Vec::<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(
            main_0(
                (args.len() - 1) as libc::c_int,
                args.as_mut_ptr() as *mut *mut libc::c_char,
            ) as i32,
        )
    }
}
