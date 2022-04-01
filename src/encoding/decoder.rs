use crate::encoding::errors::encoding_errors;

pub trait Decoder {
    fn decode(from: &Vec<u8>) -> Result<Box<Self>, encoding_errors::DecoderError<Vec<u8>>>;
}