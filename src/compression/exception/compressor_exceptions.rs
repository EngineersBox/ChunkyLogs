use std::fmt;

pub enum CompressorError {
    CompressionError(CompressionError),
    DecompressionError(DecompressionError)
}

pub struct CompressionError {
    pub message: String
}

impl fmt::Display for CompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occurred during decompression of a BLOB: {}", self.message)
    }
}

pub struct DecompressionError {
    pub message: String
}

impl fmt::Display for DecompressionError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "an error occurred while compressing a chunk: {}", self.message)
    }
}