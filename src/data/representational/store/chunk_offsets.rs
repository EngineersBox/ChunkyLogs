use crate::{byte_layout, reify};

reify!{
    #[derive(Debug,Default,Clone)]
    pub struct ChunkOffsets {
        #[byte_size=4]
        pub sector_index: u32,
        #[byte_size=2]
        pub sector_offset: u16,
    }
}

byte_layout!{
    ChunkOffsets
    value [sector_index, u32, Big]
    value [sector_offset, u16, Big]
}