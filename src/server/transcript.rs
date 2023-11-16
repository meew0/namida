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
#[no_mangle]
pub unsafe extern "C" fn xscript_close_server(
    mut session: *mut ttp_session_t,
    mut delta: u_int64_t,
) {
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    fprintf(
        (*xfer).transcript,
        b"mb_transmitted = %0.2f\n\0" as *const u8 as *const libc::c_char,
        (*param).file_size as libc::c_double / (1024.0f64 * 1024.0f64),
    );
    fprintf(
        (*xfer).transcript,
        b"duration = %0.2f\n\0" as *const u8 as *const libc::c_char,
        delta as libc::c_double / 1000000.0f64,
    );
    fprintf(
        (*xfer).transcript,
        b"throughput = %0.2f\n\0" as *const u8 as *const libc::c_char,
        (*param).file_size as libc::c_double * 8.0f64
            / (delta as libc::c_double * 1e-6f64 * 1024 as libc::c_int as libc::c_double
                * 1024 as libc::c_int as libc::c_double),
    );
    fclose((*xfer).transcript);
}
#[no_mangle]
pub unsafe extern "C" fn xscript_data_log_server(
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
pub unsafe extern "C" fn xscript_data_start_server(
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
pub unsafe extern "C" fn xscript_data_stop_server(
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
pub unsafe extern "C" fn xscript_open_server(mut session: *mut ttp_session_t) {
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut param: *mut ttp_parameter_t = (*session).parameter;
    let mut filename: [libc::c_char; 64] = [0; 64];
    make_transcript_filename(
        filename.as_mut_ptr(),
        (*param).epoch,
        b"tsus\0" as *const u8 as *const libc::c_char,
    );
    (*xfer)
        .transcript = fopen(
        filename.as_mut_ptr(),
        b"w\0" as *const u8 as *const libc::c_char,
    );
    if ((*xfer).transcript).is_null() {
        error_handler(
            b"transcript.c\0" as *const u8 as *const libc::c_char,
            141 as libc::c_int,
            b"Could not create transcript file\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return;
    }
    fprintf(
        (*xfer).transcript,
        b"filename = %s\n\0" as *const u8 as *const libc::c_char,
        (*xfer).filename,
    );
    fprintf(
        (*xfer).transcript,
        b"file_size = %llu\n\0" as *const u8 as *const libc::c_char,
        (*param).file_size as ull_t,
    );
    fprintf(
        (*xfer).transcript,
        b"block_count = %llu\n\0" as *const u8 as *const libc::c_char,
        (*param).block_count as ull_t,
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
        b"target_rate = %llu\n\0" as *const u8 as *const libc::c_char,
        (*param).target_rate as ull_t,
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
        b"ipd_time = %u\n\0" as *const u8 as *const libc::c_char,
        (*param).ipd_time,
    );
    fprintf(
        (*xfer).transcript,
        b"ipd_current = %u\n\0" as *const u8 as *const libc::c_char,
        (*xfer).ipd_current as u_int32_t,
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
    fprintf((*xfer).transcript, b"\n\0" as *const u8 as *const libc::c_char);
    fflush((*session).transfer.transcript);
}
