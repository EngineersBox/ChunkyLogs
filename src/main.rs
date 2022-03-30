mod compression;
mod data;
mod cache;
mod logging;
mod configuration;
mod macros;
mod encoding;
mod compiler;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate slog_json;
extern crate lazy_static;
extern crate regex;
extern crate chrono;
extern crate core;

use std::fs::File;
use std::io;
use std::io::{Error, Write};
use std::time::Duration;
use lazy_static::lazy_static;
use slog::Logger;
use crate::compiler::byte_unpack::ToVec;
use lipsum::lipsum;
use rand::Rng;

use crate::compression::compressor::Compressor;
use crate::logging::logging::initialize_logging;
use crate::configuration::config::Config;
use crate::data::representational::chunk::Chunk;
use crate::data::representational::chunk_entry::ChunkEntry;
use crate::data::representational::store::chunk_offsets::ChunkOffsets;
use crate::data::representational::store::chunk_store::ChunkStore;
use crate::data::representational::store::chunk_store_header::ChunkStoreHeader;

type Byte = u8;

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("chunky_logs_"));
}

macro_rules! print_chunk_data {
    ($c:expr) => {
        info!(
            &crate::LOGGER,
            "Chunk data: [TS from: {}] [TS to: {}] [Length: {}] [State: {:?}] [Data: {:02X?}]",
            $c.ts_from,
            $c.ts_to,
            $c.length,
            $c.state,
            $c.data.as_slice(),
        )
    }
}

fn write_test_file() {
    let log_entry: &[Byte] = "test log entry or something".as_bytes();
    let log_target: &[Byte] = "test target".as_bytes();
    let mut entries: Vec<Byte> = Vec::new();
    for i in (0 as u64)..(10 as u64) {
        let ts_bytes: [Byte; 8] = ((i * 1000) as u64).to_be_bytes();
        for ts_byte in ts_bytes.iter() {
            entries.push(*ts_byte);
        }
        entries.push((((i % 4) + 4) % 4) as Byte);
        for target_byte in log_target.iter() {
            entries.push(*target_byte);
        }
        entries.push(0x00);
        for entry_byte in log_entry.iter() {
            entries.push(*entry_byte);
        }
        entries.push(0x00);
    }
    entries.push(0x00);

    let mut chunk_bytes: Vec<Byte> = vec!(
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
    );

    let mut compressor: Compressor = Compressor::new();
    match compressor.compress_vec(&entries) {
        Ok(b) => entries = b,
        Err(e) => {
            error!(crate::LOGGER, "An error occurred: {}", e.message);
            return;
        }
    }

    for entries_len_bytes in (entries.len() as u32).to_be_bytes().iter() {
        chunk_bytes.push(*entries_len_bytes);
    }
    chunk_bytes.append(&mut entries);

    let mut bytes: Vec<Byte> = Vec::new();
    let length_bytes: [Byte; 8] = ((chunk_bytes.len() * 2) as u64).to_be_bytes();
    bytes.append(length_bytes.to_vec().as_mut());

    let offset_bytes: [Byte; 8] = (chunk_bytes.len() as u64).to_be_bytes();
    bytes.append(offset_bytes.to_vec().as_mut());

    bytes.append(chunk_bytes.to_vec().as_mut());
    bytes.append(chunk_bytes.to_vec().as_mut());

    let mut file: io::Result<File> = File::create("data/log_chunks.bin");
    if file.is_ok() {
        match file.unwrap().write_all(bytes.as_slice()) {
            Ok(_) => {},
            Err(e) => {
                error!(crate::LOGGER, "An error occurred: {}", e.to_string());
                return;
            }
        }
    }
}

