use super::exceptions::encoding_exceptions;

pub trait Transcoder<T> {
    fn transcode(&self) -> Result<T, encoding_exceptions::TranscoderError>;
}