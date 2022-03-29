use crate::{byte_layout, reify};
use super::chunk_offsets::ChunkOffsets;
use std::fs::File;
use std::io;
use std::io::{BufReader, ErrorKind, Read, Seek};
use nom::{AsBytes, ExtendInto};


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
    pub fn header_length() -> Result<usize, std::io::Error> {
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
        let header_length: usize = match Self::header_length() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let mut buf_reader: BufReader<&File> = BufReader::new(file);
        let mut length_bytes: Vec<u8> = Vec::with_capacity(header_length);
        buf_reader.by_ref()
            .take(header_length as u64)
            .read_to_end(&mut length_bytes)?;
        let sized_length_bytes: u64 = match nom::number::complete::be_u64::<_, nom::error::Error<_>>(length_bytes.as_bytes()) {
            Ok((_, v)) => v,
            Err(e) => return Err(io::Error::new(
                io::ErrorKind::InvalidData,
                e.to_string(),
            )),
        };
        let mut header_bytes: Vec<u8> = Vec::with_capacity(sized_length_bytes as usize);
        buf_reader.rewind()?;
        buf_reader.take(sized_length_bytes as u64)
            .read_to_end(&mut header_bytes)?;
        return match self.parse_bytes::<&'_ [u8], nom::error::Error<_>>(header_bytes.as_bytes()) {
            Ok(_) => Ok(()),
            Err(e) => Err(io::Error::new(
                ErrorKind::InvalidInput,
                e.to_string()
            )),
        };
    }
    pub fn string_format_chunk_sector_ratio(&self) -> String {
        let mut chunks_outer: String = String::new();
        let mut chunks: String = String::new();
        for i in 0..(self.chunk_offsets_length - 1) {
            let next_offset: &ChunkOffsets = self.chunk_offsets.get(i as usize + 1).unwrap();
            let chunk_size: u32 = (next_offset.sector_index * self.sector_size as u32) + next_offset.sector_offset as u32;
            chunks_outer.push('+');
            chunks_outer.push_str("-".repeat(chunk_size as usize -1).as_str());
            let mut chunk_str: String = String::new();
            chunk_str.push('|');
            chunk_str.push_str(format!(
                "C{} {}B",
                i,
                chunk_size
            ).as_str());
            chunk_str.push_str(" ".repeat(chunk_size as usize - chunk_str.len() - 1).as_str());
            chunk_str.push(' ');
            chunks.push_str(chunk_str.as_str());
        }
        let last_offset: &ChunkOffsets = self.chunk_offsets.get((self.chunk_offsets_length - 1) as usize).unwrap();
        let mut sector_markers_outer: String = String::new();
        let mut sector_markers: String = String::new();
        for i in 0..(last_offset.sector_index + 1) {
            sector_markers_outer.push('+');
            sector_markers_outer.push_str("-".repeat((self.sector_size - 1) as usize).as_str());
            sector_markers.push('|');
            let num_string: String = format!("{}", i);
            sector_markers.push_str(num_string.as_str());
            sector_markers.push_str(" ".repeat((self.sector_size - 2) as usize - num_string.len()).as_str());
            sector_markers.push(' ');
        }

        let mut last_chunk: String = String::new();
        last_chunk.push('|');
        last_chunk.push_str(format!(
            "C{} ?B",
            self.chunk_offsets_length - 1
        ).as_str());
        last_chunk.push(' ');
        chunks.push_str(last_chunk.as_str());

        chunks_outer.push('+');
        chunks_outer.push_str("-".repeat(last_chunk.len() - 1).as_str());
        sector_markers.truncate(chunks.len());
        sector_markers_outer.truncate(chunks.len());
        return format!(
            "Sector size: {}B\n         {}\nSectors: {}\n         {}\nChunks:  {}\n         {}",
            self.sector_size,
            sector_markers_outer,
            sector_markers,
            chunks_outer,
            chunks,
            chunks_outer,
        );
    }
}