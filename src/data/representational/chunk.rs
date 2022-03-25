use crate::data::abstraction::log_group::LogGroup;
use crate::reflective_attributes;
use crate::encoding::decoder::Decoder;
use crate::encoding::encoder::Encoder;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

reflective_attributes! {
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