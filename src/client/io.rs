use super::{ttp_session_t, ttp_transfer_t};
use crate::extc;
use ::libc;

#[no_mangle]
pub unsafe extern "C" fn accept_block(
    mut session: *mut ttp_session_t,
    mut block_index: u32,
    mut block: *mut u8,
) -> libc::c_int {
    let mut transfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut block_size: u32 = (*(*session).parameter).block_size;
    let mut write_size: u32 = 0;
    let mut status: libc::c_int = 0;
    if block_index == (*transfer).block_count {
        write_size = ((*transfer).file_size % block_size as u64) as u32;
        if write_size == 0 as libc::c_int as u32 {
            write_size = block_size;
        }
    } else {
        write_size = block_size;
    }
    status = extc::fseeko(
        (*transfer).file,
        (block_size as u64 * block_index.wrapping_sub(1 as libc::c_int as u32) as u64)
            as extc::__off64_t,
        0 as libc::c_int,
    );
    if status < 0 as libc::c_int {
        extc::sprintf(
            crate::common::error::g_error.as_mut_ptr(),
            b"Could not seek at block %d of file\0" as *const u8 as *const libc::c_char,
            block_index,
        );
        return crate::common::error::error_handler(
            b"io.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            crate::common::error::g_error.as_mut_ptr(),
            0 as libc::c_int,
        );
    }
    status = extc::fwrite(
        block as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        write_size as libc::c_ulong,
        (*transfer).file,
    ) as libc::c_int;
    if (status as u32) < write_size {
        extc::sprintf(
            crate::common::error::g_error.as_mut_ptr(),
            b"Could not write block %d of file\0" as *const u8 as *const libc::c_char,
            block_index,
        );
        return crate::common::error::error_handler(
            b"io.c\0" as *const u8 as *const libc::c_char,
            114 as libc::c_int,
            crate::common::error::g_error.as_mut_ptr(),
            0 as libc::c_int,
        );
    }
    return 0 as libc::c_int;
}
