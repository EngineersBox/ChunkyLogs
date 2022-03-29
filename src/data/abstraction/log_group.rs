use crate::Chunk;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

pub struct LogGroup {

}

impl Transcoder<Chunk> for LogGroup {
    fn transcode(&self) -> Result<Box<Chunk>, encoding_errors::TranscoderError<Chunk>> {
        todo!("Implement transcoding for LogGroup->Chunk")
    }
}