use crate::types::BlockIndex;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub block_index: BlockIndex,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum BlockType {
    Normal = 'O' as u16,
    Final = 'X' as u16,
    Retransmission = 'R' as u16,
}

impl TryFrom<u16> for BlockType {
    fn try_from(value: u16) -> Result<Self, ()> {
        if value == BlockType::Normal as u16 {
            Ok(BlockType::Normal)
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
pub struct View<'a> {
    pub header: Header,
    pub block: &'a [u8],
}

impl<'a> View<'a> {
    pub fn parse(slice: &'a [u8]) -> Self {
        let header = Header {
            block_index: BlockIndex(u32::from_be_bytes(slice[0..4].try_into().unwrap())),
            block_type: BlockType::try_from(u16::from_be_bytes(slice[4..6].try_into().unwrap()))
                .expect("invalid block type"),
        };

        Self {
            header,
            block: &slice[6..],
        }
    }

    pub fn write_to(&self, slice: &mut [u8]) {
        assert_eq!(slice.len(), 6 + self.block.len());
        slice[0..4].copy_from_slice(&self.header.block_index.0.to_be_bytes());
        slice[4..6].copy_from_slice(&(self.header.block_type as u16).to_be_bytes());
        slice[6..].copy_from_slice(self.block);
    }
}
