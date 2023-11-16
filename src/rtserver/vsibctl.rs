use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn snprintf(
        _: *mut libc::c_char,
        _: libc::c_ulong,
        _: *const libc::c_char,
        _: ...
    ) -> libc::c_int;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn fread(
        _: *mut libc::c_void,
        _: libc::c_ulong,
        _: libc::c_ulong,
        _: *mut FILE,
    ) -> libc::c_ulong;
    fn perror(__s: *const libc::c_char);
    fn fileno(__stream: *mut FILE) -> libc::c_int;
    fn abort() -> !;
    fn exit(_: libc::c_int) -> !;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn nanosleep(
        __requested_time: *const timespec,
        __remaining: *mut timespec,
    ) -> libc::c_int;
    fn shmctl(
        __shmid: libc::c_int,
        __cmd: libc::c_int,
        __buf: *mut shmid_ds,
    ) -> libc::c_int;
    fn shmget(__key: key_t, __size: size_t, __shmflg: libc::c_int) -> libc::c_int;
    fn shmat(
        __shmid: libc::c_int,
        __shmaddr: *const libc::c_void,
        __shmflg: libc::c_int,
    ) -> *mut libc::c_void;
    fn shmdt(__shmaddr: *const libc::c_void) -> libc::c_int;
    fn ioctl(__fd: libc::c_int, __request: libc::c_ulong, _: ...) -> libc::c_int;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint16_t = libc::c_ushort;
