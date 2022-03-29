use crate::data::abstraction::log_group::LogGroup;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;
use crate::{byte_layout, reify};
use super::chunk_entry::ChunkEntry;

reify! {
    #[derive(Debug,Default,Clone)]
    pub struct Chunk {
        #[bytes_size=4]
        pub length: u32,
        #[bytes_size=8]
        pub timestamp_from: u64,
        #[bytes_size=8]
        pub timestamp_to: u64,
        #[bytes_size=4]
        pub entries_length: u32,
        pub entries: Vec<u8>,
    }
}

byte_layout! {
    Chunk
    value [length, u32, Big]
    value [timestamp_from, u64, Big]
    value [timestamp_to, u64, Big]
    value [entries_length, u32, Big]
    bytes_vec [entries, entries_length]
}

impl Transcoder<LogGroup> for Chunk {
    fn transcode(&self) -> Result<Box<LogGroup>, encoding_errors::TranscoderError<LogGroup>> {
        todo!("Implement transcoding for Chunk<->LogGroup")
    }
}