use crate::reflective_attributes;

reflective_attributes!{
    pub struct ChunkOffsets {
        #[byte_size=4]
        pub sector_index: u32,
        #[byte_size=1]
        pub start_or_end_flag: u8,
        #[byte_size=4]
        pub sector_offset: u32,
    }
}