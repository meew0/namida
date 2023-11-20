use crate::extc;
use ::libc;

#[no_mangle]
pub unsafe extern "C" fn build_datagram(
    mut session: *mut super::ttp_session_t,
    mut block_index: u32,
    mut block_type: u16,
    mut datagram: *mut u8,
) -> libc::c_int {
    static mut last_block: u32 = 0 as libc::c_int as u32;
    let mut status: libc::c_int = 0;
    if block_index != last_block.wrapping_add(1 as libc::c_int as u32) {
        extc::fseeko(
            (*session).transfer.file,
            ((*(*session).parameter).block_size as u64
                * block_index.wrapping_sub(1 as libc::c_int as u32) as u64)
                as extc::__off64_t,
            0 as libc::c_int,
        );
    }
    status = extc::fread(
        datagram.offset(6 as libc::c_int as isize) as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        (*(*session).parameter).block_size as libc::c_ulong,
        (*session).transfer.file,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        extc::sprintf(
            crate::common::error::g_error.as_mut_ptr(),
            b"Could not read block #%u\0" as *const u8 as *const libc::c_char,
            block_index,
        );
        return crate::common::error::error_handler(
            b"io.c\0" as *const u8 as *const libc::c_char,
            107 as libc::c_int,
            crate::common::error::g_error.as_mut_ptr(),
            0 as libc::c_int,
        );
    }
    *(datagram.offset(0 as libc::c_int as isize) as *mut u32) = extc::__bswap_32(block_index);
    *(datagram.offset(4 as libc::c_int as isize) as *mut u16) = extc::__bswap_16(block_type);
    last_block = block_index;
    return 0 as libc::c_int;
}
