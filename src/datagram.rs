use crate::types::BlockIndex;

#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub block_index: BlockIndex,
    pub block_type: BlockType,
}

#[derive(Debug, Clone, Copy)]
#[repr(u16)]
pub enum BlockType {
    Original = 'O' as u16,
    Final = 'X' as u16,
    Retransmission = 'R' as u16,
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
pub struct View<'a> {
    pub header: Header,
    pub block: &'a [u8],
}

impl<'a> View<'a> {
    /// Parses the given byte slice into a datagram `View`. Returns `None` if the input data
    /// specifies an invalid block type.
    ///
    /// # Panics
    /// Panics if conversions of dynamically-sized to statically-sized slices fail.
    #[must_use]
    pub fn parse(slice: &'a [u8]) -> Option<Self> {
        assert!(slice.len() > 5);

        let Ok(block_type) = BlockType::try_from(u16::from_be_bytes(
            slice[4..6]
                .try_into()
                .expect("block type slice should be the correct size"),
        )) else {
            return None;
        };

        let header = Header {
            block_index: BlockIndex(u32::from_be_bytes(
                slice[0..4]
                    .try_into()
                    .expect("block index slice should be the correct size"),
            )),
            block_type,
        };

        Some(Self {
            header,
            block: &slice[6..],
        })
    }

    /// Writes a byte representation of this datagram into the given slice. The slice must have
    /// exactly `self.block.len() + 6` elements.
    ///
    /// # Panics
    /// Panics if the current slice length is less than 6 bytes away from overflowing.
    #[allow(clippy::missing_asserts_for_indexing)] // we assert the length exactly
    pub fn write_to(&self, slice: &mut [u8]) {
        assert_eq!(
            slice.len(),
            self.block
                .len()
                .checked_add(6)
                .expect("datagram length overflow")
        );
        slice[0..4].copy_from_slice(&self.header.block_index.0.to_be_bytes());
        slice[4..6].copy_from_slice(&(self.header.block_type as u16).to_be_bytes());
        slice[6..].copy_from_slice(self.block);
    }
}
