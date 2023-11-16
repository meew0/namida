use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fwrite(
        _: *const libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn fseeko(
        __stream: *mut FILE,
        __off: __off64_t,
        __whence: libc::c_int,
    ) -> libc::c_int;
    static mut g_error: [libc::c_char; 0];
    fn error_handler(
        file: *const libc::c_char,
        line: libc::c_int,
        message: *const libc::c_char,
        fatal_yn: libc::c_int,
    ) -> libc::c_int;
    fn read_vsib_block(
        session: *mut ttp_session_t,
        memblk: *mut libc::c_uchar,
        blksize: size_t,
    );
}
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __socklen_t = libc::c_uint;
pub type size_t = libc::c_ulong;
pub type u_char = __u_char;
pub type time_t = __time_t;
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
pub unsafe extern "C" fn build_datagram(
    mut session: *mut ttp_session_t,
    mut block_index: u_int32_t,
    mut block_type: u_int16_t,
    mut datagram: *mut u_char,
) -> libc::c_int {
    let mut block_size: u_int32_t = (*(*session).parameter).block_size;
    static mut last_block: u_int32_t = 0 as libc::c_int as u_int32_t;
    static mut last_written_vsib_block: u_int32_t = 0 as libc::c_int as u_int32_t;
    let mut status: libc::c_int = 0 as libc::c_int;
    let mut write_size: u_int32_t = 0;
    if 1 as libc::c_int as u_int32_t == block_index {
        last_written_vsib_block = 0 as libc::c_int as u_int32_t;
        last_block = 0 as libc::c_int as u_int32_t;
    }
    if block_index != last_block.wrapping_add(1 as libc::c_int as u_int32_t) {
        fseeko(
            (*session).transfer.vsib,
            ((*(*session).parameter).block_size as u_int64_t
                * block_index.wrapping_sub(1 as libc::c_int as u_int32_t) as u_int64_t)
                as __off64_t,
            0 as libc::c_int,
        );
    }
    read_vsib_block(
        session,
        datagram.offset(6 as libc::c_int as isize),
        (*(*session).parameter).block_size as size_t,
    );
    if (*(*session).parameter).fileout as libc::c_int != 0
        && (block_index != 0 as libc::c_int as u_int32_t) as libc::c_int
            & (block_index
                == last_written_vsib_block.wrapping_add(1 as libc::c_int as u_int32_t))
                as libc::c_int != 0
    {
        last_written_vsib_block = last_written_vsib_block.wrapping_add(1);
        last_written_vsib_block;
        write_size = (if block_index == (*(*session).parameter).block_count {
            (*(*session).parameter).file_size % block_size as u_int64_t
        } else {
            block_size as u_int64_t
        }) as u_int32_t;
        if write_size == 0 as libc::c_int as u_int32_t {
            write_size = block_size;
        }
        status = fwrite(
            datagram.offset(6 as libc::c_int as isize) as *const libc::c_void,
            1 as libc::c_int as libc::c_ulong,
            write_size as libc::c_ulong,
            (*session).transfer.file,
        ) as libc::c_int;
        if (status as u_int32_t) < write_size {
            sprintf(
                g_error.as_mut_ptr(),
                b"Could not write block %d of file\0" as *const u8
                    as *const libc::c_char,
                block_index,
            );
            return error_handler(
                b"io.c\0" as *const u8 as *const libc::c_char,
                181 as libc::c_int,
                g_error.as_mut_ptr(),
                0 as libc::c_int,
            );
        }
    }
    *(datagram.offset(0 as libc::c_int as isize)
        as *mut u_int32_t) = __bswap_32(block_index);
    *(datagram.offset(4 as libc::c_int as isize)
        as *mut u_int16_t) = __bswap_16(block_type);
    return 0 as libc::c_int;
}
