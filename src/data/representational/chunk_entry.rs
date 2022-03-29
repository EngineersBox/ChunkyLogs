use crate::data::abstraction::log_entry::LogEntry;
use crate::{byte_layout, reify};
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

reify!{
    #[derive(Debug,Default,Clone)]
    pub struct ChunkEntry {
        #[bytes_size=8]
        pub timestamp: u64,
        #[bytes_size=1]
        pub action: u8,
        pub target: Vec<u8>,
        pub message: Vec<u8>,
    }
}

byte_layout!{
    ChunkEntry
    value [timestamp, u64, Big]
    value [action, u8]
    bytes_vec_null_term [target]
    bytes_vec_null_term [message]
}

impl Transcoder<LogEntry> for ChunkEntry {
    fn transcode(&self) -> Result<Box<LogEntry>, encoding_errors::TranscoderError<LogEntry>> {
        todo!("Implement transcoding for ChunkEntry<->LogEntry")
    }
}