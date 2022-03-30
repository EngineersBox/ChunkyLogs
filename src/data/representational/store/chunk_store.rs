use crate::data::abstraction::log_store::LogStore;
use crate::{byte_layout, reify};
use crate::data::representational::chunk::Chunk;
use super::chunk_store_header::ChunkStoreHeader;
use crate::encoding::errors::encoding_errors;
use crate::encoding::transcoder::Transcoder;

reify!{
    #[derive(Debug,Default)]
    pub struct ChunkStore {
        pub header: ChunkStoreHeader,
        #[byte_size=8]
        pub chunks_length: u64,
        pub chunks: Vec<Chunk>,
    }
}

byte_layout!{
    ChunkStore
    composite [header, ChunkStoreHeader]
    value [chunks_length, u64, Big]
    composite_vec [chunks, chunks_length, Chunk]
}

impl ChunkStore {
    pub fn chunks_length_bytes_length() -> Result<u64, std::io::Error> {
        let attribute: Option<String> = match Self::get_field_attribute("chunks_length") {
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
            return Err(std::io::Error::new(
                std::io::ErrorKind::NotFound,
                "Field has no attributes",
            ));
        }
        return match split_val.unwrap().parse::<u64>() {
            Ok(v) => Ok(v),
            Err(e) => Err(std::io::Error::new(
                std::io::ErrorKind::InvalidData,
                "Could not parse field",
            )),
        };
    }
}

impl Transcoder<LogStore> for ChunkStore {
    fn transcode(&self) -> Result<Box<LogStore>, encoding_errors::TranscoderError<LogStore>> {
        todo!("Implement transcoding for ChunkStore<->LogStore")
    }
}