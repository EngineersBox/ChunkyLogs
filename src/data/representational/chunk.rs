use crate::data::abstraction::log_group::LogGroup;
use crate::encoding::decoder::Decoder;
use crate::encoding::encoder::Encoder;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;
use crate::{byte_layout, reify};
use super::chunk_entry::ChunkEntry;

reify! {
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
    value [timestamp_from, {nom::number::complete::be_u64::<I,E>}]
    value [timestamp_to, {nom::number::complete::be_u64::<I,E>}]
    value [length, {nom::number::complete::be_u32::<I,E>}]
    composite_vec [entries, length, ChunkEntry]
}

impl Decoder for Chunk {
    fn decode(from: &Vec<u8>) -> Result<Box<Self>, encoding_errors::DecoderError<Vec<u8>>> {
        todo!("Implement decoding for Chunk")
    }
}

impl Encoder for Chunk {
    fn encode(&self) -> Result<Vec<u8>, encoding_errors::EncoderError<Vec<u8>>> {
        todo!("Implement decoding for Chunk")
    }
}

impl Transcoder<LogGroup> for Chunk {
    fn transcode(&self) -> Result<Box<LogGroup>, encoding_errors::TranscoderError<LogGroup>> {
        todo!("Implement transcoding for Chunk<->LogGroup")
    }
}