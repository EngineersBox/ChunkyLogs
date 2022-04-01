use crate::data::abstraction::log_group::LogGroup;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;
use crate::{byte_layout, reify};

reify! {
    #[derive(Debug,Default,Clone)]
    pub struct Chunk {
        #[bytes_size=4]
        pub length: u32,
        #[bytes_size=8]
        pub timestamp_from: u64,
        #[bytes_size=8]
        pub timestamp_to: u64,
        #[bytes_size=4]
        pub entries_length: u32,
        pub entries: Vec<u8>,
    }
}

byte_layout! {
    Chunk
    value [length, u32, Big]
    value [timestamp_from, u64, Big]
    value [timestamp_to, u64, Big]
    value [entries_length, u32, Big]
    bytes_vec [entries, entries_length]
}

impl Chunk {
    pub fn header_length() -> Result<u32, std::io::Error> {
        let attribute: Option<String> = match Self::get_field_attribute("length") {
            Ok(v) => v,
            Err(e) => return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                e.to_string(),
            )),
        };
        if attribute.is_none() {
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Field has no attributes",
            ))
        }
        let attr_string: String = attribute.unwrap();
        let attribute_split: Vec<&str> = attr_string.split("=").collect::<Vec<&str>>();
        let split_val: Option<&&str> = attribute_split.get(1);
        if split_val.is_none() {
            return Ok(0);
        }
        return match split_val.unwrap().parse::<u32>() {
            Ok(v) => Ok(v),
            Err(_) => Ok(0),
        };
    }
}

impl Transcoder<LogGroup> for Chunk {
    fn transcode(&self) -> Result<Box<LogGroup>, encoding_errors::TranscoderError<LogGroup>> {
        todo!("Implement transcoding for Chunk<->LogGroup")
    }
}