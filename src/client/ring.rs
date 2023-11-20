use crate::extc;
use ::libc;
use anyhow::bail;

use super::{ring_buffer_t, ttp_session_t};

pub const EMPTY: i32 = -1;
pub unsafe fn ring_full(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    let mut full: libc::c_int = 0;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not get access to ring buffer mutex");
    }
    full = ((*ring).space_ready == 0) as libc::c_int;
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not relinquish access to ring buffer mutex");
    }
    return full;
}
pub unsafe fn ring_cancel(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not get access to ring buffer mutex");
    }
    (*ring).count_reserved -= 1;
    if (*ring).count_reserved < 0 as libc::c_int {
        panic!("Attempt made to cancel unreserved slot in ring buffer");
    }
    (*ring).space_ready = 1 as libc::c_int;
    status = extc::pthread_cond_signal(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        panic!("Could not signal space-ready condition");
    }
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not relinquish access to ring buffer mutex");
    }
    return 0 as libc::c_int;
}
pub unsafe fn ring_confirm(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not get access to ring buffer mutex");
    }
    (*ring).count_data += 1;
    (*ring).count_data;
    (*ring).count_reserved -= 1;
    if (*ring).count_reserved < 0 as libc::c_int {
        panic!("Attempt made to confirm unreserved slot in ring buffer");
    }
    (*ring).data_ready = 1 as libc::c_int;
    status = extc::pthread_cond_signal(&mut (*ring).data_ready_cond);
    if status != 0 as libc::c_int {
        panic!("Could not signal data-ready condition");
    }
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not relinquish access to ring buffer mutex");
    }
    return 0 as libc::c_int;
}
pub unsafe fn ring_create(mut session: *mut ttp_session_t) -> *mut ring_buffer_t {
    let mut ring: *mut ring_buffer_t = 0 as *mut ring_buffer_t;
    let mut status: libc::c_int = 0;
    ring = extc::calloc(
        1 as libc::c_int as libc::c_ulong,
        ::core::mem::size_of::<ring_buffer_t>() as libc::c_ulong,
    ) as *mut ring_buffer_t;
    if ring.is_null() {
        panic!("Could not allocate ring buffer object");
    }
    (*ring).datagram_size =
        (6 as libc::c_int as u32).wrapping_add((*(*session).parameter).block_size) as libc::c_int;
    (*ring).datagrams =
        extc::malloc(((*ring).datagram_size * 4096 as libc::c_int) as libc::c_ulong) as *mut u8;
    if ((*ring).datagrams).is_null() {
        panic!("Could not allocate buffer for ring buffer");
    }
    status = extc::pthread_mutex_init(&mut (*ring).mutex, 0 as *const extc::pthread_mutexattr_t);
    if status != 0 as libc::c_int {
        panic!("Could not create mutex for ring buffer");
    }
    status = extc::pthread_cond_init(
        &mut (*ring).data_ready_cond,
        0 as *const extc::pthread_condattr_t,
    );
    if status != 0 as libc::c_int {
        panic!("Could not create data-ready condition variable");
    }
    (*ring).data_ready = 0 as libc::c_int;
    status = extc::pthread_cond_init(
        &mut (*ring).space_ready_cond,
        0 as *const extc::pthread_condattr_t,
    );
    if status != 0 as libc::c_int {
        panic!("Could not create space-ready condition variable");
    }
    (*ring).space_ready = 1 as libc::c_int;
    (*ring).count_data = 0 as libc::c_int;
    (*ring).count_reserved = 0 as libc::c_int;
    (*ring).base_data = 0 as libc::c_int;
    return ring;
}
pub unsafe fn ring_destroy(mut ring: *mut ring_buffer_t) -> anyhow::Result<()> {
    let mut status: libc::c_int = 0;
    status = extc::pthread_mutex_destroy(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        bail!("Could not destroy mutex for ring buffer");
    }
    status = extc::pthread_cond_destroy(&mut (*ring).data_ready_cond);
    if status != 0 as libc::c_int {
        bail!("Could not destroy data-ready condition variable");
    }
    status = extc::pthread_cond_destroy(&mut (*ring).space_ready_cond);
    if status != 0 as libc::c_int {
        bail!("Could not destroy space-ready condition variable");
    }
    extc::free((*ring).datagrams as *mut libc::c_void);
    extc::free(ring as *mut libc::c_void);
    Ok(())
}
pub unsafe fn ring_dump(
    mut ring: *mut ring_buffer_t,
    mut out: *mut extc::FILE,
) -> anyhow::Result<()> {
    let mut status: libc::c_int = 0;
    let mut index: libc::c_int = 0;
    let mut datagram: *mut u8 = 0 as *mut u8;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        bail!("Could not get access to ring buffer mutex");
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
        bail!("Could not relinquish access to ring buffer mutex");
    }
    Ok(())
}
pub unsafe fn ring_peek(mut ring: *mut ring_buffer_t) -> anyhow::Result<*mut u8> {
    let mut status: libc::c_int = 0;
    let mut address: *mut u8 = 0 as *mut u8;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        bail!("Could not get access to ring buffer mutex");
    }
    while (*ring).data_ready == 0 as libc::c_int {
        status = extc::pthread_cond_wait(&mut (*ring).data_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            bail!("Could not wait for ring buffer to accumulate data");
        }
    }
    address = ((*ring).datagrams).offset(((*ring).datagram_size * (*ring).base_data) as isize);
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        bail!("Could not relinquish access to ring buffer mutex");
    }
    Ok(address)
}
pub unsafe fn ring_pop(mut ring: *mut ring_buffer_t) -> libc::c_int {
    let mut status: libc::c_int = 0;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not get access to ring buffer mutex");
    }
    while (*ring).data_ready == 0 as libc::c_int {
        status = extc::pthread_cond_wait(&mut (*ring).data_ready_cond, &mut (*ring).mutex);
        if status != 0 as libc::c_int {
            panic!("Could not wait for ring buffer to accumulate data");
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
        panic!("Could not signal space-ready condition");
    }
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not relinquish access to ring buffer mutex");
    }
    return 0 as libc::c_int;
}
pub unsafe fn ring_reserve(mut ring: *mut ring_buffer_t) -> *mut u8 {
    let mut status: libc::c_int = 0;
    let mut next: libc::c_int = 0;
    let mut address: *mut u8 = 0 as *mut u8;
    status = extc::pthread_mutex_lock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not get access to ring buffer mutex");
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
            panic!("Could not wait for ring buffer to clear space");
        }
    }
    (*ring).count_reserved += 1;
    if (*ring).count_reserved > 1 as libc::c_int {
        panic!("Attempt made to reserve two slots in ring buffer");
    }
    if (next + 1 as libc::c_int) % 4096 as libc::c_int == (*ring).base_data {
        (*ring).space_ready = 0 as libc::c_int;
    }
    address = ((*ring).datagrams).offset((next * (*ring).datagram_size) as isize);
    status = extc::pthread_mutex_unlock(&mut (*ring).mutex);
    if status != 0 as libc::c_int {
        panic!("Could not relinquish access to ring buffer mutex");
    }
    return address;
}
