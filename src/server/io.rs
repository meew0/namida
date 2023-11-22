use std::io::{Read, Seek, SeekFrom};

use crate::datagram;

use super::{Parameter, Session};

pub fn build_datagram<'a>(
    session: &mut Session,
    parameter: &Parameter,
    block_index: u32,
    block_type: u16,
    block_buffer: &'a mut [u8],
) -> anyhow::Result<datagram::View<'a>> {
    assert_eq!(block_buffer.len(), parameter.block_size as usize);

    let file = session.transfer.file.as_mut().unwrap();
    file.seek(SeekFrom::Start(
        (parameter.block_size * block_index.checked_sub(1).unwrap()) as u64,
    ))?;

    let read_amount = file.read(block_buffer)?;
    if read_amount < parameter.block_size as usize && block_index < parameter.block_count {
        println!(
            "WARNING: only read {} instead of {} bytes for block {} out of {}",
            read_amount, parameter.block_size, block_index, parameter.block_count
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
