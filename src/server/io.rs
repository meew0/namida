use crate::extc;
use ::libc;
use anyhow::bail;

use super::{Parameter, Session};

pub unsafe fn build_datagram(
    session: &mut Session,
    parameter: &Parameter,
    mut block_index: u32,
    mut block_type: u16,
    mut datagram: *mut u8,
) -> anyhow::Result<()> {
    static mut last_block: u32 = 0 as libc::c_int as u32;
    let mut status: libc::c_int = 0;
    if block_index != last_block.wrapping_add(1 as libc::c_int as u32) {
        extc::fseeko(
            session.transfer.file,
            (parameter.block_size as u64 * block_index.wrapping_sub(1 as libc::c_int as u32) as u64)
                as extc::__off64_t,
            0 as libc::c_int,
        );
    }
    status = extc::fread(
        datagram.offset(6 as libc::c_int as isize) as *mut libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        parameter.block_size as libc::c_ulong,
        session.transfer.file,
    ) as libc::c_int;
    if status < 0 as libc::c_int {
        bail!("Could not read block #{}", block_index);
    }
    *(datagram.offset(0 as libc::c_int as isize) as *mut u32) = extc::__bswap_32(block_index);
    *(datagram.offset(4 as libc::c_int as isize) as *mut u16) = extc::__bswap_16(block_type);
    last_block = block_index;
    Ok(())
}
