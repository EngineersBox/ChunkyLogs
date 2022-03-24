use super::exceptions::encoding_exceptions;

pub trait Encoder {
    fn encode(&self) -> Result<Vec<u8>, encoding_exceptions::EncoderError>;
}