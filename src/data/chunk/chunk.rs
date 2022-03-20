type Byte = u8;

pub struct Chunk {
    pub ts_from: u64,
    pub ts_to: u64,
    pub length: u32,
    pub compressed_data: Vec<Byte>,
}

impl Chunk {
    fn new() -> Chunk {
        Chunk {
            ts_from: 0,
            ts_to: 0,
            length: 0,
            compressed_data: vec!{},
        }
    }
}