#[derive(Debug, Clone, PartialEq)]
pub struct DataBlock {
    pub id: u64,
    pub data: Vec<u8>,
}

impl DataBlock {
    pub fn new(id: u64, data: Vec<u8>) -> Self {
        Self { id, data }
    }
}

pub struct BlockMetadata {
    pub size: usize,
    pub checksum: u64,
}
