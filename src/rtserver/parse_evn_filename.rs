use ::libc;
extern "C" {
    pub type _IO_wide_data;
    pub type _IO_codecvt;
    pub type _IO_marker;
    static mut stderr: *mut FILE;
    fn fprintf(_: *mut FILE, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn printf(_: *const libc::c_char, _: ...) -> libc::c_int;
    fn sscanf(_: *const libc::c_char, _: *const libc::c_char, _: ...) -> libc::c_int;
    fn malloc(_: libc::c_ulong) -> *mut libc::c_void;
    fn calloc(_: libc::c_ulong, _: libc::c_ulong) -> *mut libc::c_void;
    fn realloc(_: *mut libc::c_void, _: libc::c_ulong) -> *mut libc::c_void;
    fn free(_: *mut libc::c_void);
    fn memset(
        _: *mut libc::c_void,
        _: libc::c_int,
        _: libc::c_ulong,
    ) -> *mut libc::c_void;
    fn strncmp(
        _: *const libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> libc::c_int;
    fn strdup(_: *const libc::c_char) -> *mut libc::c_char;
    fn strchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strrchr(_: *const libc::c_char, _: libc::c_int) -> *mut libc::c_char;
    fn strlen(_: *const libc::c_char) -> libc::c_ulong;
    fn strncpy(
        _: *mut libc::c_char,
        _: *const libc::c_char,
        _: libc::c_ulong,
    ) -> *mut libc::c_char;
    fn __assert_fail(
        __assertion: *const libc::c_char,
        __file: *const libc::c_char,
        __line: libc::c_uint,
        __function: *const libc::c_char,
    ) -> !;
    fn time(__timer: *mut time_t) -> time_t;
    fn difftime(__time1: time_t, __time0: time_t) -> libc::c_double;
    fn mktime(__tp: *mut tm) -> time_t;
    fn gmtime(__timer: *const time_t) -> *mut tm;
    static mut timezone: libc::c_long;
    fn gettimeofday(__tv: *mut timeval, __tz: *mut libc::c_void) -> libc::c_int;
    fn floor(_: libc::c_double) -> libc::c_double;
}
pub type size_t = libc::c_ulong;
pub type __off_t = libc::c_long;
pub type __off64_t = libc::c_long;
pub type __time_t = libc::c_long;
pub type __suseconds_t = libc::c_long;
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
pub type time_t = __time_t;
#[derive(Copy, Clone)]
#[repr(C)]
pub struct timeval {
    pub tv_sec: __time_t,
    pub tv_usec: __suseconds_t,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct tm {
    pub tm_sec: libc::c_int,
    pub tm_min: libc::c_int,
    pub tm_hour: libc::c_int,
    pub tm_mday: libc::c_int,
    pub tm_mon: libc::c_int,
    pub tm_year: libc::c_int,
    pub tm_wday: libc::c_int,
    pub tm_yday: libc::c_int,
    pub tm_isdst: libc::c_int,
    pub tm_gmtoff: libc::c_long,
    pub tm_zone: *const libc::c_char,
}
#[derive(Copy, Clone)]
#[repr(C)]
pub struct evn_filename {
    pub exp_name: *mut libc::c_char,
    pub station_code: *mut libc::c_char,
    pub scan_name: *mut libc::c_char,
    pub data_start_time_ascii: *mut libc::c_char,
    pub data_start_time: libc::c_double,
    pub auxinfo: *mut *mut libc::c_char,
    pub nr_auxinfo: libc::c_int,
    pub file_type: *mut libc::c_char,
    pub valid: libc::c_char,
}
#[no_mangle]
pub unsafe extern "C" fn substrdup(
    mut str: *mut libc::c_char,
    mut maxlength: size_t,
) -> *mut libc::c_char {
    let mut s: *mut libc::c_char = malloc(
        (::core::mem::size_of::<libc::c_char>() as libc::c_ulong)
            .wrapping_mul(maxlength.wrapping_add(1 as libc::c_int as size_t)),
    ) as *mut libc::c_char;
    if s.is_null() {
        return 0 as *mut libc::c_char;
    }
    strncpy(s, str, maxlength);
    *s.offset(maxlength as isize) = '\0' as i32 as libc::c_char;
    return s;
}
#[no_mangle]
pub unsafe extern "C" fn add_aux_entry(
    mut ef: *mut evn_filename,
    mut auxentry: *mut libc::c_char,
) {
    let mut p: *mut *mut libc::c_char = 0 as *mut *mut libc::c_char;
    (*ef).nr_auxinfo += 1;
    (*ef).nr_auxinfo;
    p = realloc(
        (*ef).auxinfo as *mut libc::c_void,
        (::core::mem::size_of::<*mut libc::c_char>() as libc::c_ulong)
            .wrapping_mul((*ef).nr_auxinfo as libc::c_ulong),
    ) as *mut *mut libc::c_char;
    if !p.is_null() {} else {
        __assert_fail(
            b"p\0" as *const u8 as *const libc::c_char,
            b"parse_evn_filename.c\0" as *const u8 as *const libc::c_char,
            55 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 50],
                &[libc::c_char; 50],
            >(b"void add_aux_entry(struct evn_filename *, char *)\0"))
                .as_ptr(),
        );
    }
    'c_6757: {
        if !p.is_null() {} else {
            __assert_fail(
                b"p\0" as *const u8 as *const libc::c_char,
                b"parse_evn_filename.c\0" as *const u8 as *const libc::c_char,
                55 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 50],
                    &[libc::c_char; 50],
                >(b"void add_aux_entry(struct evn_filename *, char *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*ef).auxinfo = p;
    let ref mut fresh0 = *((*ef).auxinfo)
        .offset(((*ef).nr_auxinfo - 1 as libc::c_int) as isize);
    *fresh0 = auxentry;
}
#[no_mangle]
pub unsafe extern "C" fn year_to_utc(mut year: libc::c_int) -> time_t {
    let mut tm: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    memset(
        &mut tm as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<tm>() as libc::c_ulong,
    );
    tm.tm_year = year - 1900 as libc::c_int;
    tm.tm_mday = 1 as libc::c_int;
    return mktime(&mut tm) as libc::c_double as time_t;
}
#[no_mangle]
pub unsafe extern "C" fn get_current_year() -> libc::c_int {
    let mut t: time_t = 0;
    let mut tm: *mut tm = 0 as *mut tm;
    time(&mut t);
    tm = gmtime(&mut t);
    return (*tm).tm_year + 1900 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn day_to_utc(mut day: libc::c_int) -> libc::c_double {
    return ((day - 1 as libc::c_int) * 24 as libc::c_int * 60 as libc::c_int
        * 60 as libc::c_int) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn hour_to_utc(mut hour: libc::c_int) -> libc::c_double {
    return (hour * 60 as libc::c_int * 60 as libc::c_int) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn minute_to_utc(mut minute: libc::c_int) -> libc::c_double {
    return (minute * 60 as libc::c_int) as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn interpret_as_utc(
    mut year: libc::c_int,
    mut daycount: libc::c_int,
    mut month: libc::c_int,
    mut day: libc::c_int,
    mut hour: libc::c_int,
    mut min: libc::c_int,
    mut sec: libc::c_int,
) -> libc::c_double {
    let mut ret: time_t = 0;
    let mut tt: tm = tm {
        tm_sec: 0,
        tm_min: 0,
        tm_hour: 0,
        tm_mday: 0,
        tm_mon: 0,
        tm_year: 0,
        tm_wday: 0,
        tm_yday: 0,
        tm_isdst: 0,
        tm_gmtoff: 0,
        tm_zone: 0 as *const libc::c_char,
    };
    let mut p_tt2: *mut tm = 0 as *mut tm;
    let mut timet: time_t = 0;
    memset(
        &mut tt as *mut tm as *mut libc::c_void,
        0 as libc::c_int,
        ::core::mem::size_of::<tm>() as libc::c_ulong,
    );
    if daycount == 0 as libc::c_int
        && (day != 0 as libc::c_int && month != 0 as libc::c_int)
    {
        tt.tm_mday = day;
        tt.tm_mon = month - 1 as libc::c_int;
    } else if daycount != 0 as libc::c_int
        && (day == 0 as libc::c_int && month == 0 as libc::c_int)
    {
        tt.tm_mday = 1 as libc::c_int;
        tt.tm_mon = 0 as libc::c_int;
    } else if daycount == 0 as libc::c_int && day == 0 as libc::c_int
        && month == 0 as libc::c_int
    {
        time(&mut timet);
        p_tt2 = gmtime(&mut timet);
        tt.tm_mon = (*p_tt2).tm_mon;
        tt.tm_mday = (*p_tt2).tm_mday;
        year = 0 as libc::c_int;
    }
    if year == 0 as libc::c_int {
        year = get_current_year();
    }
    tt.tm_year = year - 1900 as libc::c_int;
    tt.tm_sec = sec;
    tt.tm_min = min;
    tt.tm_hour = hour;
    tt.tm_isdst = 0 as libc::c_int;
    ret = mktime(&mut tt);
    if daycount != 0 as libc::c_int {
        ret
            += (daycount - 1 as libc::c_int) as libc::c_long * 24 as libc::c_long
                * 60 as libc::c_long * 60 as libc::c_long;
    }
    ret -= timezone;
    printf(
        b"DEBUG: interpret_as_utc(%d, %d days, %d, %d, %dh, %dm, %ds) : fixed_mktime()=%f\n\0"
            as *const u8 as *const libc::c_char,
        year,
        daycount,
        month,
        day,
        hour,
        min,
        sec,
        ret as libc::c_double,
    );
    return ret as libc::c_double;
}
#[no_mangle]
pub unsafe extern "C" fn parse_time(
    mut str: *const libc::c_char,
    mut retval: *mut libc::c_double,
) -> libc::c_int {
    let mut yyyy: libc::c_int = 0 as libc::c_int;
    let mut mm: libc::c_int = 0 as libc::c_int;
    let mut dd: libc::c_int = 0 as libc::c_int;
    let mut hh: libc::c_int = 0 as libc::c_int;
    let mut min: libc::c_int = 0 as libc::c_int;
    let mut yday: libc::c_int = 0 as libc::c_int;
    let mut sec: libc::c_int = 0 as libc::c_int;
    let mut dsec: libc::c_double = 0.0f64;
    let mut fsec: libc::c_float = 0.0f64 as libc::c_float;
    let mut consumed: libc::c_int = 0 as libc::c_int;
    let mut tvnow: timeval = timeval { tv_sec: 0, tv_usec: 0 };
    *retval = 0 as libc::c_int as libc::c_double;
    if sscanf(
        str,
        b"%4d-%2d-%2dT%2d:%2d:%lg%n\0" as *const u8 as *const libc::c_char,
        &mut yyyy as *mut libc::c_int,
        &mut mm as *mut libc::c_int,
        &mut dd as *mut libc::c_int,
        &mut hh as *mut libc::c_int,
        &mut min as *mut libc::c_int,
        &mut dsec as *mut libc::c_double,
        &mut consumed as *mut libc::c_int,
    ) == 6 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            yyyy,
            0 as libc::c_int,
            mm,
            dd,
            hh,
            min,
            floor(dsec) as libc::c_int,
        );
        *retval += dsec - floor(dsec);
        fprintf(
            stderr,
            b"Detected time format: ISO basic extended\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strlen(str) == 13 as libc::c_int as libc::c_ulong
        && sscanf(
            str,
            b"%04d%03d%02d%02d%02d%n\0" as *const u8 as *const libc::c_char,
            &mut yyyy as *mut libc::c_int,
            &mut yday as *mut libc::c_int,
            &mut hh as *mut libc::c_int,
            &mut mm as *mut libc::c_int,
            &mut sec as *mut libc::c_int,
            &mut consumed as *mut libc::c_int,
        ) == 5 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            yyyy,
            yday,
            0 as libc::c_int,
            0 as libc::c_int,
            hh,
            mm,
            sec,
        );
        fprintf(
            stderr,
            b"Detected time format: yyyydddhhmmss\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strlen(str) == 9 as libc::c_int as libc::c_ulong
        && sscanf(
            str,
            b"%03d%02d%02d%02d%n\0" as *const u8 as *const libc::c_char,
            &mut yday as *mut libc::c_int,
            &mut hh as *mut libc::c_int,
            &mut mm as *mut libc::c_int,
            &mut sec as *mut libc::c_int,
            &mut consumed as *mut libc::c_int,
        ) == 4 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            0 as libc::c_int,
            yday,
            0 as libc::c_int,
            0 as libc::c_int,
            hh,
            mm,
            sec,
        );
        fprintf(
            stderr,
            b"Detected time format: dddhhmmss\n\0" as *const u8 as *const libc::c_char,
        );
    } else if sscanf(
        str,
        b"%4dy%dd%n\0" as *const u8 as *const libc::c_char,
        &mut yyyy as *mut libc::c_int,
        &mut yday as *mut libc::c_int,
        &mut consumed as *mut libc::c_int,
    ) == 2 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            yyyy,
            yday,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        fprintf(
            stderr,
            b"Detected time format: [yyyy]y[dd]d\n\0" as *const u8 as *const libc::c_char,
        );
    } else if sscanf(
        str,
        b"%dy%dd%dh%dm%ds%n\0" as *const u8 as *const libc::c_char,
        &mut yyyy as *mut libc::c_int,
        &mut yday as *mut libc::c_int,
        &mut hh as *mut libc::c_int,
        &mut mm as *mut libc::c_int,
        &mut sec as *mut libc::c_int,
        &mut consumed as *mut libc::c_int,
    ) == 5 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            yyyy,
            yday,
            0 as libc::c_int,
            0 as libc::c_int,
            hh,
            mm,
            sec,
        );
        fprintf(
            stderr,
            b"Detected time format: [yyyy]y[dd]d[hh]h[mm]m[ss]s\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strlen(str) == 7 as libc::c_int as libc::c_ulong
        && sscanf(
            str,
            b"%04d%03d%n\0" as *const u8 as *const libc::c_char,
            &mut yyyy as *mut libc::c_int,
            &mut yday as *mut libc::c_int,
            &mut consumed as *mut libc::c_int,
        ) == 2 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            yyyy,
            yday,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
        );
        fprintf(
            stderr,
            b"Detected time format: yyyyddd\n\0" as *const u8 as *const libc::c_char,
        );
    } else if sscanf(
        str,
        b"%dd%dh%dm%ds%n\0" as *const u8 as *const libc::c_char,
        &mut yday as *mut libc::c_int,
        &mut hh as *mut libc::c_int,
        &mut mm as *mut libc::c_int,
        &mut sec as *mut libc::c_int,
        &mut consumed as *mut libc::c_int,
    ) == 4 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            0 as libc::c_int,
            yday,
            0 as libc::c_int,
            0 as libc::c_int,
            hh,
            mm,
            sec,
        );
        fprintf(
            stderr,
            b"Detected time format: [d]d[h]h[m]m[s]s\n\0" as *const u8
                as *const libc::c_char,
        );
    } else if strlen(str) >= 6 as libc::c_int as libc::c_ulong
        && sscanf(
            str,
            b"%02d%02d%02f%n\0" as *const u8 as *const libc::c_char,
            &mut hh as *mut libc::c_int,
            &mut mm as *mut libc::c_int,
            &mut fsec as *mut libc::c_float,
            &mut consumed as *mut libc::c_int,
        ) == 3 as libc::c_int && consumed as libc::c_ulong == strlen(str)
    {
        *retval = interpret_as_utc(
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            0 as libc::c_int,
            hh,
            mm,
            floor(fsec as libc::c_double) as libc::c_int,
        );
        *retval += fsec as libc::c_double - floor(fsec as libc::c_double);
        fprintf(
            stderr,
            b"Detected time format: hhmmss.cc\n\0" as *const u8 as *const libc::c_char,
        );
    } else {
        fprintf(
            stderr,
            b"Warning: string with unknown time format passed to parse_time(), assuming it is aux entries.\n\0"
                as *const u8 as *const libc::c_char,
        );
        return 1 as libc::c_int;
    }
    if gettimeofday(&mut tvnow, 0 as *mut libc::c_void) == 0 as libc::c_int
        && difftime(*retval as time_t, tvnow.tv_sec)
            <= 0 as libc::c_int as libc::c_double
    {
        fprintf(
            stderr,
            b"Start time in the past\n\0" as *const u8 as *const libc::c_char,
        );
        *retval = 0.0f64;
    }
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn get_aux_entry(
    mut key: *mut libc::c_char,
    mut auxinfo: *mut *mut libc::c_char,
    mut nr_auxinfo: libc::c_int,
) -> *mut libc::c_char {
    let mut i: libc::c_int = 0;
    i = 0 as libc::c_int;
    while i < nr_auxinfo {
        if strlen(*auxinfo.offset(i as isize)) > strlen(key)
            && strncmp(*auxinfo.offset(i as isize), key, strlen(key)) == 0
            && *(*auxinfo.offset(i as isize)).offset(strlen(key) as isize) as libc::c_int
                == '=' as i32
        {
            return strdup(
                (*auxinfo.offset(i as isize))
                    .offset(strlen(key) as isize)
                    .offset(1 as libc::c_int as isize),
            );
        }
        i += 1;
        i;
    }
    return 0 as *mut libc::c_char;
}
#[no_mangle]
pub unsafe extern "C" fn get_token(
    mut str: *mut *mut libc::c_char,
) -> *mut libc::c_char {
    let mut p: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut retval: *mut libc::c_char = 0 as *mut libc::c_char;
    if (*str).is_null() || **str.offset(0 as libc::c_int as isize) == 0 {
        return 0 as *mut libc::c_char;
    }
    p = strchr(*str, '_' as i32);
    if p.is_null() {
        p = strchr(*str, '\0' as i32);
        if p.is_null() {
            return 0 as *mut libc::c_char;
        }
        retval = strdup(*str);
        *str = p;
    } else {
        retval = substrdup(*str, p.offset_from(*str) as libc::c_long as size_t);
        *str = p.offset(1 as libc::c_int as isize);
    }
    return retval;
}
#[no_mangle]
pub unsafe extern "C" fn parse_evn_filename(
    mut filename: *mut libc::c_char,
) -> *mut evn_filename {
    let mut ef: *mut evn_filename = 0 as *mut evn_filename;
    let mut parsebuf: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut parseptr: *mut libc::c_char = 0 as *mut libc::c_char;
    ef = calloc(
        ::core::mem::size_of::<evn_filename>() as libc::c_ulong,
        1 as libc::c_int as libc::c_ulong,
    ) as *mut evn_filename;
    if !ef.is_null() {} else {
        __assert_fail(
            b"ef\0" as *const u8 as *const libc::c_char,
            b"parse_evn_filename.c\0" as *const u8 as *const libc::c_char,
            287 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"struct evn_filename *parse_evn_filename(char *)\0"))
                .as_ptr(),
        );
    }
    'c_8395: {
        if !ef.is_null() {} else {
            __assert_fail(
                b"ef\0" as *const u8 as *const libc::c_char,
                b"parse_evn_filename.c\0" as *const u8 as *const libc::c_char,
                287 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"struct evn_filename *parse_evn_filename(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    parsebuf = strdup(filename);
    parseptr = parsebuf;
    if !parsebuf.is_null() {} else {
        __assert_fail(
            b"parsebuf\0" as *const u8 as *const libc::c_char,
            b"parse_evn_filename.c\0" as *const u8 as *const libc::c_char,
            290 as libc::c_int as libc::c_uint,
            (*::core::mem::transmute::<
                &[u8; 48],
                &[libc::c_char; 48],
            >(b"struct evn_filename *parse_evn_filename(char *)\0"))
                .as_ptr(),
        );
    }
    'c_8351: {
        if !parsebuf.is_null() {} else {
            __assert_fail(
                b"parsebuf\0" as *const u8 as *const libc::c_char,
                b"parse_evn_filename.c\0" as *const u8 as *const libc::c_char,
                290 as libc::c_int as libc::c_uint,
                (*::core::mem::transmute::<
                    &[u8; 48],
                    &[libc::c_char; 48],
                >(b"struct evn_filename *parse_evn_filename(char *)\0"))
                    .as_ptr(),
            );
        }
    };
    (*ef).data_start_time_ascii = 0 as *mut libc::c_char;
    (*ef).valid = 1 as libc::c_int as libc::c_char;
    let mut dot: *mut libc::c_char = 0 as *mut libc::c_char;
    let mut filetype: *mut libc::c_char = 0 as *mut libc::c_char;
    dot = strrchr(parseptr, '.' as i32);
    if dot.is_null() {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(dot)\n\0" as *const u8 as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    filetype = dot.offset(1 as libc::c_int as isize);
    (*ef).file_type = get_token(&mut filetype);
    if ((*ef).file_type).is_null() {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(ef->file_type)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    if strlen((*ef).file_type) < 2 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(strlen(ef->file_type)>=2)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    *dot = 0 as libc::c_int as libc::c_char;
    (*ef).exp_name = get_token(&mut parseptr);
    if ((*ef).exp_name).is_null() {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(ef->exp_name)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    if strlen((*ef).exp_name) > 6 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(strlen(ef->exp_name) <= 6)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    (*ef).station_code = get_token(&mut parseptr);
    if ((*ef).station_code).is_null() {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(ef->station_code)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    if strlen((*ef).station_code) < 2 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(strlen(ef->station_code) >= 2)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    (*ef).scan_name = get_token(&mut parseptr);
    if ((*ef).scan_name).is_null() {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(ef->scan_name)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    if strlen((*ef).scan_name) > 16 as libc::c_int as libc::c_ulong {
        fprintf(
            stderr,
            b"parse_evn_filename: assert(strlen(ef->scan_name) <= 16)\n\0" as *const u8
                as *const libc::c_char,
        );
        (*ef).valid = 0 as libc::c_int as libc::c_char;
        return ef;
    }
    (*ef).data_start_time_ascii = get_token(&mut parseptr);
    if !((*ef).data_start_time_ascii).is_null() {
        if strlen((*ef).data_start_time_ascii) < 2 as libc::c_int as libc::c_ulong {
            (*ef).data_start_time_ascii = 0 as *mut libc::c_char;
            (*ef).valid = 0 as libc::c_int as libc::c_char;
            fprintf(
                stderr,
                b"parse_evn_filename: assert(strlen(ef->data_start_time_ascii) >= 2)\n\0"
                    as *const u8 as *const libc::c_char,
            );
            return ef;
        }
        if parse_time((*ef).data_start_time_ascii, &mut (*ef).data_start_time) != 0 {
            add_aux_entry(ef, (*ef).data_start_time_ascii);
            (*ef).data_start_time_ascii = 0 as *mut libc::c_char;
        }
    }
    let mut auxentry: *mut libc::c_char = 0 as *mut libc::c_char;
    loop {
        auxentry = get_token(&mut parseptr);
        if auxentry.is_null() {
            break;
        }
        add_aux_entry(ef, auxentry);
    }
    free(parsebuf as *mut libc::c_void);
    return ef;
}
