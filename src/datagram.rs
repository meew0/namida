#[derive(Debug, Clone, Copy)]
pub struct Header {
    pub block_index: u32,
    pub block_type: u16,
}

#[derive(Debug, Clone, Copy)]
pub struct View<'a> {
    pub header: Header,
    pub block: &'a [u8],
}

impl<'a> View<'a> {
    pub fn parse(slice: &'a [u8]) -> Self {
        let header = Header {
            block_index: u32::from_be_bytes(slice[0..4].try_into().unwrap()),
            block_type: u16::from_be_bytes(slice[4..6].try_into().unwrap()),
        };

        Self {
            header,
            block: &slice[6..],
        }
    }

    pub fn write_to(&self, slice: &mut [u8]) {
        assert_eq!(slice.len(), 6 + self.block.len());
        slice[0..4].copy_from_slice(&self.header.block_index.to_be_bytes());
        slice[4..6].copy_from_slice(&self.header.block_type.to_be_bytes());
        slice[6..].copy_from_slice(self.block);
    }
}
