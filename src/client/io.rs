use std::io::{Seek, SeekFrom, Write};

use crate::{
    datagram,
    types::{BlockIndex, FileSize},
};

/// Accepts the given block of data, which involves writing the block to disk.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics on arithmetic overflow.
pub fn accept_block(
    datagram: datagram::View,
    block_count: BlockIndex,
    file_size: FileSize,
    file: &mut std::fs::File,
) -> anyhow::Result<()> {
    let block_size = datagram.block.len();

    // seek to the proper location
    let offset = (block_size as u64)
        .checked_mul(u64::from(
            (datagram.header.block_index.safe_sub(BlockIndex(1))).0,
        ))
        .expect("offset overflow");
    file.seek(SeekFrom::Start(offset))?;

    // figure out how many bytes to write
    let slice_to_write = if datagram.header.block_index == block_count {
        // The last block may be smaller than `block_size`.
        let write_size: usize = (file_size
            .0
            .checked_rem(block_size as u64)
            .expect("block_size is 0"))
        .try_into()
        .expect("write_size overflow");
        if write_size == 0 {
            datagram.block
        } else {
            &datagram.block[0..write_size]
        }
    } else {
        datagram.block
    };

    // write the block to disk
    file.write_all(slice_to_write)?;

    Ok(())
}
