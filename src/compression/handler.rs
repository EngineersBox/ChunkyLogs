use crate::data::chunk::chunk::{Chunk,Byte};

use std::io::{Read, Write};
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;
use crate::compression::exception::compressor_exceptions;

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
    pub fn compress(&self, chunk: &Chunk) -> Result<Vec<Byte>, compressor_exceptions::CompressionError> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        match encoder.write_all(chunk.decompressed_data.as_bytes()) {
            Err(e) => return Err(compressor_exceptions::CompressionError{
                message: e.to_string()
            }),
            Ok(()) => (),
        };
        return match encoder.finish() {
            Err(e) => Err(compressor_exceptions::CompressionError{
                message: e.to_string()
            }),
            Ok(bytes) => Ok(bytes),
        };
    }
    pub fn decompress(&self, data: &Vec<Byte>) -> Result<Chunk, compressor_exceptions::DecompressionError> {
        let mut decoder = GzDecoder::new(data.as_slice());
        let mut chunk: Chunk = Chunk::new();
        return match decoder.read_to_string(&mut chunk.decompressed_data) {
            Err(e) => Err(compressor_exceptions::DecompressionError{
                message: e.to_string()
            }),
            Ok(_) => Ok(chunk),
        };
    }
}