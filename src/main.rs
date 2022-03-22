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

use lazy_static::lazy_static;
use slog::Logger;
use crate::compression::compressor::{CompressionHandler, Compressor};

use crate::logging::logging::initialize_logging;
use crate::configuration::config::Config;
use crate::data::chunk::chunk::{Chunk, ChunkCompressionState};

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("chunky_logs_"));
}

fn main() {
    info!(&crate::LOGGER, "Configured logging");
    let mut properties: Config = Config::new("config/config.properties");
    properties.read();
    // let bytes: Vec<u8> = vec!(
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    //     0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x01,
    //     0x00, 0x00, 0x00, 0x04,
    //     0xde, 0xad, 0xbe, 0xef
    // );
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
    let data: &[u8] = "test".as_bytes();
    let mut chunk: Chunk = Chunk::new();
    chunk.ts_from = 0;
    chunk.ts_to = 1;
    let mut compressor: Compressor = Compressor::new();
    match compressor.compress_slice(&data) {
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
    std::thread::sleep_ms(1000);
}