fn create_test_bytes() {
    let mut rng = rand::thread_rng();
    let mut bytes: Vec<Byte> = Vec::new();

    let mut chunk_store_header: ChunkStoreHeader = ChunkStoreHeader{
        length: 0,
        sector_size: 30,
        chunk_count: 2,
        chunk_offsets_length: 0,
        chunk_offsets: Vec::new(),
    };

    let mut chunk: Chunk = Chunk::default();
    let mut chunk_entry: ChunkEntry = ChunkEntry::default();
    chunk_entry.target.push(0x00);
    chunk_entry.message.push(0x00);
    for i in 0..2 {
        chunk_entry.timestamp = i as u64;
        chunk_entry.action = i as u8;
        chunk_entry.target = lipsum::lipsum(rng.gen_range(1..5)).as_bytes().to_vec();
        chunk_entry.message = lipsum::lipsum_words(rng.gen_range(10..20)).as_bytes().to_vec();
        chunk.entries.append(&mut chunk_entry.into_bytes());
        chunk.entries_length += 1;
    }
    chunk.timestamp_from = 0;
    chunk.timestamp_to = 1;

    let mut compressor: Compressor = Compressor::new();
    match compressor.compress_vec(&chunk.entries) {
        Ok(b) => chunk.entries = b,
        Err(e) => {
            error!(crate::LOGGER, "An error occurred: {}", e.message);
            return;
        }
    }
    chunk.length = chunk.into_bytes().len() as u32;

    let mut chunk_store: ChunkStore = ChunkStore::default();
    chunk_store.header = chunk_store_header;
    let mut running_length: u32 = 0;
    for _ in 0..chunk_store.header.chunk_count {
        let chunk_offset: ChunkOffsets = ChunkOffsets{
            sector_index: running_length / chunk_store.header.sector_size as u32,
            sector_offset: running_length as u16 % chunk_store.header.sector_size as u16,
        };
        chunk_store.header.chunk_offsets.push(chunk_offset);
        chunk_store.header.chunk_offsets_length += 1;
        chunk_store.chunks.push(chunk.clone());
        chunk_store.chunks_length += 1;
        running_length += chunk.length;
    }
    chunk_store.header.length = chunk_store.header.into_bytes().len() as u64;

    bytes.append(chunk_store.header.into_bytes().to_vec().as_mut());

    let file: io::Result<File> = File::create("data/chunk_store.bin");
    if file.is_ok() {
        match file.unwrap().write_all(chunk_store.into_bytes().as_slice()) {
            Ok(_) => {},
            Err(e) => {
                error!(crate::LOGGER, "Error while writing to file: {}", e.to_string());
                return;
            }
        }
    }
}

fn main() {
    info!(&crate::LOGGER, "Configured logging");
    // let mut properties: Config = Config::new("config/config.properties");
    // properties.read();

    create_test_bytes();
    let mut chunk_store_header: ChunkStoreHeader = ChunkStoreHeader::default();
    let mut file: io::Result<File> = File::open("data/chunk_store.bin");
    if file.is_ok() {
        match chunk_store_header.read_from_file(&file.unwrap()) {
            Ok(_) => info!(crate::LOGGER, "Header: {:?}", chunk_store_header),
            Err(e) => error!(crate::LOGGER, "An error occurred: {}", e.to_string()),
        }
    }
    file = File::open("data/chunk_store.bin");
    if file.is_ok() {
        match chunk_store_header.string_format_chunk_sector_ratio(&file.unwrap()) {
            Ok(s) => info!(crate::LOGGER, "{}", s),
            Err(e) => error!(crate::LOGGER, "An error occurred: {}", e.to_string()),
        };
    }

    std::thread::sleep(Duration::from_millis(1000));
}

/*
* ==== HEADER ====
* 00, 00, 00, 00, 00, 00, 01, FF,
* 00, 00, 00, 00, 00, 00, 00, 00,
* 00, 00, 00, 02,
* 00, 00, 00, 00,
* --- CHUNK OFFSET 0 ----
* 00, 00, 00, 00
* 00,
* 00, 00, 00, 00
* --- CHUNK OFFSET 1 ----
*
* ==== CHUNK 0 ====
* ...
* ==== CHUNK 1 ====
* ---- CHUNK HEADER ----
* 00, 00, 00, 00, 00, 00, 00, 00,
* 00, 00, 00, 00, 00, 00, 00, 01,
* 00, 00, 01, EB,
*  ---- DECOMPRESSED DATA ----
* 00, 00, 00, 00, 00, 00, 00, 00,
* 00,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 01,
* 01,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 02,
* 02,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 03,
* 03,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 04,
* 00,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 05,
* 01,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 06,
* 02,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 07,
* 03,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 08,
* 00,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00, 00, 00, 00, 00, 00, 00, 09,
* 01,
* 74, 65, 73, 74, 20, 74, 61, 72, 67, 65, 74, 00,
* 74, 65, 73, 74, 20, 6C, 6F, 67, 20, 65, 6E, 74, 72, 79, 20, 6F, 72, 20, 73, 6F, 6D, 65, 74, 68, 69, 6E, 67, 00,
*
* 00
* ---- END ----
*/

