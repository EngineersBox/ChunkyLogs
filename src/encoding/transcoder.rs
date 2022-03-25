use crate::encoding::errors::encoding_errors;

pub trait Transcoder<T> {
    fn transcode(&self) -> Result<Box<T>, encoding_errors::TranscoderError<T>>;
}