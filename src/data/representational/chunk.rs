use crate::data::abstraction::log_group::LogGroup;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;
use crate::{byte_layout, reify};
use super::chunk_entry::ChunkEntry;

reify! {
    #[derive(Debug,Default,Clone)]
    pub struct Chunk {
        #[bytes_size=8]
        pub timestamp_from: u64,
        #[bytes_size=8]
        pub timestamp_to: u64,
        #[bytes_size=4]
        pub length: u32,
        pub entries: Vec<ChunkEntry>,
    }
}

byte_layout! {
    Chunk
    value [timestamp_from, u64, Big]
    value [timestamp_to, u64, Big]
    value [length, u32, Big]
    composite_vec [entries, length, ChunkEntry]
}

impl Transcoder<LogGroup> for Chunk {
    fn transcode(&self) -> Result<Box<LogGroup>, encoding_errors::TranscoderError<LogGroup>> {
        todo!("Implement transcoding for Chunk<->LogGroup")
    }
}