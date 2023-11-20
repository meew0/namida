use crate::extc;
use ::libc;

use super::{ring_buffer_t, ttp_session_t};

#[no_mangle]
pub static mut EMPTY: libc::c_int = -(1 as libc::c_int);
#[no_mangle]
pub unsafe extern "C" fn ring_full(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut full: libc::c_int = 0;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            91 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    full = ((*ring).space_ready == 0) as libc::c_int;
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
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
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            117 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).count_reserved -= 1;
    if (*ring).count_reserved < 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            121 as libc::c_int,
            b"Attempt made to cancel unreserved slot in ring buffer\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).space_ready = 1 as libc::c_int;
    status = extc::pthread_cond_signal(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            127 as libc::c_int,
            b"Could not signal space-ready condition\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
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
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
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
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            158 as libc::c_int,
            b"Attempt made to confirm unreserved slot in ring buffer\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).data_ready = 1 as libc::c_int;
    status = extc::pthread_cond_signal(&mut (*ring).data_ready_cond);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            164 as libc::c_int,
            b"Could not signal data-ready condition\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
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
    ring = extc::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ring_buffer_t>() as libc::c_ulong,
    ) as *mut ring_buffer_t;
    if ring.is_null() {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            192 as libc::c_int,
            b"Could not allocate ring buffer object\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).datagram_size =
        (6 as libc::c_int as u32).wrapping_add((*(*session).parameter).block_size) as libc::c_int;
    (*ring).datagrams =
        extc::malloc(((*ring).datagram_size * 4096 as libc::c_int) as libc::c_ulong) as *mut u8;
    if ((*ring).datagrams).is_null() {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            198 as libc::c_int,
            b"Could not allocate buffer for ring buffer\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = extc::pthread_mutex_init(&mut (*ring).mutex, 0 as *const extc::pthread_mutexattr_t);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            203 as libc::c_int,
            b"Could not create mutex for ring buffer\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = extc::pthread_cond_init(
        &mut (*ring).data_ready_cond,
        0 as *const extc::pthread_condattr_t,
    );
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            208 as libc::c_int,
            b"Could not create data-ready condition variable\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    (*ring).data_ready = 0 as libc::c_int;
    status = extc::pthread_cond_init(
        &mut (*ring).space_ready_cond,
        0 as *const extc::pthread_condattr_t,
    );
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
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
    status = extc::pthread_mutex_destroy(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        return crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            241 as libc::c_int,
            b"Could not destroy mutex for ring buffer\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = extc::pthread_cond_destroy(&mut (*ring).data_ready_cond);
    if status != 0 as libc::c_int {
        return crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            246 as libc::c_int,
            b"Could not destroy data-ready condition variable\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    status = extc::pthread_cond_destroy(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        return crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            249 as libc::c_int,
            b"Could not destroy space-ready condition variable\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    extc::free((*ring).datagrams as *mut libc::c_void);
    extc::free(ring as *mut libc::c_void);
    return 0 as libc::c_int;
}
#[no_mangle]
pub unsafe extern "C" fn ring_dump(
    mut ring: *mut ring_buffer_t,
    mut out: *mut extc::FILE,
) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut datagram: *mut u8 = 0 as *mut u8;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        return crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            275 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
    }
    extc::fprintf(
        out,
        b"datagram_size  = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).datagram_size,
    );
    extc::fprintf(
        out,
        b"base_data      = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).base_data,
    );
    extc::fprintf(
        out,
        b"count_data     = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).count_data,
    );
    extc::fprintf(
        out,
        b"count_reserved = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).count_reserved,
    );
    extc::fprintf(
        out,
        b"data_ready     = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).data_ready,
    );
    extc::fprintf(
        out,
        b"space_ready    = %d\n\0" as *const u8 as *const libc::c_char,
        (*ring).space_ready,
    );
    extc::fprintf(
        out,
        b"block list     = [\0" as *const u8 as *const libc::c_char,
    );
    index = (*ring).base_data;
    while index < (*ring).base_data + (*ring).count_data {
        datagram = ((*ring).datagrams)
            .offset((index % 4096 as libc::c_int * (*ring).datagram_size) as isize);
        extc::fprintf(
            out,
            b"%d \0" as *const u8 as *const libc::c_char,
            extc::__bswap_32(*(datagram as *mut u32)),
        );
        index += 1;
        index;
    }
    extc::fprintf(out, b"]\n\0" as *const u8 as *const libc::c_char);
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        return crate::common::error::error_handler(
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
pub unsafe extern "C" fn ring_peek(mut ring: *mut ring_buffer_t) -> *mut u8 {
    let mut status: libc::c_int = 0;
    let mut address: *mut u8 = 0 as *mut u8;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            317 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            0 as libc::c_int,
        );
        return 0 as *mut u8;
    }
    while (*ring).data_ready == 0 as libc::c_int {
        status = extc::pthread_cond_wait(&mut (*ring).data_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            crate::common::error::error_handler(
                b"ring.c\0" as *const u8 as *const libc::c_char,
                325 as libc::c_int,
                b"Could not wait for ring buffer to accumulate data\0" as *const u8
                    as *const libc::c_char,
                0 as libc::c_int,
            );
            return 0 as *mut u8;
        }
    }
    address = ((*ring).datagrams).offset(((*ring).datagram_size * (*ring).base_data) as isize);
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            336 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            0 as libc::c_int,
        );
        return 0 as *mut u8;
    }
    return address;
}
#[no_mangle]
pub unsafe extern "C" fn ring_pop(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            361 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    while (*ring).data_ready == 0 as libc::c_int {
        status = extc::pthread_cond_wait(&mut (*ring).data_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            crate::common::error::error_handler(
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
    status = extc::pthread_cond_signal(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            379 as libc::c_int,
            b"Could not signal space-ready condition\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
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
pub unsafe extern "C" fn ring_reserve(mut ring: *mut ring_buffer_t) -> *mut u8 {
    let mut status: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut address: *mut u8 = 0 as *mut u8;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            408 as libc::c_int,
            b"Could not get access to ring buffer mutex\0" as *const u8 as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    next = ((*ring).base_data + (*ring).count_data + (*ring).count_reserved) % 4096 as libc::c_int;
    while (*ring).space_ready == 0 as libc::c_int {
        extc::printf(b"FULL! -- ring_reserve() blocking.\n\0" as *const u8 as *const libc::c_char);
        extc::printf(
            b"space_ready = %d, data_ready = %d\n\0" as *const u8 as *const libc::c_char,
            (*ring).space_ready,
            (*ring).data_ready,
        );
        status = extc::pthread_cond_wait(&mut (*ring).space_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            crate::common::error::error_handler(
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
        crate::common::error::error_handler(
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
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        crate::common::error::error_handler(
            b"ring.c\0" as *const u8 as *const libc::c_char,
            434 as libc::c_int,
            b"Could not relinquish access to ring buffer mutex\0" as *const u8
                as *const libc::c_char,
            1 as libc::c_int,
        );
    }
    return address;
}
