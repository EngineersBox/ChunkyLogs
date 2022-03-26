use std::fs::File;
use std::io;
use memmap::{Mmap, MmapOptions};
use crate::data::abstraction::log_store::LogStore;
use crate::reify;
use super::chunk_store_header::ChunkStoreHeader;
use crate::encoding::decoder::Decoder;
use crate::encoding::encoder::Encoder;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

reify!{
    pub struct ChunkStore {
        pub header: ChunkStoreHeader,
        #[byte_size=8]
        pub chunks_length: u64,
        pub chunks: Vec<u8>,
    }
}

impl Decoder for ChunkStore {
    fn decode(from: &Vec<u8>) -> Result<Box<Self>, encoding_errors::DecoderError<Vec<u8>>> {
        todo!("Implement decoding for ChunkStore")
    }
}

impl Encoder for ChunkStore {
    fn encode(&self) -> Result<Vec<u8>, encoding_errors::EncoderError<Vec<u8>>> {
        todo!("Implement decoding for ChunkStore")
    }
}

impl Transcoder<LogStore> for ChunkStore {
    fn transcode(&self) -> Result<Box<LogStore>, encoding_errors::TranscoderError<LogStore>> {
        todo!("Implement transcoding for ChunkStore<->LogStore")
    }
}