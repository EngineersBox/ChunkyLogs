use crate::encoding::errors::encoding_errors;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>, encoding_errors::EncoderError<Vec<u8>>>;
}