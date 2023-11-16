use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fclose(__stream: *mut FILE) -> libc::c_int;
    fn fflush(__stream: *mut FILE) -> libc::c_int;
    fn fopen(_: *const libc::c_char, _: *const libc::c_char) -> *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    static PROTOCOL_REVISION: u_int32_t;
    fn make_transcript_filename(
        buffer: *mut libc::c_char,
        epoch: time_t,
        extension: *const libc::c_char,
    ) -> *mut libc::c_char;
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
#[no_mangle]
pub unsafe extern "C" fn xscript_close_client(
    mut session: *mut ttp_session_t,
    mut delta: u_int64_t,
) {
    let mut mb_thru: libc::c_double = 0.;
    let mut mb_good: libc::c_double = 0.;
    let mut mb_file: libc::c_double = 0.;
    let mut secs: libc::c_double = 0.;
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    mb_thru = ((*xfer).stats.total_blocks * (*(*session).parameter).block_size) as libc::c_double;
    mb_good = mb_thru
        - ((*xfer).stats.total_recvd_retransmits * (*(*session).parameter).block_size)
            as libc::c_double;
    mb_file = (*xfer).file_size as libc::c_double;
    mb_thru /= 1024.0f64 * 1024.0f64;
    mb_good /= 1024.0f64 * 1024.0f64;
    mb_file /= 1024.0f64 * 1024.0f64;
    secs = delta as libc::c_double / 1e6f64;
    fprintf(
        (*xfer).transcript,
        b"mbyte_transmitted = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_thru,
    );
    fprintf(
        (*xfer).transcript,
        b"mbyte_usable = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_good,
    );
    fprintf(
        (*xfer).transcript,
        b"mbyte_file = %0.2f\n\0" as *const u8 as *const libc::c_char,
        mb_file,
    );
    fprintf(
        (*xfer).transcript,
        b"duration = %0.2f\n\0" as *const u8 as *const libc::c_char,
        secs,
    );
    fprintf(
        (*xfer).transcript,
        b"throughput = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_thru / secs,
    );
    fprintf(
        (*xfer).transcript,
        b"goodput_with_restarts = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_good / secs,
    );
    fprintf(
        (*xfer).transcript,
        b"file_rate = %0.2f\n\0" as *const u8 as *const libc::c_char,
        8.0f64 * mb_file / secs,
    );
    fclose((*xfer).transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_log_client(
    mut session: *mut ttp_session_t,
    mut logline: *const libc::c_char,
) {
    fprintf(
        (*session).transfer.transcript,
        b"%s\0" as *const u8 as *const libc::c_char,
        logline,
    );
    fflush((*session).transfer.transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_start_client(
    mut session: *mut ttp_session_t,
    mut epoch: *const timeval,
) {
    fprintf(
        (*session).transfer.transcript,
        b"START %lu.%06lu\n\0" as *const u8 as *const libc::c_char,
        (*epoch).tv_sec as libc::c_ulong,
        (*epoch).tv_usec as libc::c_ulong,
    );
    fflush((*session).transfer.transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_stop_client(
    mut session: *mut ttp_session_t,
    mut epoch: *const timeval,
) {
    fprintf(
        (*session).transfer.transcript,
        b"STOP %lu.%06lu\n\n\0" as *const u8 as *const libc::c_char,
        (*epoch).tv_sec as libc::c_ulong,
        (*epoch).tv_usec as libc::c_ulong,
    );
    fflush((*session).transfer.transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_open_client(mut session: *mut ttp_session_t) {
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    let mut filename: [libc::c_char; 64] = [0; 64];
    make_transcript_filename(
        filename.as_mut_ptr(),
        (*xfer).epoch,
        b"tsuc\0" as *const u8 as *const libc::c_char,
    );
    (*xfer).transcript = fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if ((*xfer).transcript).is_null() {
        error_handler(
            b"transcript.c\0" as *const u8 as *const libc::c_char,
            153 as libc::c_int,
            b"Could not create transcript file\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return;
    }
    fprintf(
        (*xfer).transcript,
        b"remote_filename = %s\n\0" as *const u8 as *const libc::c_char,
        (*xfer).remote_filename,
    );
    fprintf(
        (*xfer).transcript,
        b"local_filename = %s\n\0" as *const u8 as *const libc::c_char,
        (*xfer).local_filename,
    );
    fprintf(
        (*xfer).transcript,
        b"file_size = %llu\n\0" as *const u8 as *const libc::c_char,
        (*xfer).file_size as ull_t,
    );
    fprintf(
        (*xfer).transcript,
        b"block_count = %u\n\0" as *const u8 as *const libc::c_char,
        (*xfer).block_count,
    );
    fprintf(
        (*xfer).transcript,
        b"udp_buffer = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).udp_buffer,
    );
    fprintf(
        (*xfer).transcript,
        b"block_size = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).block_size,
    );
    fprintf(
        (*xfer).transcript,
        b"target_rate = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).target_rate,
    );
    fprintf(
        (*xfer).transcript,
        b"error_rate = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).error_rate,
    );
    fprintf(
        (*xfer).transcript,
        b"slower_num = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).slower_num as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"slower_den = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).slower_den as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"faster_num = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).faster_num as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"faster_den = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).faster_den as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"history = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).history as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"lossless = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).lossless as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"losswindow = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).losswindow_ms,
    );
    fprintf(
        (*xfer).transcript,
        b"blockdump = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).blockdump as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"update_period = %llu\n\0" as *const u8 as *const libc::c_char,
        350000 as libc::c_longlong,
    );
    fprintf(
        (*xfer).transcript,
        b"rexmit_period = %llu\n\0" as *const u8 as *const libc::c_char,
        350000 as libc::c_longlong,
    );
    fprintf(
        (*xfer).transcript,
        b"protocol_version = 0x%x\n\0" as *const u8 as *const libc::c_char,
        PROTOCOL_REVISION,
    );
    fprintf(
        (*xfer).transcript,
        b"software_version = %s\n\0" as *const u8 as *const libc::c_char,
        b"v1.1 devel cvsbuild 43\0" as *const u8 as *const libc::c_char,
    );
    fprintf(
        (*xfer).transcript,
        b"ipv6 = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).ipv6_yn as libc::c_int,
    );
    fprintf(
        (*xfer).transcript,
        b"\n\0" as *const u8 as *const libc::c_char,
    );
    fflush((*session).transfer.transcript);
}
