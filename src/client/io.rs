use super::{ttp_session_t, ttp_transfer_t};
use crate::{datagram, extc};
use ::libc;
use anyhow::bail;

pub unsafe fn accept_block(
    mut session: *mut ttp_session_t,
    datagram: datagram::View,
) -> anyhow::Result<()> {
    let mut transfer: *mut ttp_transfer_t = &mut (*session).transfer;
    let mut block_size: u32 = (*(*session).parameter).block_size;
    let mut write_size: u32 = 0;
    let mut status: libc::c_int = 0;
    if datagram.header.block_index == (*transfer).block_count {
        write_size = ((*transfer).file_size % block_size as u64) as u32;
        if write_size == 0 as libc::c_int as u32 {
            write_size = block_size;
        }
    } else {
        write_size = block_size;
    }
    status = extc::fseeko(
        (*transfer).file,
        (block_size as u64
            * datagram
                .header
                .block_index
                .wrapping_sub(1 as libc::c_int as u32) as u64) as extc::__off64_t,
        0 as libc::c_int,
    );
    if status < 0 as libc::c_int {
        bail!(
            "Could not seek at block {} of file",
            datagram.header.block_index
        );
    }
    status = extc::fwrite(
        datagram.block.as_ptr() as *const libc::c_void,
        1 as libc::c_int as libc::c_ulong,
        write_size as libc::c_ulong,
        (*transfer).file,
    ) as libc::c_int;
    if (status as u32) < write_size {
        bail!(
            "Could not write block {} of file",
            datagram.header.block_index
        );
    }

    Ok(())
}
