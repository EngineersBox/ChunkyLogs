use crate::data::abstraction::log_store::LogStore;
use crate::{byte_layout, reify};
use crate::data::representational::chunk::Chunk;
use super::chunk_store_header::ChunkStoreHeader;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

reify!{
    #[derive(Debug,Default)]
    pub struct ChunkStore {
        pub header: ChunkStoreHeader,
        #[byte_size=8]
        pub chunks_length: u64,
        pub chunks: Vec<Chunk>,
    }
}

byte_layout!{
    ChunkStore
    composite [header, ChunkStoreHeader]
    value [chunks_length, u64, Big]
    composite_vec [chunks, chunks_length, Chunk]
}

impl Transcoder<LogStore> for ChunkStore {
    fn transcode(&self) -> Result<Box<LogStore>, encoding_errors::TranscoderError<LogStore>> {
        todo!("Implement transcoding for ChunkStore<->LogStore")
    }
}