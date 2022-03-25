use crate::reflective_attributes;
use super::chunk_offsets::ChunkOffsets;

reflective_attributes!{
    pub struct ChunkStoreHeader {
        #[byte_size=8]
        pub length: u64,
        #[byte_size=8]
        pub sector_size: u64,
        #[byte_size=4]
        pub chunk_count: u32,
        #[byte_size=4]
        pub chunk_offsets_length: u32,
        pub chunk_offsets: Vec<ChunkOffsets>,
    }
}