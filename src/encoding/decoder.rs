use super::exceptions::encoding_exceptions;

pub trait Decoder {
    fn decode(&mut self, raw: &Vec<u8>) -> Result<(), encoding_exceptions::DecoderError>;
}