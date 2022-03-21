use crate::data::chunk::chunk::{Chunk,Byte};

use std::io::{Read, Write};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;
use crate::compression::exception::compressor_exceptions::CompressionError;
use crate::compression::exception::compressor_exceptions::DecompressionError;

enum CompressionAction {
    COMPRESS,
    DECOMPRESS,
    IDLE,
}

struct Compressor {
    pub action: CompressionAction,
}

impl Compressor {
    pub fn new() -> Compressor {
        Compressor{
            action: CompressionAction::IDLE,
        }
    }
    pub fn compress(&self, chunk: &Chunk) -> Result<Vec<Byte>, CompressionError> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        match encoder.write_all(chunk.decompressed_data.as_bytes()) {
            Err(e) => return Err(CompressionError{
                message: e.to_string()
            }),
            Ok(()) => (),
        };
        return match encoder.finish() {
            Err(e) => Err(CompressionError{
                message: e.to_string()
            }),
            Ok(bytes) => Ok(bytes),
        };
    }
    pub fn decompress(&self, data: &Vec<Byte>) -> Result<Chunk, DecompressionError> {
        let mut decoder = GzDecoder::new(data.as_slice());
        let mut chunk: Chunk = Chunk::new();
        return match decoder.read_to_string(&mut chunk.decompressed_data) {
            Err(e) => Err(DecompressionError{
                message: e.to_string()
            }),
            Ok(_) => Ok(chunk),
        };
    }
}