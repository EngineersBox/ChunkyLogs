use crate::{byte_layout, reify};
use super::chunk_offsets::ChunkOffsets;
use std::fs::File;
use std::io;
use std::io::ErrorKind;
use memmap::{Mmap, MmapOptions};
use nom::AsBytes;
use crate::compiler::errors::proc_macro_errors::{ByteLayoutParsingError, StructFieldNotFoundError};

reify!{
    #[derive(Debug,Default,Clone)]
    pub struct ChunkStoreHeader {
        #[byte_size=8]
        pub length: u64,
        #[byte_size=2]
        pub sector_size: u16,
        #[byte_size=2]
        pub chunk_count: u16,
        #[byte_size=4]
        pub chunk_offsets_length: u32,
        pub chunk_offsets: Vec<ChunkOffsets>,
    }
}

byte_layout! {
    ChunkStoreHeader
    value [length, u64, Big]
    value [sector_size, u16, Big]
    value [chunk_count, u16, Big]
    value [chunk_offsets_length, u32, Big]
    composite_vec [chunk_offsets, chunk_offsets_length, ChunkOffsets]
}

impl ChunkStoreHeader {
    pub fn header_length_bytes_count() -> Result<usize, std::io::Error> {
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
        return match split_val.unwrap().parse::<usize>() {
            Ok(v) => Ok(v),
            Err(_) => Ok(0),
        };
    }
    pub fn read_from_file(&mut self, file: &File) -> Result<(), io::Error> {
        let header_length_bytes: usize = match Self::header_length_bytes_count() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let mut mmap_file: Mmap = unsafe {
            MmapOptions::new()
                .len(header_length_bytes)
                .map(file)?
        };
        let length_bytes: &[u8] = mmap_file.as_bytes();
        if length_bytes.len() > 8 {
            return Err(io::Error::new(
                io::ErrorKind::InvalidInput,
                "Length was not a u64"
            ));
        }
        let sized_length_bytes: u64 = match nom::number::complete::be_u64::<_, nom::error::Error<_>>(length_bytes) {
            Ok((_, v)) => v,
            Err(e) => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        };
        mmap_file = unsafe {
            MmapOptions::new()
                .len(sized_length_bytes as usize)
                .map(file)?
        };
        return match self.parse_bytes::<&'_ [u8], nom::error::Error<_>>(mmap_file.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(
                ErrorKind::InvalidInput,
                e.to_string()
            )),
        };
    }
}