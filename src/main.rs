mod compression;
mod data;
mod cache;
mod logging;
mod configuration;
mod macros;

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
use std::io::Write;
use std::time::Duration;
use lazy_static::lazy_static;
use slog::Logger;

use crate::compression::compressor::{CompressionHandler, Compressor};
use crate::logging::logging::initialize_logging;
use crate::configuration::config::Config;
use crate::data::chunk::chunk::{Chunk, Byte, ChunkCompressionState};
use crate::data::group::log_entry::LogEntry;
use crate::data::group::log_group::LogGroup;
use crate::data::store::log_store::LogStore;

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("chunky_logs_"));
}

fn main() {
    info!(&crate::LOGGER, "Configured logging");
    let mut properties: Config = Config::new("config/config.properties");
    properties.read();

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

    let mut bytes: Vec<Byte> = Vec::new();
    let length_bytes: [Byte; 8] = ((chunk_bytes.len() * 2) as u64).to_be_bytes();
    bytes.append(length_bytes.to_vec().as_mut());

    let offset_bytes: [Byte; 8] = (chunk_bytes.len() as u64).to_be_bytes();
    bytes.append(offset_bytes.to_vec().as_mut());

    bytes.append(chunk_bytes.to_vec().as_mut());
    bytes.append(chunk_bytes.to_vec().as_mut());

    // let mut file: io::Result<File> = File::create("E:/ChunkyLogs/log_chunks.bin");
    // if file.is_ok() {
    //     match file.unwrap().write_all(bytes.as_slice()) {
    //         Ok(_) => {},
    //         Err(e) => {
    //             error!(crate::LOGGER, "An error occurred: {}", e.to_string());
    //             return;
    //         }
    //     }
    // }

    let mut store: LogStore = LogStore::with_filepath("log_chunks.bin");
    match store.import_latest() {
        Ok(()) => {},
        Err(e) => {
            error!(crate::LOGGER, "An error occurred: {}", e.message);
            return;
        }
    }
    info!(
        crate::LOGGER,
        "Log store [Length: {}] [Current Offset: {}]",
        store.length,
        store.current_chunk_offset,
    );
    print_chunk_data!(store.current_chunk);
    std::thread::sleep(Duration::from_millis(1000));
}

/*
* ==== HEADER ====
* 00, 00, 00, 00, 00, 00, 01, FF,
* 00, 00, 00, 00, 00, 00, 00, 00,
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

