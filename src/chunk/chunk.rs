type Byte u8;

pub struct Chunk {
    pub ts_from: u64;
    pub ts_to: u64;
    pub length: u32;
    pub compressed_data: Vec<Byte>;
}