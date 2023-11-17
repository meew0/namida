use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn __ctype_b_loc() -> *mut *const libc::c_ushort;
    fn exit(_: libc::c_int) -> !;
    fn memset(_: *mut libc::c_void, _: libc::c_int, _: libc::c_ulong) -> *mut libc::c_void;
    fn strcpy(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcat(_: *mut libc::c_char, _: *const libc::c_char) -> *mut libc::c_char;
    fn strcasecmp(_: *const libc::c_char, _: *const libc::c_char) -> libc::c_int;
    static mut stdin: *mut FILE;
    static mut stdout: *mut FILE;
    static mut stderr: *mut FILE;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fgets(__s: *mut libc::c_char, __n: libc::c_int, __stream: *mut FILE) -> *mut libc::c_char;
    static PROTOCOL_REVISION: u_int32_t;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    fn command_close(command: *mut command_t, session: *mut ttp_session_t) -> libc::c_int;
    fn command_connect(
        command: *mut command_t,
        parameter: *mut ttp_parameter_t,
    ) -> *mut ttp_session_t;
    fn command_get(command: *mut command_t, session: *mut ttp_session_t) -> libc::c_int;
    fn command_help(command: *mut command_t, session: *mut ttp_session_t) -> libc::c_int;
    fn command_quit(command: *mut command_t, session: *mut ttp_session_t) -> libc::c_int;
    fn command_set(command: *mut command_t, parameter: *mut ttp_parameter_t) -> libc::c_int;
    fn command_dir(command: *mut command_t, session: *mut ttp_session_t) -> libc::c_int;
    fn reset_client(parameter: *mut ttp_parameter_t);
}
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type C2RustUnnamed = libc::c_uint;
pub const _ISalnum: C2RustUnnamed = 8;
pub const _ISpunct: C2RustUnnamed = 4;
pub const _IScntrl: C2RustUnnamed = 2;
pub const _ISblank: C2RustUnnamed = 1;
pub const _ISgraph: C2RustUnnamed = 32768;
pub const _ISprint: C2RustUnnamed = 16384;
pub const _ISspace: C2RustUnnamed = 8192;
pub const _ISxdigit: C2RustUnnamed = 4096;
pub const _ISdigit: C2RustUnnamed = 2048;
pub const _ISalpha: C2RustUnnamed = 1024;
pub const _ISlower: C2RustUnnamed = 512;
pub const _ISupper: C2RustUnnamed = 256;
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
    pub __value32: C2RustUnnamed_0,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct C2RustUnnamed_0 {
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
pub type sa_family_t = libc::c_ushort;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sockaddr {
    pub sa_family: sa_family_t,
    pub sa_data: [libc::c_char; 14],
}
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
pub unsafe fn main_0(mut argc: libc::c_int, mut argv: *mut *const libc::c_char) -> libc::c_int {
    let mut command: command_t = command_t {
        count: 0,
        text: [0 as *const libc::c_char; 10],
    };
    let vla = super::config::MAX_COMMAND_LENGTH as usize;
    let mut command_text: Vec<libc::c_char> = ::std::vec::from_elem(0, vla);
    let mut session: *mut ttp_session_t = 0 as *mut ttp_session_t;
    let mut parameter: ttp_parameter_t = ttp_parameter_t {
        server_name: 0 as *mut libc::c_char,
        server_port: 0,
        client_port: 0,
        udp_buffer: 0,
        verbose_yn: 0,
        transcript_yn: 0,
        ipv6_yn: 0,
        output_mode: 0,
        block_size: 0,
        target_rate: 0,
        rate_adjust: 0,
        error_rate: 0,
        slower_num: 0,
        slower_den: 0,
        faster_num: 0,
        faster_den: 0,
        history: 0,
        lossless: 0,
        losswindow_ms: 0,
        blockdump: 0,
        passphrase: 0 as *mut libc::c_char,
        ringbuf: 0 as *mut libc::c_char,
    };
    let mut argc_curr: libc::c_int = 1 as libc::c_int;
    let mut ptr_command_text: *mut libc::c_char =
        &mut *command_text.as_mut_ptr().offset(0 as libc::c_int as isize) as *mut libc::c_char;
    memset(
        &mut parameter as *mut ttp_parameter_t as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<ttp_parameter_t>() as libc::c_ulong,
    );
    reset_client(&mut parameter);
    fprintf(
        stderr,
        b"Tsunami Client for protocol rev %X\nRevision: %s\nCompiled: %s %s\n\0" as *const u8
            as *const libc::c_char,
        PROTOCOL_REVISION,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
        b"Nov 16 2023\0" as *const u8 as *const libc::c_char,
        b"21:24:18\0" as *const u8 as *const libc::c_char,
    );
    loop {
        if argc <= 1 as libc::c_int || argc_curr >= argc {
            fprintf(stdout, b"tsunami> \0" as *const u8 as *const libc::c_char);
            fflush(stdout);
            if (fgets(
                command_text.as_mut_ptr(),
                super::config::MAX_COMMAND_LENGTH,
                stdin,
            ))
            .is_null()
            {
                error_handler(
                    b"main.c\0" as *const u8 as *const libc::c_char,
                    121 as libc::c_int,
                    b"Could not read command input\0" as *const u8 as *const libc::c_char,
                    1 as libc::c_int,
                );
            }
        } else {
            while argc_curr < argc {
                if strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"close\0" as *const u8 as *const libc::c_char,
                ) == 0
                    || strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"quit\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"exit\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"bye\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"help\0" as *const u8 as *const libc::c_char,
                    ) == 0
                    || strcasecmp(
                        *argv.offset(argc_curr as isize),
                        b"dir\0" as *const u8 as *const libc::c_char,
                    ) == 0
                {
                    strcpy(command_text.as_mut_ptr(), *argv.offset(argc_curr as isize));
                    argc_curr += 1 as libc::c_int;
                    break;
                } else if strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"connect\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (argc_curr + 1 as libc::c_int) < argc {
                        strcpy(ptr_command_text, *argv.offset(argc_curr as isize));
                        strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 1 as libc::c_int) as isize),
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"Connect: no host specified\n\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    argc_curr += 2 as libc::c_int;
                    break;
                } else if strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"get\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (argc_curr + 1 as libc::c_int) < argc {
                        strcpy(ptr_command_text, *argv.offset(argc_curr as isize));
                        strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 1 as libc::c_int) as isize),
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"Get: no file specified\n\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    argc_curr += 2 as libc::c_int;
                    break;
                } else if strcasecmp(
                    *argv.offset(argc_curr as isize),
                    b"set\0" as *const u8 as *const libc::c_char,
                ) == 0
                {
                    if (argc_curr + 2 as libc::c_int) < argc {
                        strcpy(ptr_command_text, *argv.offset(argc_curr as isize));
                        strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 1 as libc::c_int) as isize),
                        );
                        strcat(
                            command_text.as_mut_ptr(),
                            b" \0" as *const u8 as *const libc::c_char,
                        );
                        strcat(
                            command_text.as_mut_ptr(),
                            *argv.offset((argc_curr + 2 as libc::c_int) as isize),
                        );
                    } else {
                        fprintf(
                            stderr,
                            b"Connect: no host specified\n\0" as *const u8 as *const libc::c_char,
                        );
                        exit(1 as libc::c_int);
                    }
                    argc_curr += 3 as libc::c_int;
                    break;
                } else {
                    fprintf(
                        stderr,
                        b"Unsupported command console command: %s\n\0" as *const u8
                            as *const libc::c_char,
                        *argv.offset(argc_curr as isize),
                    );
                    argc_curr += 1;
                    argc_curr;
                }
            }
        }
        parse_command(&mut command, command_text.as_mut_ptr());
        if command.count as libc::c_int == 0 as libc::c_int {
            continue;
        }
        if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"close\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_close(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"connect\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            session = command_connect(&mut command, &mut parameter);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"get\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_get(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"dir\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_dir(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"help\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_help(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"quit\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_quit(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"exit\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_quit(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"bye\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_quit(&mut command, session);
        } else if strcasecmp(
            command.text[0 as libc::c_int as usize],
            b"set\0" as *const u8 as *const libc::c_char,
        ) == 0
        {
            command_set(&mut command, &mut parameter);
        } else {
            fprintf(
                stderr,
                b"Unrecognized command: '%s'.  Use 'HELP' for help.\n\n\0" as *const u8
                    as *const libc::c_char,
                command.text[0 as libc::c_int as usize],
            );
        }
    }
}
#[no_mangle]
pub unsafe extern "C" fn parse_command(mut command: *mut command_t, mut buffer: *mut libc::c_char) {
    (*command).count = 0 as libc::c_int as u_char;
    while *(*__ctype_b_loc()).offset(*buffer as libc::c_int as isize) as libc::c_int
        & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
        != 0
        && *buffer as libc::c_int != 0
    {
        buffer = buffer.offset(1);
        buffer;
    }
    while ((*command).count as libc::c_int) < 10 as libc::c_int && *buffer as libc::c_int != 0 {
        let fresh0 = (*command).count;
        (*command).count = ((*command).count).wrapping_add(1);
        (*command).text[fresh0 as usize] = buffer;
        while *buffer as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*buffer as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                == 0
        {
            buffer = buffer.offset(1);
            buffer;
        }
        while *buffer as libc::c_int != 0
            && *(*__ctype_b_loc()).offset(*buffer as libc::c_int as isize) as libc::c_int
                & _ISspace as libc::c_int as libc::c_ushort as libc::c_int
                != 0
        {
            let fresh1 = buffer;
            buffer = buffer.offset(1);
            *fresh1 = '\0' as i32 as libc::c_char;
        }
    }
}
pub fn main() {
    let mut args: Vec<*mut libc::c_char> = Vec::new();
    for arg in ::std::env::args() {
        args.push(
            (::std::ffi::CString::new(arg))
                .expect("Failed to convert argument into CString.")
                .into_raw(),
        );
    }
    args.push(::core::ptr::null_mut());
    unsafe {
        ::std::process::exit(main_0(
            (args.len() - 1) as libc::c_int,
            args.as_mut_ptr() as *mut *const libc::c_char,
        ) as i32)
    }
}
