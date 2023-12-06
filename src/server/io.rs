use std::io::{Read, Seek, SeekFrom};

use crate::{
    datagram::{self, BlockType},
    types::BlockIndex,
};

use super::Session;

/// Tries to read the given block from the currently open file. If successful, a datagram view is
/// created based on the data read. The caller must supply a buffer exactly big enough to fit one
/// block.
///
/// # Errors
/// Returns an error on I/O failure.
///
/// # Panics
/// Panics if no file is present, or if the seek position overflows.
pub fn build_datagram<'a>(
    session: &mut Session,
    block_index: BlockIndex,
    block_type: BlockType,
    block_buffer: &'a mut [u8],
) -> anyhow::Result<datagram::View<'a>> {
    assert_eq!(block_buffer.len(), crate::common::BLOCK_SIZE as usize);

    // move the file pointer to the appropriate location
    let file = session
        .transfer
        .file
        .as_mut()
        .expect("a file should be present");
    file.seek(SeekFrom::Start(
        u64::from(crate::common::BLOCK_SIZE)
            .checked_mul(u64::from((block_index.safe_sub(BlockIndex(1))).0))
            .expect("file position overflow"),
    ))?;

    // try to read in the block
    let read_amount = file.read(block_buffer)?;
    if read_amount < crate::common::BLOCK_SIZE as usize
        && block_index < session.properties.block_count
    {
        println!(
            "WARNING: only read {} instead of {} bytes for block {} out of {}",
            read_amount,
            crate::common::BLOCK_SIZE,
            block_index.0,
            session.properties.block_count.0
        );
    }

    // build the datagram & return success
    Ok(datagram::View {
        header: datagram::Header {
            block_index,
            block_type,
        },
        block: block_buffer,
    })
}