pub type __uint32_t = libc::c_uint;
pub type __uint64_t = libc::c_ulong;
pub type __uid_t = libc::c_uint;
pub type __gid_t = libc::c_uint;
pub type __mode_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __pid_t = libc::c_int;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
pub type __key_t = libc::c_int;
pub type __syscall_slong_t = libc::c_long;
pub type __syscall_ulong_t = libc::c_ulong;
pub type __socklen_t = libc::c_uint;
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
pub type u_char = __u_char;
pub type key_t = __key_t;
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
pub struct timespec {
    pub tv_sec: __time_t,
    pub tv_nsec: __syscall_slong_t,
}
pub type socklen_t = __socklen_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct ipc_perm {
    pub __key: __key_t,
    pub uid: __uid_t,
    pub gid: __gid_t,
    pub cuid: __uid_t,
    pub cgid: __gid_t,
    pub mode: __mode_t,
    pub __seq: libc::c_ushort,
    pub __pad2: libc::c_ushort,
    pub __glibc_reserved1: __syscall_ulong_t,
    pub __glibc_reserved2: __syscall_ulong_t,
}
pub type shmatt_t = __syscall_ulong_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct shmid_ds {
    pub shm_perm: ipc_perm,
    pub shm_segsz: size_t,
    pub shm_atime: __time_t,
    pub shm_dtime: __time_t,
    pub shm_ctime: __time_t,
    pub shm_cpid: __pid_t,
    pub shm_lpid: __pid_t,
    pub shm_nattch: shmatt_t,
    pub __glibc_reserved5: __syscall_ulong_t,
    pub __glibc_reserved6: __syscall_ulong_t,
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
pub type ptSh = *mut sSh;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct sSh {
    pub relSeekBlocks: libc::c_int,
}
pub type tSh = sSh;
#[no_mangle]
pub static mut vsib_mode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut vsib_mode_gigabit: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut vsib_mode_embed_1pps_markers: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut vsib_mode_skip_samples: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut readMode: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut vsib_started: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut usleeps: libc::c_int = 0 as libc::c_int;
#[no_mangle]
pub static mut shKey: key_t = 0;
#[no_mangle]
pub static mut shId: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub static mut sh: ptSh = 0 as *const sSh as *mut sSh;
#[no_mangle]
pub static mut vsib_fileno: libc::c_int = 0;
unsafe extern "C" fn vsib_ioctl(mut mode: libc::c_uint, mut arg: libc::c_ulong) {
    if ioctl(vsib_fileno, mode as libc::c_ulong, arg) != 0 {
        let mut err: [libc::c_char; 255] = [0; 255];
        snprintf(
            err.as_mut_ptr(),
            ::core::mem::size_of::<[libc::c_char; 255]>() as libc::c_ulong,
            b"Fatal VSIB access error: ioctl(vsib_fileno=0x%04x, 0x%04x, %ld) failed\0"
                as *const u8 as *const libc::c_char,
            vsib_fileno,
            mode,
            arg,
        );
        perror(err.as_mut_ptr());
        exit(1 as libc::c_int);
    }
}
#[no_mangle]
pub unsafe extern "C" fn tim() -> libc::c_double {
    let mut tv: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    let mut t: libc::c_double = 0.0f64;
    if gettimeofday(&mut tv, 0 as *mut libc::c_void) == 0 as libc::c_int {
        t = tv.tv_sec as libc::c_double + tv.tv_usec as libc::c_double / 1000000.0f64;
    }
    return t;
}
#[no_mangle]
pub unsafe extern "C" fn start_vsib(mut session: *mut ttp_session_t) {
    let mut xfer: *mut ttp_transfer_t = &mut (*session).transfer;
    vsib_fileno = fileno((*xfer).vsib);
    shKey = (('v' as i32 as libc::c_long) << 24 as libc::c_int
        | ('s' as i32 as libc::c_long) << 16 as libc::c_int
        | ('i' as i32 as libc::c_long) << 8 as libc::c_int | 'b' as i32 as libc::c_long)
        as key_t;
    shId = shmget(
        shKey,
        ::core::mem::size_of::<tSh>() as libc::c_ulong,
        0o1000 as libc::c_int | 0o777 as libc::c_int,
    );
    if shId == -(1 as libc::c_int) {
        fprintf(
            stderr,
            b"Fatal VSIB access error: could not shmget() shared memory!\nPrevious instance of Tsunami was probably run as 'root' and interrupted. Reload 'vsib' kernel module.\n\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    sh = shmat(shId, 0 as *const libc::c_void, 0 as libc::c_int) as ptSh;
    if sh == -(1 as libc::c_int) as *mut libc::c_void as ptSh {
        fprintf(
            stderr,
            b"Fatal VSIB access error: could not shmat() shared memory, seeking not possible, exiting!\n\0"
                as *const u8 as *const libc::c_char,
        );
        abort();
    }
    (*sh).relSeekBlocks = 0 as libc::c_int;
    vsib_ioctl(
        0x7801 as libc::c_int as libc::c_uint,
        (((vsib_mode & 0xf as libc::c_int) << 16 as libc::c_int) as libc::c_uint
            | 0x80000000 as libc::c_uint
            | (if vsib_mode_gigabit != 0 {
                0x40000000 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint
            | (if vsib_mode_embed_1pps_markers != 0 {
                0x20000000 as libc::c_int
            } else {
                0 as libc::c_int
            }) as libc::c_uint
            | (vsib_mode_skip_samples & 0xffff as libc::c_int) as libc::c_uint)
            as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn read_vsib_block(
    mut session: *mut ttp_session_t,
    mut memblk: *mut libc::c_uchar,
    mut blksize: size_t,
) {
    let mut nread: size_t = 0;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    ts.tv_sec = 0 as libc::c_int as __time_t;
    ts.tv_nsec = 1000000 as libc::c_long;
    nread = fread(
        memblk as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        blksize,
        (*session).transfer.vsib,
    );
    while nread < blksize {
        nanosleep(&mut ts, 0 as *mut timespec);
        nread = (nread as libc::c_ulong)
            .wrapping_add(
                fread(
                    memblk.offset(nread as isize) as *mut libc::c_void,
                    1 as libc::c_int as libc::c_ulong,
                    blksize.wrapping_sub(nread),
                    (*session).transfer.vsib,
                ),
            ) as size_t as size_t;
    }
}
#[no_mangle]
pub unsafe extern "C" fn stop_vsib(mut session: *mut ttp_session_t) {
    let mut timeout: libc::c_int = 0 as libc::c_int;
    let mut ts: timespec = timespec { tv_sec: 0, tv_nsec: 0 };
    ts.tv_sec = 0 as libc::c_int as __time_t;
    ts.tv_nsec = 100000000 as libc::c_long;
    vsib_ioctl(0x7806 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_ulong);
    let mut b: libc::c_ulong = 0;
    vsib_ioctl(
        0x7807 as libc::c_int as libc::c_uint,
        &mut b as *mut libc::c_ulong as libc::c_ulong,
    );
    while b == 0
        && {
            let fresh0 = timeout;
            timeout = timeout + 1;
            fresh0 < 25 as libc::c_int
        }
    {
        fprintf(
            stderr,
            b"Waiting for last DMA descriptor (sl=%d)\n\0" as *const u8
                as *const libc::c_char,
            usleeps,
        );
        nanosleep(&mut ts, 0 as *mut timespec);
        vsib_ioctl(
            0x7807 as libc::c_int as libc::c_uint,
            &mut b as *mut libc::c_ulong as libc::c_ulong,
        );
    }
    vsib_ioctl(0x7801 as libc::c_int as libc::c_uint, 0 as libc::c_int as libc::c_ulong);
    if shId != -(1 as libc::c_int) && sh != -(1 as libc::c_int) as ptSh && !sh.is_null()
    {
        if shmctl(shId, 0 as libc::c_int, 0 as *mut shmid_ds) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"VSIB access error: shared memory ID remove shmctl() returned non-0\n\0"
                    as *const u8 as *const libc::c_char,
            );
        } else if shmdt(sh as *const libc::c_void) != 0 as libc::c_int {
            fprintf(
                stderr,
                b"VSIB access error: shared memory data remove shmdt() returned non-0\n\0"
                    as *const u8 as *const libc::c_char,
            );
        }
    }
}
