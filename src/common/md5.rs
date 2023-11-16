use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn sprintf(_: *mut libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn memcpy(
        _: *mut libc::c_void,
        _: *const libc::c_void,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
}
pub type size_t = libc::c_ulong;
pub type __u_char = libc::c_uchar;
pub type __uint32_t = libc::c_uint;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
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
pub type u_int32_t = __uint32_t;
pub type md5_byte_t = u_char;
pub type md5_word_t = u_int32_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct md5_state_s {
    pub count: [md5_word_t; 2],
    pub abcd: [md5_word_t; 4],
    pub buf: [md5_byte_t; 64],
}
pub type md5_state_t = md5_state_s;
#[no_mangle]
pub static mut T: [u_int32_t; 64] = [
    0xd76aa478 as libc::c_uint,
    0xe8c7b756 as libc::c_uint,
    0x242070db as libc::c_int as u_int32_t,
    0xc1bdceee as libc::c_uint,
    0xf57c0faf as libc::c_uint,
    0x4787c62a as libc::c_int as u_int32_t,
    0xa8304613 as libc::c_uint,
    0xfd469501 as libc::c_uint,
    0x698098d8 as libc::c_int as u_int32_t,
    0x8b44f7af as libc::c_uint,
    0xffff5bb1 as libc::c_uint,
    0x895cd7be as libc::c_uint,
    0x6b901122 as libc::c_int as u_int32_t,
    0xfd987193 as libc::c_uint,
    0xa679438e as libc::c_uint,
    0x49b40821 as libc::c_int as u_int32_t,
    0xf61e2562 as libc::c_uint,
    0xc040b340 as libc::c_uint,
    0x265e5a51 as libc::c_int as u_int32_t,
    0xe9b6c7aa as libc::c_uint,
    0xd62f105d as libc::c_uint,
    0x2441453 as libc::c_int as u_int32_t,
    0xd8a1e681 as libc::c_uint,
    0xe7d3fbc8 as libc::c_uint,
    0x21e1cde6 as libc::c_int as u_int32_t,
    0xc33707d6 as libc::c_uint,
    0xf4d50d87 as libc::c_uint,
    0x455a14ed as libc::c_int as u_int32_t,
    0xa9e3e905 as libc::c_uint,
    0xfcefa3f8 as libc::c_uint,
    0x676f02d9 as libc::c_int as u_int32_t,
    0x8d2a4c8a as libc::c_uint,
    0xfffa3942 as libc::c_uint,
    0x8771f681 as libc::c_uint,
    0x6d9d6122 as libc::c_int as u_int32_t,
    0xfde5380c as libc::c_uint,
    0xa4beea44 as libc::c_uint,
    0x4bdecfa9 as libc::c_int as u_int32_t,
    0xf6bb4b60 as libc::c_uint,
    0xbebfbc70 as libc::c_uint,
    0x289b7ec6 as libc::c_int as u_int32_t,
    0xeaa127fa as libc::c_uint,
    0xd4ef3085 as libc::c_uint,
    0x4881d05 as libc::c_int as u_int32_t,
    0xd9d4d039 as libc::c_uint,
    0xe6db99e5 as libc::c_uint,
    0x1fa27cf8 as libc::c_int as u_int32_t,
    0xc4ac5665 as libc::c_uint,
    0xf4292244 as libc::c_uint,
    0x432aff97 as libc::c_int as u_int32_t,
    0xab9423a7 as libc::c_uint,
    0xfc93a039 as libc::c_uint,
    0x655b59c3 as libc::c_int as u_int32_t,
    0x8f0ccc92 as libc::c_uint,
    0xffeff47d as libc::c_uint,
    0x85845dd1 as libc::c_uint,
    0x6fa87e4f as libc::c_int as u_int32_t,
    0xfe2ce6e0 as libc::c_uint,
    0xa3014314 as libc::c_uint,
    0x4e0811a1 as libc::c_int as u_int32_t,
    0xf7537e82 as libc::c_uint,
    0xbd3af235 as libc::c_uint,
    0x2ad7d2bb as libc::c_int as u_int32_t,
    0xeb86d391 as libc::c_uint,
];
#[no_mangle]
pub static mut pad: [u_char; 64] = [
    0x80 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0 as libc::c_int as u_char,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
    0,
];
#[no_mangle]
pub static mut md5_table: [[u_char; 2]; 64] = [
    [0 as libc::c_int as u_char, 7 as libc::c_int as u_char],
    [1 as libc::c_int as u_char, 12 as libc::c_int as u_char],
    [2 as libc::c_int as u_char, 17 as libc::c_int as u_char],
    [3 as libc::c_int as u_char, 22 as libc::c_int as u_char],
    [4 as libc::c_int as u_char, 7 as libc::c_int as u_char],
    [5 as libc::c_int as u_char, 12 as libc::c_int as u_char],
    [6 as libc::c_int as u_char, 17 as libc::c_int as u_char],
    [7 as libc::c_int as u_char, 22 as libc::c_int as u_char],
    [8 as libc::c_int as u_char, 7 as libc::c_int as u_char],
    [9 as libc::c_int as u_char, 12 as libc::c_int as u_char],
    [10 as libc::c_int as u_char, 17 as libc::c_int as u_char],
    [11 as libc::c_int as u_char, 22 as libc::c_int as u_char],
    [12 as libc::c_int as u_char, 7 as libc::c_int as u_char],
    [13 as libc::c_int as u_char, 12 as libc::c_int as u_char],
    [14 as libc::c_int as u_char, 17 as libc::c_int as u_char],
    [15 as libc::c_int as u_char, 22 as libc::c_int as u_char],
    [1 as libc::c_int as u_char, 5 as libc::c_int as u_char],
    [6 as libc::c_int as u_char, 9 as libc::c_int as u_char],
    [11 as libc::c_int as u_char, 14 as libc::c_int as u_char],
    [0 as libc::c_int as u_char, 20 as libc::c_int as u_char],
    [5 as libc::c_int as u_char, 5 as libc::c_int as u_char],
    [10 as libc::c_int as u_char, 9 as libc::c_int as u_char],
    [15 as libc::c_int as u_char, 14 as libc::c_int as u_char],
    [4 as libc::c_int as u_char, 20 as libc::c_int as u_char],
    [9 as libc::c_int as u_char, 5 as libc::c_int as u_char],
    [14 as libc::c_int as u_char, 9 as libc::c_int as u_char],
    [3 as libc::c_int as u_char, 14 as libc::c_int as u_char],
    [8 as libc::c_int as u_char, 20 as libc::c_int as u_char],
    [13 as libc::c_int as u_char, 5 as libc::c_int as u_char],
    [2 as libc::c_int as u_char, 9 as libc::c_int as u_char],
    [7 as libc::c_int as u_char, 14 as libc::c_int as u_char],
    [12 as libc::c_int as u_char, 20 as libc::c_int as u_char],
    [5 as libc::c_int as u_char, 4 as libc::c_int as u_char],
    [8 as libc::c_int as u_char, 11 as libc::c_int as u_char],
    [11 as libc::c_int as u_char, 16 as libc::c_int as u_char],
    [14 as libc::c_int as u_char, 23 as libc::c_int as u_char],
    [1 as libc::c_int as u_char, 4 as libc::c_int as u_char],
    [4 as libc::c_int as u_char, 11 as libc::c_int as u_char],
    [7 as libc::c_int as u_char, 16 as libc::c_int as u_char],
    [10 as libc::c_int as u_char, 23 as libc::c_int as u_char],
    [13 as libc::c_int as u_char, 4 as libc::c_int as u_char],
    [0 as libc::c_int as u_char, 11 as libc::c_int as u_char],
    [3 as libc::c_int as u_char, 16 as libc::c_int as u_char],
    [6 as libc::c_int as u_char, 23 as libc::c_int as u_char],
    [9 as libc::c_int as u_char, 4 as libc::c_int as u_char],
    [12 as libc::c_int as u_char, 11 as libc::c_int as u_char],
    [15 as libc::c_int as u_char, 16 as libc::c_int as u_char],
    [2 as libc::c_int as u_char, 23 as libc::c_int as u_char],
    [0 as libc::c_int as u_char, 6 as libc::c_int as u_char],
    [7 as libc::c_int as u_char, 10 as libc::c_int as u_char],
    [14 as libc::c_int as u_char, 15 as libc::c_int as u_char],
    [5 as libc::c_int as u_char, 21 as libc::c_int as u_char],
    [12 as libc::c_int as u_char, 6 as libc::c_int as u_char],
    [3 as libc::c_int as u_char, 10 as libc::c_int as u_char],
    [10 as libc::c_int as u_char, 15 as libc::c_int as u_char],
    [1 as libc::c_int as u_char, 21 as libc::c_int as u_char],
    [8 as libc::c_int as u_char, 6 as libc::c_int as u_char],
    [15 as libc::c_int as u_char, 10 as libc::c_int as u_char],
    [6 as libc::c_int as u_char, 15 as libc::c_int as u_char],
    [13 as libc::c_int as u_char, 21 as libc::c_int as u_char],
    [4 as libc::c_int as u_char, 6 as libc::c_int as u_char],
    [11 as libc::c_int as u_char, 10 as libc::c_int as u_char],
    [2 as libc::c_int as u_char, 15 as libc::c_int as u_char],
    [9 as libc::c_int as u_char, 21 as libc::c_int as u_char],
];
#[no_mangle]
pub unsafe extern "C" fn md5_F(
    mut x: u_int32_t,
    mut y: u_int32_t,
    mut z: u_int32_t,
) -> u_int32_t {
    return x & y | !x & z;
}
#[no_mangle]
pub unsafe extern "C" fn md5_G(
    mut x: u_int32_t,
    mut y: u_int32_t,
    mut z: u_int32_t,
) -> u_int32_t {
    return x & z | y & !z;
}
#[no_mangle]
pub unsafe extern "C" fn md5_H(
    mut x: u_int32_t,
    mut y: u_int32_t,
    mut z: u_int32_t,
) -> u_int32_t {
    return x ^ y ^ z;
}
#[no_mangle]
pub unsafe extern "C" fn md5_I(
    mut x: u_int32_t,
    mut y: u_int32_t,
    mut z: u_int32_t,
) -> u_int32_t {
    return y ^ (x | !z);
}
#[no_mangle]
pub static mut md5_dispatch: [Option::<
    unsafe extern "C" fn(u_int32_t, u_int32_t, u_int32_t) -> u_int32_t,
>; 4] = unsafe {
    [
        Some(
            md5_F as unsafe extern "C" fn(u_int32_t, u_int32_t, u_int32_t) -> u_int32_t,
        ),
        Some(
            md5_G as unsafe extern "C" fn(u_int32_t, u_int32_t, u_int32_t) -> u_int32_t,
        ),
        Some(
            md5_H as unsafe extern "C" fn(u_int32_t, u_int32_t, u_int32_t) -> u_int32_t,
        ),
        Some(md5_I as unsafe extern "C" fn(u_int32_t, u_int32_t, u_int32_t) -> u_int32_t),
    ]
};
#[no_mangle]
pub unsafe extern "C" fn md5_digest(
    mut buffer: *mut u_char,
    mut size: size_t,
    mut digest: *mut u_char,
) {
    let mut X: [u_int32_t; 16] = [0; 16];
    let mut state: [u_int32_t; 4] = [0; 4];
    let mut tempState: [u_int32_t; 4] = [0; 4];
    let mut func: u_int32_t = 0;
    let mut sum: u_int32_t = 0;
    let mut i: libc::c_int = 0;
    let mut blocks: libc::c_int = 0;
    let mut j: libc::c_int = 0;
    state[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as u_int32_t;
    state[1 as libc::c_int as usize] = 0xefcdab89 as libc::c_uint;
    state[2 as libc::c_int as usize] = 0x98badcfe as libc::c_uint;
    state[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as u_int32_t;
    blocks = (size / 64 as libc::c_int as size_t) as libc::c_int;
    if size % 64 as libc::c_int as size_t > 56 as libc::c_int as size_t {
        blocks += 1;
        blocks;
    }
    i = 0 as libc::c_int;
    while i <= blocks {
        if i < blocks - 1 as libc::c_int {
            memcpy(
                X.as_mut_ptr() as *mut u_char as *mut libc::c_void,
                buffer.offset((64 as libc::c_int * i) as isize) as *const libc::c_void,
                64 as libc::c_int as libc::c_ulong,
            );
        } else if i == blocks - 1 as libc::c_int {
            if (64 as libc::c_int * (i + 1 as libc::c_int)) as size_t > size {
                memcpy(
                    X.as_mut_ptr() as *mut u_char as *mut libc::c_void,
                    buffer.offset((64 as libc::c_int * i) as isize)
                        as *const libc::c_void,
                    size % 64 as libc::c_int as size_t,
                );
                memcpy(
                    (X.as_mut_ptr() as *mut u_char)
                        .offset((size % 64 as libc::c_int as size_t) as isize)
                        as *mut libc::c_void,
                    pad.as_mut_ptr() as *const libc::c_void,
                    (64 as libc::c_int as size_t)
                        .wrapping_sub(size % 64 as libc::c_int as size_t),
                );
            } else {
                memcpy(
                    X.as_mut_ptr() as *mut u_char as *mut libc::c_void,
                    buffer.offset((64 as libc::c_int * i) as isize)
                        as *const libc::c_void,
                    64 as libc::c_int as libc::c_ulong,
                );
            }
        } else {
            if (64 as libc::c_int * i) as size_t > size {
                memset(
                    X.as_mut_ptr() as *mut u_char as *mut libc::c_void,
                    0 as libc::c_int,
                    64 as libc::c_int as libc::c_ulong,
                );
            } else {
                memcpy(
                    X.as_mut_ptr() as *mut u_char as *mut libc::c_void,
                    buffer.offset((64 as libc::c_int * i) as isize)
                        as *const libc::c_void,
                    size % 64 as libc::c_int as size_t,
                );
                memcpy(
                    (X.as_mut_ptr() as *mut u_char)
                        .offset((size % 64 as libc::c_int as size_t) as isize)
                        as *mut libc::c_void,
                    pad.as_mut_ptr() as *const libc::c_void,
                    (64 as libc::c_int as size_t)
                        .wrapping_sub(size % 64 as libc::c_int as size_t),
                );
            }
            X[14 as libc::c_int
                as usize] = (size * 8 as libc::c_int as size_t) as u_int32_t;
            X[15 as libc::c_int as usize] = 0 as libc::c_int as u_int32_t;
        }
        memcpy(
            tempState.as_mut_ptr() as *mut u_char as *mut libc::c_void,
            state.as_mut_ptr() as *mut u_char as *const libc::c_void,
            16 as libc::c_int as libc::c_ulong,
        );
        j = 0 as libc::c_int;
        while j < 64 as libc::c_int {
            func = (md5_dispatch[(j / 16 as libc::c_int) as usize])
                .expect(
                    "non-null function pointer",
                )(
                state[((4 as libc::c_int - j % 4 as libc::c_int + 1 as libc::c_int)
                    % 4 as libc::c_int) as usize],
                state[((4 as libc::c_int - j % 4 as libc::c_int + 2 as libc::c_int)
                    % 4 as libc::c_int) as usize],
                state[((4 as libc::c_int - j % 4 as libc::c_int + 3 as libc::c_int)
                    % 4 as libc::c_int) as usize],
            );
            sum = (state[((4 as libc::c_int - j % 4 as libc::c_int) % 4 as libc::c_int)
                as usize])
                .wrapping_add(func)
                .wrapping_add(
                    X[md5_table[j as usize][0 as libc::c_int as usize] as usize],
                )
                .wrapping_add(T[j as usize]);
            state[((4 as libc::c_int - j % 4 as libc::c_int) % 4 as libc::c_int)
                as usize] = (state[((4 as libc::c_int - j % 4 as libc::c_int
                + 1 as libc::c_int) % 4 as libc::c_int) as usize])
                .wrapping_add(
                    sum
                        << md5_table[j as usize][1 as libc::c_int as usize]
                            as libc::c_int
                        | sum
                            >> 32 as libc::c_int
                                - md5_table[j as usize][1 as libc::c_int as usize]
                                    as libc::c_int,
                );
            j += 1;
            j;
        }
        j = 0 as libc::c_int;
        while j < 4 as libc::c_int {
            state[j as usize] = (state[j as usize]).wrapping_add(tempState[j as usize]);
            j += 1;
            j;
        }
        i += 1;
        i;
    }
    memcpy(
        digest as *mut libc::c_void,
        state.as_mut_ptr() as *mut u_char as *const libc::c_void,
        16 as libc::c_int as libc::c_ulong,
    );
}
#[no_mangle]
pub unsafe extern "C" fn md5_fprint_digest(
    mut file: *mut FILE,
    mut digest: *mut u_char,
) {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        fprintf(
            file,
            b"%02x\0" as *const u8 as *const libc::c_char,
            *digest.offset(i as isize) as libc::c_int,
        );
        i += 1;
        i;
    }
}
#[no_mangle]
pub unsafe extern "C" fn md5_sprint_digest(
    mut buffer: *mut libc::c_char,
    mut digest: *mut u_char,
) {
    sprintf(
        buffer,
        b"%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x%02x\0"
            as *const u8 as *const libc::c_char,
        *digest.offset(0 as libc::c_int as isize) as libc::c_int,
        *digest.offset(1 as libc::c_int as isize) as libc::c_int,
        *digest.offset(2 as libc::c_int as isize) as libc::c_int,
        *digest.offset(3 as libc::c_int as isize) as libc::c_int,
        *digest.offset(4 as libc::c_int as isize) as libc::c_int,
        *digest.offset(5 as libc::c_int as isize) as libc::c_int,
        *digest.offset(6 as libc::c_int as isize) as libc::c_int,
        *digest.offset(7 as libc::c_int as isize) as libc::c_int,
        *digest.offset(8 as libc::c_int as isize) as libc::c_int,
        *digest.offset(9 as libc::c_int as isize) as libc::c_int,
        *digest.offset(10 as libc::c_int as isize) as libc::c_int,
        *digest.offset(11 as libc::c_int as isize) as libc::c_int,
        *digest.offset(12 as libc::c_int as isize) as libc::c_int,
        *digest.offset(13 as libc::c_int as isize) as libc::c_int,
        *digest.offset(14 as libc::c_int as isize) as libc::c_int,
        *digest.offset(15 as libc::c_int as isize) as libc::c_int,
    );
}
unsafe extern "C" fn md5_process(
    mut pms: *mut md5_state_t,
    mut data: *const md5_byte_t,
) {
    let mut a: md5_word_t = (*pms).abcd[0 as libc::c_int as usize];
    let mut b: md5_word_t = (*pms).abcd[1 as libc::c_int as usize];
    let mut c: md5_word_t = (*pms).abcd[2 as libc::c_int as usize];
    let mut d: md5_word_t = (*pms).abcd[3 as libc::c_int as usize];
    let mut t: md5_word_t = 0;
    let mut xbuf: [md5_word_t; 16] = [0; 16];
    let mut X: *const md5_word_t = 0 as *const md5_word_t;
    static mut w: libc::c_int = 1 as libc::c_int;
    if *(&w as *const libc::c_int as *const md5_byte_t) != 0 {
        if data.offset_from(0 as *const md5_byte_t) as libc::c_long
            & 3 as libc::c_int as libc::c_long == 0
        {
            X = data as *const md5_word_t;
        } else {
            memcpy(
                xbuf.as_mut_ptr() as *mut libc::c_void,
                data as *const libc::c_void,
                64 as libc::c_int as libc::c_ulong,
            );
            X = xbuf.as_mut_ptr();
        }
    } else {
        let mut xp: *const md5_byte_t = data;
        let mut i: libc::c_int = 0;
        X = xbuf.as_mut_ptr();
        i = 0 as libc::c_int;
        while i < 16 as libc::c_int {
            xbuf[i
                as usize] = (*xp.offset(0 as libc::c_int as isize) as libc::c_int
                + ((*xp.offset(1 as libc::c_int as isize) as libc::c_int)
                    << 8 as libc::c_int)
                + ((*xp.offset(2 as libc::c_int as isize) as libc::c_int)
                    << 16 as libc::c_int)
                + ((*xp.offset(3 as libc::c_int as isize) as libc::c_int)
                    << 24 as libc::c_int)) as md5_word_t;
            i += 1;
            i;
            xp = xp.offset(4 as libc::c_int as isize);
        }
    }
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x28955b87 as libc::c_int as md5_word_t,
        );
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x173848a9 as libc::c_int as md5_word_t,
        );
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(0x242070db as libc::c_int as md5_word_t);
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3e423111 as libc::c_int as md5_word_t,
        );
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xa83f050 as libc::c_int as md5_word_t,
        );
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(0x4787c62a as libc::c_int as md5_word_t);
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x57cfb9ec as libc::c_int as md5_word_t,
        );
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x2b96afe as libc::c_int as md5_word_t,
        );
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(0x698098d8 as libc::c_int as md5_word_t);
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x74bb0850 as libc::c_int as md5_word_t,
        );
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xa44e as libc::c_int as md5_word_t,
        );
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x76a32841 as libc::c_int as md5_word_t,
        );
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & c | !b & d)
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(0x6b901122 as libc::c_int as md5_word_t);
    a = (t << 7 as libc::c_int | t >> 32 as libc::c_int - 7 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & b | !a & c)
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x2678e6c as libc::c_int as md5_word_t,
        );
    d = (t << 12 as libc::c_int | t >> 32 as libc::c_int - 12 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & a | !d & b)
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5986bc71 as libc::c_int as md5_word_t,
        );
    c = (t << 17 as libc::c_int | t >> 32 as libc::c_int - 17 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & d | !c & a)
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(0x49b40821 as libc::c_int as md5_word_t);
    b = (t << 22 as libc::c_int | t >> 32 as libc::c_int - 22 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x9e1da9d as libc::c_int as md5_word_t,
        );
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3fbf4cbf as libc::c_int as md5_word_t,
        );
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(0x265e5a51 as libc::c_int as md5_word_t);
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x16493855 as libc::c_int as md5_word_t,
        );
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x29d0efa2 as libc::c_int as md5_word_t,
        );
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(0x2441453 as libc::c_int as md5_word_t);
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x275e197e as libc::c_int as md5_word_t,
        );
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x182c0437 as libc::c_int as md5_word_t,
        );
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(0x21e1cde6 as libc::c_int as md5_word_t);
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3cc8f829 as libc::c_int as md5_word_t,
        );
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xb2af278 as libc::c_int as md5_word_t,
        );
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(0x455a14ed as libc::c_int as md5_word_t);
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b & d | c & !d)
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x561c16fa as libc::c_int as md5_word_t,
        );
    a = (t << 5 as libc::c_int | t >> 32 as libc::c_int - 5 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a & c | b & !c)
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3105c07 as libc::c_int as md5_word_t,
        );
    d = (t << 9 as libc::c_int | t >> 32 as libc::c_int - 9 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d & b | a & !b)
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(0x676f02d9 as libc::c_int as md5_word_t);
    c = (t << 14 as libc::c_int | t >> 32 as libc::c_int - 14 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c & a | d & !a)
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x72d5b375 as libc::c_int as md5_word_t,
        );
    b = (t << 20 as libc::c_int | t >> 32 as libc::c_int - 20 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5c6bd as libc::c_int as md5_word_t,
        );
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x788e097e as libc::c_int as md5_word_t,
        );
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(0x6d9d6122 as libc::c_int as md5_word_t);
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x21ac7f3 as libc::c_int as md5_word_t,
        );
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5b4115bb as libc::c_int as md5_word_t,
        );
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(0x4bdecfa9 as libc::c_int as md5_word_t);
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x944b49f as libc::c_int as md5_word_t,
        );
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x4140438f as libc::c_int as md5_word_t,
        );
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(0x289b7ec6 as libc::c_int as md5_word_t);
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x155ed805 as libc::c_int as md5_word_t,
        );
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x2b10cf7a as libc::c_int as md5_word_t,
        );
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(0x4881d05 as libc::c_int as md5_word_t);
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(b ^ c ^ d)
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x262b2fc6 as libc::c_int as md5_word_t,
        );
    a = (t << 4 as libc::c_int | t >> 32 as libc::c_int - 4 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(a ^ b ^ c)
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x1924661a as libc::c_int as md5_word_t,
        );
    d = (t << 11 as libc::c_int | t >> 32 as libc::c_int - 11 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(d ^ a ^ b)
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(0x1fa27cf8 as libc::c_int as md5_word_t);
    c = (t << 16 as libc::c_int | t >> 32 as libc::c_int - 16 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(c ^ d ^ a)
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x3b53a99a as libc::c_int as md5_word_t,
        );
    b = (t << 23 as libc::c_int | t >> 32 as libc::c_int - 23 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(0 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0xbd6ddbb as libc::c_int as md5_word_t,
        );
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(7 as libc::c_int as isize))
        .wrapping_add(0x432aff97 as libc::c_int as md5_word_t);
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(14 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x546bdc58 as libc::c_int as md5_word_t,
        );
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(5 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x36c5fc6 as libc::c_int as md5_word_t,
        );
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(12 as libc::c_int as isize))
        .wrapping_add(0x655b59c3 as libc::c_int as md5_word_t);
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(3 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x70f3336d as libc::c_int as md5_word_t,
        );
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(10 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x100b82 as libc::c_int as md5_word_t,
        );
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(1 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x7a7ba22e as libc::c_int as md5_word_t,
        );
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(8 as libc::c_int as isize))
        .wrapping_add(0x6fa87e4f as libc::c_int as md5_word_t);
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(15 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x1d3191f as libc::c_int as md5_word_t,
        );
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(6 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x5cfebceb as libc::c_int as md5_word_t,
        );
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(13 as libc::c_int as isize))
        .wrapping_add(0x4e0811a1 as libc::c_int as md5_word_t);
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    t = a
        .wrapping_add(c ^ (b | !d))
        .wrapping_add(*X.offset(4 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x8ac817d as libc::c_int as md5_word_t,
        );
    a = (t << 6 as libc::c_int | t >> 32 as libc::c_int - 6 as libc::c_int)
        .wrapping_add(b);
    t = d
        .wrapping_add(b ^ (a | !c))
        .wrapping_add(*X.offset(11 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x42c50dca as libc::c_int as md5_word_t,
        );
    d = (t << 10 as libc::c_int | t >> 32 as libc::c_int - 10 as libc::c_int)
        .wrapping_add(a);
    t = c
        .wrapping_add(a ^ (d | !b))
        .wrapping_add(*X.offset(2 as libc::c_int as isize))
        .wrapping_add(0x2ad7d2bb as libc::c_int as md5_word_t);
    c = (t << 15 as libc::c_int | t >> 32 as libc::c_int - 15 as libc::c_int)
        .wrapping_add(d);
    t = b
        .wrapping_add(d ^ (c | !a))
        .wrapping_add(*X.offset(9 as libc::c_int as isize))
        .wrapping_add(
            !(0 as libc::c_int) as md5_word_t ^ 0x14792c6e as libc::c_int as md5_word_t,
        );
    b = (t << 21 as libc::c_int | t >> 32 as libc::c_int - 21 as libc::c_int)
        .wrapping_add(c);
    (*pms)
        .abcd[0 as libc::c_int
        as usize] = ((*pms).abcd[0 as libc::c_int as usize]).wrapping_add(a);
    (*pms)
        .abcd[1 as libc::c_int
        as usize] = ((*pms).abcd[1 as libc::c_int as usize]).wrapping_add(b);
    (*pms)
        .abcd[2 as libc::c_int
        as usize] = ((*pms).abcd[2 as libc::c_int as usize]).wrapping_add(c);
    (*pms)
        .abcd[3 as libc::c_int
        as usize] = ((*pms).abcd[3 as libc::c_int as usize]).wrapping_add(d);
}
#[no_mangle]
pub unsafe extern "C" fn md5_init(mut pms: *mut md5_state_t) {
    (*pms).count[1 as libc::c_int as usize] = 0 as libc::c_int as md5_word_t;
    (*pms).count[0 as libc::c_int as usize] = (*pms).count[1 as libc::c_int as usize];
    (*pms).abcd[0 as libc::c_int as usize] = 0x67452301 as libc::c_int as md5_word_t;
    (*pms)
        .abcd[1 as libc::c_int
        as usize] = !(0 as libc::c_int) as md5_word_t
        ^ 0x10325476 as libc::c_int as md5_word_t;
    (*pms)
        .abcd[2 as libc::c_int
        as usize] = !(0 as libc::c_int) as md5_word_t
        ^ 0x67452301 as libc::c_int as md5_word_t;
    (*pms).abcd[3 as libc::c_int as usize] = 0x10325476 as libc::c_int as md5_word_t;
}
#[no_mangle]
pub unsafe extern "C" fn md5_append(
    mut pms: *mut md5_state_t,
    mut data: *const md5_byte_t,
    mut nbytes: libc::c_int,
) {
    let mut p: *const md5_byte_t = data;
    let mut left: libc::c_int = nbytes;
    let mut offset: libc::c_int = ((*pms).count[0 as libc::c_int as usize]
        >> 3 as libc::c_int & 63 as libc::c_int as md5_word_t) as libc::c_int;
    let mut nbits: md5_word_t = (nbytes << 3 as libc::c_int) as md5_word_t;
    if nbytes <= 0 as libc::c_int {
        return;
    }
    (*pms)
        .count[1 as libc::c_int
        as usize] = ((*pms).count[1 as libc::c_int as usize])
        .wrapping_add((nbytes >> 29 as libc::c_int) as md5_word_t);
    (*pms)
        .count[0 as libc::c_int
        as usize] = ((*pms).count[0 as libc::c_int as usize]).wrapping_add(nbits);
    if (*pms).count[0 as libc::c_int as usize] < nbits {
        (*pms)
            .count[1 as libc::c_int
            as usize] = ((*pms).count[1 as libc::c_int as usize]).wrapping_add(1);
        (*pms).count[1 as libc::c_int as usize];
    }
    if offset != 0 {
        let mut copy: libc::c_int = if offset + nbytes > 64 as libc::c_int {
            64 as libc::c_int - offset
        } else {
            nbytes
        };
        memcpy(
            ((*pms).buf).as_mut_ptr().offset(offset as isize) as *mut libc::c_void,
            p as *const libc::c_void,
            copy as libc::c_ulong,
        );
        if offset + copy < 64 as libc::c_int {
            return;
        }
        p = p.offset(copy as isize);
        left -= copy;
        md5_process(pms, ((*pms).buf).as_mut_ptr());
    }
    while left >= 64 as libc::c_int {
        md5_process(pms, p);
        p = p.offset(64 as libc::c_int as isize);
        left -= 64 as libc::c_int;
    }
    if left != 0 {
        memcpy(
            ((*pms).buf).as_mut_ptr() as *mut libc::c_void,
            p as *const libc::c_void,
            left as libc::c_ulong,
        );
    }
}
#[no_mangle]
pub unsafe extern "C" fn md5_finish(
    mut pms: *mut md5_state_t,
    mut digest: *mut md5_byte_t,
) {
    static mut pad_0: [md5_byte_t; 64] = [
        0x80 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
        0 as libc::c_int as md5_byte_t,
    ];
    let mut data: [md5_byte_t; 8] = [0; 8];
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < 8 as libc::c_int {
        data[i
            as usize] = ((*pms).count[(i >> 2 as libc::c_int) as usize]
            >> ((i & 3 as libc::c_int) << 3 as libc::c_int)) as md5_byte_t;
        i += 1;
        i;
    }
    md5_append(
        pms,
        pad_0.as_ptr(),
        ((55 as libc::c_int as md5_word_t)
            .wrapping_sub((*pms).count[0 as libc::c_int as usize] >> 3 as libc::c_int)
            & 63 as libc::c_int as md5_word_t)
            .wrapping_add(1 as libc::c_int as md5_word_t) as libc::c_int,
    );
    md5_append(pms, data.as_mut_ptr(), 8 as libc::c_int);
    i = 0 as libc::c_int;
    while i < 16 as libc::c_int {
        *digest
            .offset(
                i as isize,
            ) = ((*pms).abcd[(i >> 2 as libc::c_int) as usize]
            >> ((i & 3 as libc::c_int) << 3 as libc::c_int)) as md5_byte_t;
        i += 1;
        i;
    }
}
