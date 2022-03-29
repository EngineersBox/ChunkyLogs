use crate::ChunkStore;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

pub struct LogStore {

}

impl Transcoder<ChunkStore> for LogStore {
    fn transcode(&self) -> Result<Box<ChunkStore>, encoding_errors::TranscoderError<ChunkStore>> {
        todo!("Implement transcoding for LogStore->ChunkStore")
    }
}