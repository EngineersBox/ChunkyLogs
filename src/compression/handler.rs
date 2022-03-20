use crate::data::chunk::chunk::{Chunk,Byte};

use std::io;
use std::io::Read;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;

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
    pub fn compress(&self, chunk: &Chunk) -> Result<Vec<Byte>, io::Error> {
        let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());
        return encoder.finish();
    }
    pub fn decompress(&self, data: &Vec<Byte>) -> Result<Chunk, io::Error> {
        let mut decoder = GzDecoder::new(data.as_slice());
        let mut chunk: Chunk = Chunk::new();
        decoder.read_to_string(&mut chunk.decompressed_data).unwrap();
        return Ok(chunk);
    }
}