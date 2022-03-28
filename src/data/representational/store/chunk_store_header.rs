use crate::{byte_layout, reify};
use super::chunk_offsets::ChunkOffsets;
use std::fs::File;
use std::io;
use memmap::{Mmap, MmapOptions};

reify!{
    #[derive(Debug,Default)]
    pub struct ChunkStoreHeader {
        #[byte_size=8]
        pub length: u64,
        #[byte_size=8]
        pub sector_size: u64,
        #[byte_size=4]
        pub chunk_count: u32,
        #[byte_size=4]
        pub chunk_offsets_length: u32,
        pub chunk_offsets: Vec<ChunkOffsets>,
    }
}

byte_layout! {
    ChunkStoreHeader
    value [length, {nom::number::complete::be_u64::<I,E>}]
    value [sector_size, {nom::number::complete::be_u64::<I,E>}]
    value [chunk_count, {nom::number::complete::be_u32::<I,E>}]
    value [chunk_offsets_length, {nom::number::complete::be_u32::<I,E>}]
    composite_vec [chunk_offsets, chunk_offsets_length, ChunkOffsets]
}

impl ChunkStoreHeader {
    fn bytes_len() -> Result<usize, std::io::Error> {
        let sum: usize = Self::get_field_attribute_map()
            .into_values()
            .map(|val: String| -> usize {
                let collected_split: Vec<&str> = val.split("=").collect::<Vec<&str>>();
                let split_val: Option<&&str> = collected_split.get(1);
                if split_val.is_none() {
                    return 0;
                }
                return match split_val.unwrap().parse::<usize>() {
                    Ok(v) => v,
                    Err(_) => 0,
                }
            }).sum::<usize>();
        return Ok(sum);
    }
    pub fn read_from_file(&self, file: &File) -> Result<(), io::Error> {
        let header_length: usize = match Self::bytes_len() {
            Ok(v) => v,
            Err(e) => return Err(e),
        };
        let mmap_file: Mmap = unsafe {
            MmapOptions::new()
                .len(header_length)
                .map(file)?
        };
        return Ok(());
    }
}