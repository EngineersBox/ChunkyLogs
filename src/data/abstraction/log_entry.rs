use crate::ChunkEntry;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

pub struct LogEntry {

}

impl Transcoder<ChunkEntry> for LogEntry {
    fn transcode(&self) -> Result<Box<ChunkEntry>, encoding_errors::TranscoderError<ChunkEntry>> {
        todo!("Implement transcoding for LogEntry->ChunkEntry")
    }
}