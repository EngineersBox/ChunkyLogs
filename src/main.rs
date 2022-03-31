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

fn create_test_bytes() {
    let mut rng = rand::thread_rng();
    let mut bytes: Vec<Byte> = Vec::new();

    let chunk_store_header: ChunkStoreHeader = ChunkStoreHeader{
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
    let mut chunk_store: ChunkStore = ChunkStore::default();
    let mut file: io::Result<File> = File::open("data/chunk_store.bin");
    if file.is_ok() {
        match ChunkStore::read_from_file("data/chunk_store.bin") {
            Ok(cs) => {
                chunk_store = cs;
                info!(crate::LOGGER, "Header: {:?}", chunk_store);
            },
            Err(e) => error!(crate::LOGGER, "An error occurred: {}", e.to_string()),
        }
    }
    // file = File::open("data/chunk_store.bin");
    // if file.is_ok() {
    //     match chunk_store.header.string_format_chunk_sector_ratio(&file.unwrap()) {
    //         Ok(s) => info!(crate::LOGGER, "{}", s),
    //         Err(e) => error!(crate::LOGGER, "An error occurred: {}", e.to_string()),
    //     };
    // }

    std::thread::sleep(Duration::from_millis(1000));
}