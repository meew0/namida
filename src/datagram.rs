use bincode::{de::read::BorrowReader, enc::write::Writer};

use crate::types::BlockIndex;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub block_index: BlockIndex,
    pub block_type: BlockType,
}

impl Header {
    pub const SIZE: usize = 6;
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum BlockType {
    Original,
    Final,
    Retransmission,
}

impl TryFrom<u16> for BlockType {
    fn try_from(value: u16) -> Result<Self, ()> {
        if value == BlockType::Original as u16 {
            Ok(BlockType::Original)
        } else if value == BlockType::Final as u16 {
            Ok(BlockType::Final)
        } else if value == BlockType::Retransmission as u16 {
            Ok(BlockType::Retransmission)
        } else {
            Err(())
        }
    }

    type Error = ();
}

#[derive(Debug, Clone, Copy)]
pub struct View<'v> {
    pub header: Header,
    pub block: &'v [u8],
}

impl<'v> bincode::Encode for View<'v> {
    fn encode<E: bincode::enc::Encoder>(
        &self,
        encoder: &mut E,
    ) -> Result<(), bincode::error::EncodeError> {
        bincode::Encode::encode(&self.header.block_index.0, encoder)?;
        bincode::Encode::encode(&(self.header.block_type as u16), encoder)?;
        encoder.writer().write(self.block)?;

        Ok(())
    }
}

impl<'v, 'de: 'v> bincode::BorrowDecode<'de> for View<'v> {
    fn borrow_decode<D: bincode::de::BorrowDecoder<'de>>(
        decoder: &mut D,
    ) -> Result<Self, bincode::error::DecodeError> {
        let block_index = BlockIndex(bincode::BorrowDecode::borrow_decode(decoder)?);
        let block_type_value: u16 = bincode::BorrowDecode::borrow_decode(decoder)?;
        let block_type = BlockType::try_from(block_type_value).or(Err(
            bincode::error::DecodeError::UnexpectedVariant {
                type_name: "BlockType",
                allowed: &bincode::error::AllowedEnumVariants::Range { min: 0, max: 2 },
                found: u32::from(block_type_value),
            },
        ))?;
        let block = decoder
            .borrow_reader()
            .take_bytes(crate::common::BLOCK_SIZE as usize)?;

        Ok(Self {
            header: Header {
                block_index,
                block_type,
            },
            block,
        })
    }
}
