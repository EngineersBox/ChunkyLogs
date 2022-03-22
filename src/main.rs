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

use std::time::Duration;
use lazy_static::lazy_static;
use slog::Logger;
use crate::compression::compressor::{CompressionHandler, Compressor};

use crate::logging::logging::initialize_logging;
use crate::configuration::config::Config;
use crate::data::chunk::chunk::{Chunk, ChunkCompressionState};

use crate::data::group::log_group::LogGroup;

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("chunky_logs_"));
}

fn main() {
    info!(&crate::LOGGER, "Configured logging");
    let mut properties: Config = Config::new("config/config.properties");
    properties.read();
    let log_entry: &[u8] = "test log entry or something".as_bytes();
    let log_target: &[u8] = "test target".as_bytes();
    let mut entries: Vec<u8> = Vec::new();
    for i in (0 as u64)..(10 as u64) {
        let ts_bytes: [u8; 8] = (i * 1000).to_be_bytes();
        for ts_byte in ts_bytes.iter() {
            entries.push(*ts_byte);
        }
        entries.push((((i % 4) + 4) % 4) as u8);
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
    let mut bytes: Vec<u8> = vec!(
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
        0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x09,
    );
    for entries_len_bytes in (entries.len() as u32).to_be_bytes().iter() {
        bytes.push(*entries_len_bytes);
    }
    bytes.append(&mut entries);

    // match Chunk::from_bytes(&bytes) {
    //     Err(e) => {
    //         error!(crate::LOGGER, "Error occurred: {}", e.message);
    //     },
    //     Ok(c) => info!(
    //         crate::LOGGER,
    //         "Chunk data: [TS from: {}] [TS to: {}] [Length: {}] [Data: {:02X?}]",
    //         c.ts_from,
    //         c.ts_to,
    //         c.length,
    //         c.data.as_slice()
    //     ),
    // };
    let mut chunk: Chunk = Chunk::new();
    chunk.ts_from = 0;
    chunk.ts_to = 9 * 1000;
    let mut compressor: Compressor = Compressor::new();
    match compressor.compress_slice(&bytes) {
        Ok(compressed_data) => {
            chunk.data = compressed_data;
            chunk.length = chunk.data.len() as u32;
            chunk.state = ChunkCompressionState::COMPRESSED;
            info!(
                &crate::LOGGER,
                "Chunk data: [TS from: {}] [TS to: {}] [Length: {}] [Data: {:02X?}] [State: {:?}]",
                chunk.ts_from,
                chunk.ts_to,
                chunk.length,
                chunk.data.as_slice(),
                chunk.state,
            );
        },
        Err(e) => error!(&crate::LOGGER, "Error occurred: {}", e.message),
    };
    match compressor.decompress_slice(&chunk.data.as_slice()) {
        Ok(decompressed_data) => {
            chunk.data = decompressed_data;
            chunk.length = chunk.data.len() as u32;
            chunk.state = ChunkCompressionState::DECOMPRESSED;
            info!(
                &crate::LOGGER,
                "Chunk data: [TS from: {}] [TS to: {}] [Length: {}] [Data: {:02X?}] [State: {:?}]",
                chunk.ts_from,
                chunk.ts_to,
                chunk.length,
                chunk.data.as_slice(),
                chunk.state,
            );
        },
        Err(e) => error!(&crate::LOGGER, "Error occurred: {}", e.message),
    };
    match LogGroup::from_chunk(&chunk) {
        Ok(group) => {
            info!(
                &crate::LOGGER,
                "Log group: [TS from: {}] [TS to: {}] [Entry Count: {}]",
                group.ts_from,
                group.ts_to,
                group.entries.len(),
            );
            for entry in group.entries.iter() {
                info!(
                    &crate::LOGGER,
                    "Entry: [TS: {}] [Action: {:?}] [Target: {}] [Message: {}]",
                    entry.timestamp,
                    entry.action,
                    entry.target,
                    entry.desc
                );
            }
        }
        Err(e) => error!(&crate::LOGGER, "Error occurred: {}", e.message),
    }
    std::thread::sleep(Duration::from_millis(1000));
}

/* ---- HEADER ----
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