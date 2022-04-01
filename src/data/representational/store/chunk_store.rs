use std::borrow::Borrow;
use std::fs::{File, OpenOptions};
use std::io::{BufReader, Error, Read, Seek};
use nom::AsBytes;
use crate::data::abstraction::log_store::LogStore;
use crate::{byte_layout, ChunkOffsets, reify};
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
        pub latest_chunk: Chunk,
        // TODO: Add a field to indicate chunk compression scheme
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
    pub fn read_from_file(path: &str) -> Result<ChunkStore, Error> {
        let mut store: ChunkStore = ChunkStore::default();
        let file: File = OpenOptions::new()
            .read(true)
            .write(false)
            .create(false)
            .open(path)?;
        let mut buf_reader: BufReader<&File> = BufReader::new(&file);
        store.header.read_from_file(&mut buf_reader)?;
        let mut length_bytes: Vec<u8> = Vec::with_capacity(8usize);
        buf_reader.get_ref()
            .take(8)
            .read_to_end(&mut length_bytes)?;
        store.chunks_length = match nom::number::complete::be_u64::<_, nom::error::Error<_>>(length_bytes.as_bytes()) {
            Ok((_, v)) => v,
            Err(e) => return Err(Error::new(
                std::io::ErrorKind::InvalidData
                e.to_string(),
            )),
        };
        let last_offset: &ChunkOffsets = match store.header.chunk_offsets.get(store.header.chunk_offsets_length as usize - 1) {
            Some(v) => v,
            None => return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                format!(
                    "Could not get last entry at index {}",
                    store.header.chunk_offsets_length - 1
                ),
            ))
        };
        buf_reader.seek_relative(last_offset.calculate_offset(store.header.sector_size as u32) as i64)?;
        let chunk_length_bytes: Vec<u8> = Vec::with_capacity(4usize);
        buf_reader.get_ref()
            .take(4)
            .read_to_end(&mut length_bytes)?;
        let chunk_length: u32 = match nom::number::complete::be_u32::<_, nom::error::Error<_>>(chunk_length_bytes.as_bytes()) {
            Ok((_, v)) => v,
            Err(e) => return Err(Error::new(
                std::io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        };buf_reader.seek_relative(-(chunk_length as i64))?;
        let chunk_bytes: Vec<u8> = Vec::with_capacity(chunk_length as usize);
        buf_reader.take(chunk_length as u64)
            .read_to_end(&mut length_bytes)?;
        return match store.latest_chunk.parse_bytes::<&'_ [u8], nom::error::Error<_>>(chunk_bytes.as_bytes()) {
            Ok(_) => Ok(store),
            Err(e) => Err(Error::new(
                std::io::ErrorKind::InvalidInput,
                e.to_string()
            )),
        };
    }
}

impl Transcoder<LogStore> for ChunkStore {
    fn transcode(&self) -> Result<Box<LogStore>, encoding_errors::TranscoderError<LogStore>> {
        todo!("Implement transcoding for ChunkStore<->LogStore")
    }
}