use crate::data::abstraction::log_entry::LogEntry;
use crate::reflective_attributes;
use crate::encoding::decoder::Decoder;
use crate::encoding::encoder::Encoder;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

reflective_attributes!{
    pub struct ChunkEntry {
        #[bytes_size=8]
        pub timestamp: u64,
        #[bytes_size=1]
        pub action: u8,
        pub target: Vec<u8>,
        pub message: Vec<u8>,
    }
}

impl Decoder for ChunkEntry {
    fn decode(from: &Vec<u8>) -> Result<Box<Self>, encoding_errors::DecoderError<Vec<u8>>> {
        todo!("Implement decoding for ChunkEntry")
    }
}

impl Encoder for ChunkEntry {
    fn encode(&self) -> Result<Vec<u8>, encoding_errors::EncoderError<Vec<u8>>> {
        todo!("Implement decoding for ChunkEntry")
    }
}

impl Transcoder<LogEntry> for ChunkEntry {
    fn transcode(&self) -> Result<Box<LogEntry>, encoding_errors::TranscoderError<LogEntry>> {
        todo!("Implement transcoding for ChunkEntry<->LogEntry")
    }
}