use std::io::{Read, Seek, SeekFrom};

use crate::{
    datagram::{self, BlockType},
    types::BlockIndex,
};

use super::Session;

pub fn build_datagram<'a>(
    session: &mut Session,
    block_index: BlockIndex,
    block_type: BlockType,
    block_buffer: &'a mut [u8],
) -> anyhow::Result<datagram::View<'a>> {
    assert_eq!(block_buffer.len(), session.properties.block_size.0 as usize);

    let file = session.transfer.file.as_mut().unwrap();
    file.seek(SeekFrom::Start(
        (session.properties.block_size.0 * (block_index - BlockIndex(1)).0) as u64,
    ))?;

    let read_amount = file.read(block_buffer)?;
    if read_amount < session.properties.block_size.0 as usize
        && block_index < session.properties.block_count
    {
        println!(
            "WARNING: only read {} instead of {} bytes for block {} out of {}",
            read_amount,
            session.properties.block_size,
            block_index.0,
            session.properties.block_count.0
        )
    }

    Ok(datagram::View {
        header: datagram::Header {
            block_index,
            block_type,
        },
        block: block_buffer,
    })
}
