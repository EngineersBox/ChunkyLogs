use std::io::{Read, Write};
use flate2::bufread::ZlibDecoder;
use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;
use crate::compression::exception::compressor_exceptions;

type Byte = u8;

pub enum CompressionAction {
    COMPRESS,
    DECOMPRESS,
    IDLE,
}

pub struct Compressor {
    pub action: CompressionAction,
}

pub trait CompressionHandler {
    fn compress_slice(&mut self, data: &[Byte]) -> Result<Vec<Byte>, compressor_exceptions::CompressionError>;
    fn decompress_slice(&mut self, data: &[Byte]) -> Result<Vec<Byte>, compressor_exceptions::DecompressionError>;
}

impl CompressionHandler for Compressor {
    fn compress_slice(&mut self, data: &[Byte]) -> Result<Vec<Byte>, compressor_exceptions::CompressionError> {
        self.action = CompressionAction::COMPRESS;
        let mut encoder: ZlibEncoder<Vec<Byte>> = ZlibEncoder::new(Vec::new(), Compression::default());
        match encoder.write_all(data) {
            Err(e) => return Err(compressor_exceptions::CompressionError{
                message: e.to_string()
            }),
            Ok(()) => (),
        };
        return match encoder.finish() {
            Err(e) => Err(compressor_exceptions::CompressionError{
                message: e.to_string()
            }),
            Ok(bytes) => {
                self.action = CompressionAction::IDLE;
                Ok(bytes)
            },
        };
    }
    fn decompress_slice(&mut self, data: &[Byte]) -> Result<Vec<Byte>, compressor_exceptions::DecompressionError> {
        self.action = CompressionAction::DECOMPRESS;
        let mut decoder: ZlibDecoder<&[Byte]> = ZlibDecoder::new(data);
        let mut buffer: Vec<Byte> = Vec::new();
        return match decoder.read_to_end(&mut buffer) {
            Err(e) => Err(compressor_exceptions::DecompressionError{
                message: e.to_string()
            }),
            Ok(_) => {
                self.action = CompressionAction::IDLE;
                Ok(buffer)
            },
        };
    }
}

impl Compressor {
    pub fn new() -> Compressor {
        Compressor{
            action: CompressionAction::IDLE,
        }
    }
    pub fn compress_vec(&mut self, data: &Vec<Byte>) -> Result<Vec<Byte>, compressor_exceptions::CompressionError> {
        return self.compress_slice(data.as_slice());
    }
    pub fn decompress_vec(&mut self, data: &Vec<Byte>) -> Result<Vec<Byte>, compressor_exceptions::DecompressionError> {
        return self.decompress_slice(data.as_slice());
    }
}