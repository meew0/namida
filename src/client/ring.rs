use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn pthread_mutex_init(
        __mutex: *mut pthread_mutex_t,
        __mutexattr: *const pthread_mutexattr_t,
    ) -> libc::c_int;
    fn pthread_mutex_destroy(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_lock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_mutex_unlock(__mutex: *mut pthread_mutex_t) -> libc::c_int;
    fn pthread_cond_init(
        __cond: *mut pthread_cond_t,
        __cond_attr: *const pthread_condattr_t,
    ) -> libc::c_int;
    fn pthread_cond_destroy(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_signal(__cond: *mut pthread_cond_t) -> libc::c_int;
    fn pthread_cond_wait(__cond: *mut pthread_cond_t, __mutex: *mut pthread_mutex_t)
        -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
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
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_mutexattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub union pthread_condattr_t {
    pub __size: [libc::c_char; 4],
    pub __align: libc::c_int,
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
unsafe extern "C" fn __bswap_32(mut __bsx: __uint32_t) -> __uint32_t {
    return (__bsx & 0xff000000 as libc::c_uint) >> 24 as libc::c_int
        | (__bsx & 0xff0000 as libc::c_uint) >> 8 as libc::c_int
        | (__bsx & 0xff00 as libc::c_uint) << 8 as libc::c_int
        | (__bsx & 0xff as libc::c_uint) << 24 as libc::c_int;
}
#[no_mangle]
pub static mut EMPTY: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn ring_full(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut full: libc::c_int = 0;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    full = ((*ring).space_ready == 0) as libc::c_int;
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            98 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return full;
}
#[no_mangle]
pub unsafe extern "C" fn ring_cancel(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).count_reserved -= 1;
    if (*ring).count_reserved < 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            b"Attempt made to cancel unreserved slot in ring buffer\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).space_ready = 1 as libc::c_int;
    status = pthread_cond_signal(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            b"Could not signal space-ready condition\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            132 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ring_confirm(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).count_data += 1;
    (*ring).count_data;
    (*ring).count_reserved -= 1;
    if (*ring).count_reserved < 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            b"Attempt made to confirm unreserved slot in ring buffer\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).data_ready = 1 as libc::c_int;
    status = pthread_cond_signal(&mut (*ring).data_ready_cond);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            b"Could not signal data-ready condition\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            169 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ring_create(mut session: *mut ttp_session_t) -> *mut ring_buffer_t {
    let mut ring: *mut ring_buffer_t = 0 as *mut ring_buffer_t;
    let mut status: libc::c_int = 0;
    ring = calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ring_buffer_t>() as libc::c_ulong,
    ) as *mut ring_buffer_t;
    if ring.is_null() {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            b"Could not allocate ring buffer object\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).datagram_size = (6 as libc::c_int as u_int32_t)
        .wrapping_add((*(*session).parameter).block_size)
        as libc::c_int;
    (*ring).datagrams =
        malloc(((*ring).datagram_size * 4096 as libc::c_int) as libc::c_ulong) as *mut u_char;
    if ((*ring).datagrams).is_null() {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            b"Could not allocate buffer for ring buffer\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = pthread_mutex_init(&mut (*ring).mutex, 0 as *const pthread_mutexattr_t);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            b"Could not create mutex for ring buffer\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = pthread_cond_init(&mut (*ring).data_ready_cond, 0 as *const pthread_condattr_t);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            b"Could not create data-ready condition variable\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).data_ready = 0 as libc::c_int;
    status = pthread_cond_init(
        &mut (*ring).space_ready_cond,
        0 as *const pthread_condattr_t,
    );
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            214 as libc::c_int,
            b"Could not create space-ready condition variable\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).space_ready = 1 as libc::c_int;
    (*ring).count_data = 0 as libc::c_int;
    (*ring).count_reserved = 0 as libc::c_int;
    (*ring).base_data = 0 as libc::c_int;
    return ring;
}
#[no_mangle]
pub unsafe extern "C" fn ring_destroy(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = pthread_mutex_destroy(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        return error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            b"Could not destroy mutex for ring buffer\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = pthread_cond_destroy(&mut (*ring).data_ready_cond);
    if status != 0 as libc::c_int {
        return error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"Could not destroy data-ready condition variable\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = pthread_cond_destroy(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        return error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            b"Could not destroy space-ready condition variable\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    free((*ring).datagrams as *mut libc::c_void);
    free(ring as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ring_dump(
    mut ring: *mut ring_buffer_t,
    mut out: *mut FILE,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut datagram: *mut u_char = 0 as *mut u_char;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        return error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    fprintf(
        out,
        b"datagram_size  = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).datagram_size,
    );
    fprintf(
        out,
        b"base_data      = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).base_data,
    );
    fprintf(
        out,
        b"count_data     = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).count_data,
    );
    fprintf(
        out,
        b"count_reserved = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).count_reserved,
    );
    fprintf(
        out,
        b"data_ready     = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).data_ready,
    );
    fprintf(
        out,
        b"space_ready    = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).space_ready,
    );
    fprintf(
        out,
        b"block list     = [\0" as *const u8 as *const libc::c_char,
    );
    index = (*ring).base_data;
    while index < (*ring).base_data + (*ring).count_data {
        datagram = ((*ring).datagrams)
            .offset((index % 4096 as libc::c_int * (*ring).datagram_size) as isize);
        fprintf(
            out,
            b"%d \0" as *const u8 as *const libc::c_char,
            __bswap_32(*(datagram as *mut u_int32_t)),
        );
        index += 1;
        index;
    }
    fprintf(out, b"]\n\0" as *const u8 as *const libc::c_char);
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        return error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            296 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ring_peek(mut ring: *mut ring_buffer_t) -> *mut u_char {
    let mut status: libc::c_int = 0;
    let mut address: *mut u_char = 0 as *mut u_char;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return 0 as *mut u_char;
    }
    while (*ring).data_ready == 0 as libc::c_int {
        status = pthread_cond_wait(&mut (*ring).data_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            error_handler(
                b"ring.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int,
                b"Could not wait for ring buffer to accumulate data\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
            return 0 as *mut u_char;
        }
    }
    address = ((*ring).datagrams).offset(((*ring).datagram_size * (*ring).base_data) as isize);
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
        return 0 as *mut u_char;
    }
    return address;
}
#[no_mangle]
pub unsafe extern "C" fn ring_pop(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    while (*ring).data_ready == 0 as libc::c_int {
        status = pthread_cond_wait(&mut (*ring).data_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            error_handler(
                b"ring.c\0" as *const u8 as *const libc::c_char,
                367 as libc::c_int,
                b"Could not wait for ring buffer to accumulate data\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
            );
        }
    }
    (*ring).base_data = ((*ring).base_data + 1 as libc::c_int) % 4096 as libc::c_int;
    (*ring).count_data -= 1;
    if (*ring).count_data == 0 as libc::c_int {
        (*ring).data_ready = 0 as libc::c_int;
    }
    (*ring).space_ready = 1 as libc::c_int;
    status = pthread_cond_signal(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            379 as libc::c_int,
            b"Could not signal space-ready condition\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            384 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ring_reserve(mut ring: *mut ring_buffer_t) -> *mut u_char {
    let mut status: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut address: *mut u_char = 0 as *mut u_char;
    status = pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    next = ((*ring).base_data + (*ring).count_data + (*ring).count_reserved) % 4096 as libc::c_int;
    while (*ring).space_ready == 0 as libc::c_int {
        printf(b"FULL! -- ring_reserve() blocking.\n\0" as *const u8 as *const libc::c_char);
        printf(
            b"space_ready = %d, data_ready = %d\n\0" as *const u8 as *const libc::c_char,
            (*ring).space_ready,
            (*ring).data_ready,
        );
        status = pthread_cond_wait(&mut (*ring).space_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            error_handler(
                b"ring.c\0" as *const u8 as *const libc::c_char,
                419 as libc::c_int,
                b"Could not wait for ring buffer to clear space\0" as *const u8
                    as *const libc::c_char,
                1 as libc::c_int,
            );
        }
    }
    (*ring).count_reserved += 1;
    if (*ring).count_reserved > 1 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            424 as libc::c_int,
            b"Attempt made to reserve two slots in ring buffer\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    if (next + 1 as libc::c_int) % 4096 as libc::c_int == (*ring).base_data {
        (*ring).space_ready = 0 as libc::c_int;
    }
    address = ((*ring).datagrams).offset((next * (*ring).datagram_size) as isize);
    status = pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return address;
}
