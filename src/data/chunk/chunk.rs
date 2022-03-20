use super::exceptions::chunk_exceptions;

use std::convert::TryInto;

pub type Byte = u8;

pub struct Chunk {
    pub ts_from: u64,
    pub ts_to: u64,
    pub length: u32,
    pub compressed_data: Vec<Byte>,
    pub decompressed_data: String,
}

static FROM_TIMESTAMP_OFFSET: usize = 0;
static TO_TIMESTAMP_OFFSET: usize = FROM_TIMESTAMP_OFFSET + 64;
static DATA_LENGTH_OFFSET: usize = TO_TIMESTAMP_OFFSET + 64;
static COMPRESSED_DATA_OFFSET: usize = DATA_LENGTH_OFFSET + 32;

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            ts_from: 0,
            ts_to: 0,
            length: 0,
            compressed_data: vec!{},
            decompressed_data: String::new(),
        }
    }
    pub fn from_bytes(bytes: &Vec<Byte>) -> Result<Chunk, chunk_exceptions::ChunkProcessingException> {
        if bytes.len() <= COMPRESSED_DATA_OFFSET - 1 {
            return Err(chunk_exceptions::ChunkProcessingException{
                message: format!(
                    "Byte array must be at least {}",
                    COMPRESSED_DATA_OFFSET - 1
                ),
            });
        }
        let mut new_chunk: Chunk = Chunk::new();
        let bytes_slice: &[Byte] = bytes.as_slice();

        let mut ts_from_bytes: [u8; 8] = [0; 8];
        ts_from_bytes.copy_from_slice(&bytes_slice[FROM_TIMESTAMP_OFFSET..TO_TIMESTAMP_OFFSET]);
        new_chunk.ts_from = u64::from_ne_bytes(ts_from_bytes);

        let mut ts_to_bytes: [u8; 8] = [0; 8];
        ts_to_bytes.copy_from_slice(&bytes_slice[TO_TIMESTAMP_OFFSET..DATA_LENGTH_OFFSET]);
        new_chunk.ts_to = u64::from_ne_bytes(ts_to_bytes);

        let mut length_bytes: [u8; 4] = [0; 4];
        length_bytes.copy_from_slice(&bytes_slice[DATA_LENGTH_OFFSET..COMPRESSED_DATA_OFFSET]);
        new_chunk.length = u32::from_ne_bytes(length_bytes);

        let compress_data_actual_length: u32 = (bytes.len() - COMPRESSED_DATA_OFFSET).try_into().unwrap();
        if new_chunk.length != compress_data_actual_length {
            return Err(chunk_exceptions::ChunkProcessingException{
                message: format!(
                    "Compressed data was of length {}, expected {}",
                    compress_data_actual_length,
                    new_chunk.length
                ),
            });
        }
        new_chunk.compressed_data = bytes_slice[COMPRESSED_DATA_OFFSET..].to_vec();
        return Ok(new_chunk);
    }
    pub fn to_bytes(&self) -> Vec<Byte> {
        // TODO: Implement this
        return vec!();
    }
}