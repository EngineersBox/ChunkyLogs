use super::exceptions::chunk_exceptions;
use std::convert::TryInto;
use std::fs::File;
use std::io::{BufReader, IoSliceMut, Read, Seek};
use crate::Compressor;

pub type Byte = u8;

#[derive(PartialEq, Eq, Debug)]
pub enum ChunkCompressionState {
    COMPRESSED,
    DECOMPRESSED,
}

pub struct Chunk {
    pub ts_from: u64,
    pub ts_to: u64,
    pub length: u32,
    pub data: Vec<Byte>,
    pub state: ChunkCompressionState,
}

const FROM_TIMESTAMP_OFFSET: usize = 0;
const TO_TIMESTAMP_OFFSET: usize = FROM_TIMESTAMP_OFFSET + 8;
const DATA_LENGTH_OFFSET: usize = TO_TIMESTAMP_OFFSET + 8;
const COMPRESSED_DATA_OFFSET: usize = DATA_LENGTH_OFFSET + 4;

impl Chunk {
    pub fn new() -> Chunk {
        Chunk {
            ts_from: 0,
            ts_to: 0,
            length: 0,
            data: Vec::new(),
            state: ChunkCompressionState::DECOMPRESSED,
        }
    }
    pub fn from_bytes_buffer(bytes: &mut BufReader<&File>) -> Result<Chunk, chunk_exceptions::ChunkProcessingException> {
        let mut new_chunk: Chunk = Chunk::new();

        // Read header
        let mut header: [Byte; COMPRESSED_DATA_OFFSET] = [0; COMPRESSED_DATA_OFFSET];
        match bytes.read(&mut header) {
            Ok(n) => {
                if n != COMPRESSED_DATA_OFFSET {
                    return Err(chunk_exceptions::ChunkProcessingException{
                        message: format!(
                            "Expected to read {} bytes for header, got {} bytes",
                            COMPRESSED_DATA_OFFSET,
                            n,
                        ),
                    })
                }
            },
            Err(e) => return Err(chunk_exceptions::ChunkProcessingException{
                message: e.to_string(),
            }),
        };

        // Process header
        let mut ts_from_bytes: [Byte; 8] = [0; 8];
        ts_from_bytes.copy_from_slice(&header[FROM_TIMESTAMP_OFFSET..TO_TIMESTAMP_OFFSET]);
        new_chunk.ts_from = u64::from_be_bytes(ts_from_bytes);

        let mut ts_to_bytes: [Byte; 8] = [0; 8];
        ts_to_bytes.copy_from_slice(&header[TO_TIMESTAMP_OFFSET..DATA_LENGTH_OFFSET]);
        new_chunk.ts_to = u64::from_be_bytes(ts_to_bytes);

        let mut length_bytes: [Byte; 4] = [0; 4];
        length_bytes.copy_from_slice(&header[DATA_LENGTH_OFFSET..COMPRESSED_DATA_OFFSET]);
        new_chunk.length = u32::from_be_bytes(length_bytes);

        // Read data
        let mut data: Vec<IoSliceMut> = Vec::with_capacity(new_chunk.length as usize);
        match bytes.read_vectored(&mut data) {
            Ok(n) => {
                if n != COMPRESSED_DATA_OFFSET {
                    return Err(chunk_exceptions::ChunkProcessingException{
                        message: format!(
                            "Expected to read {} bytes for data, got {} bytes",
                            COMPRESSED_DATA_OFFSET,
                            n,
                        ),
                    })
                }
            },
            Err(e) => return Err(chunk_exceptions::ChunkProcessingException{
                message: e.to_string(),
            }),
        };

        // Process data
        let mut data_bytes: Vec<Byte> = Vec::new();
        for byte_slice in data.iter() {
            for byte_item in byte_slice.iter() {
                data_bytes.push(*byte_item);
            }
        }
        let compress_data_actual_length: u32 = (data_bytes.len() - COMPRESSED_DATA_OFFSET) as u32;
        if new_chunk.length != compress_data_actual_length {
            return Err(chunk_exceptions::ChunkProcessingException{
                message: format!(
                    "Compressed data was of length {}, expected {}",
                    compress_data_actual_length,
                    new_chunk.length
                ),
            });
        }
        new_chunk.data = data_bytes;
        new_chunk.state = ChunkCompressionState::COMPRESSED;

        return Ok(new_chunk);
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

        let mut ts_from_bytes: [Byte; 8] = [0; 8];
        ts_from_bytes.copy_from_slice(&bytes_slice[FROM_TIMESTAMP_OFFSET..TO_TIMESTAMP_OFFSET]);
        new_chunk.ts_from = u64::from_be_bytes(ts_from_bytes);

        let mut ts_to_bytes: [Byte; 8] = [0; 8];
        ts_to_bytes.copy_from_slice(&bytes_slice[TO_TIMESTAMP_OFFSET..DATA_LENGTH_OFFSET]);
        new_chunk.ts_to = u64::from_be_bytes(ts_to_bytes);

        let mut length_bytes: [Byte; 4] = [0; 4];
        length_bytes.copy_from_slice(&bytes_slice[DATA_LENGTH_OFFSET..COMPRESSED_DATA_OFFSET]);
        new_chunk.length = u32::from_be_bytes(length_bytes);

        let compress_data_actual_length: u32 = (bytes.len() - COMPRESSED_DATA_OFFSET) as u32;
        if new_chunk.length != compress_data_actual_length {
            return Err(chunk_exceptions::ChunkProcessingException{
                message: format!(
                    "Compressed data was of length {}, expected {}",
                    compress_data_actual_length,
                    new_chunk.length
                ),
            });
        }
        new_chunk.data = bytes_slice[COMPRESSED_DATA_OFFSET..].to_vec();
        new_chunk.state = ChunkCompressionState::COMPRESSED;
        return Ok(new_chunk);
    }
    pub fn compress(&mut self) -> Result<(), chunk_exceptions::ChunkProcessingException> {
        let mut compressor: Compressor = Compressor::new();
        return match compressor.compress_vec(&self.data) {
            Ok(compressed) => {
                self.data = compressed;
                self.length = self.data.len() as u32;
                self.state = ChunkCompressionState::COMPRESSED;
                return Ok(());
            },
            Err(e) => Err(chunk_exceptions::ChunkProcessingException{
                message: e.message
            }),
        };
    }
    pub fn to_be_bytes(&mut self) -> Result<Vec<Byte>, chunk_exceptions::ChunkProcessingException> {
        if self.state == ChunkCompressionState::DECOMPRESSED {
            self.compress();
        }
        todo!("Implement Chunk::to_be_bytes()")
    }
    pub fn to_le_bytes(&mut self) -> Result<Vec<Byte>, chunk_exceptions::ChunkProcessingException> {
        if self.state == ChunkCompressionState::DECOMPRESSED {
            self.compress();
        }
        todo!("Implement Chunk::to_le_bytes()")
    }
}