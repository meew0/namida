use std::io::{Seek, SeekFrom, Write};

use crate::{
    datagram,
    types::{BlockIndex, FileSize},
};

pub fn accept_block(
    datagram: datagram::View,
    block_count: BlockIndex,
    file_size: FileSize,
    file: &mut std::fs::File,
) -> anyhow::Result<()> {
    let block_size = datagram.block.len();

    let offset = block_size as u64 * (datagram.header.block_index - BlockIndex(1)).0 as u64;
    file.seek(SeekFrom::Start(offset))?;

    let slice_to_write = if datagram.header.block_index == block_count {
        let write_size: usize = (file_size.0 % block_size as u64).try_into().unwrap();
        if write_size == 0 {
            datagram.block
        } else {
            &datagram.block[0..write_size]
        }
    } else {
        datagram.block
    };
    file.write_all(slice_to_write)?;

    Ok(())
}
