use crate::{byte_layout, reify};

reify!{
    #[derive(Debug,Default)]
    pub struct ChunkOffsets {
        #[byte_size=4]
        pub sector_index: u32,
        #[byte_size=1]
        pub start_or_end_flag: u8,
        #[byte_size=4]
        pub sector_offset: u32,
    }
}

byte_layout!{
    ChunkOffsets
    value [sector_index, u32, Big]
    value [start_or_end_flag, u8]
    value [sector_offset, u32, Big]
}