use super::exceptions::encoding_exceptions;

pub trait Decoder {
    fn decode(raw: &Vec<u8>) -> Result<Box<Self>, encoding_exceptions::DecoderError>;
}