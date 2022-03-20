use std::fs::{File, OpenOptions};
use std::fs;
use std::sync::Mutex;

use lazy_static::lazy_static;
use slog::{Drain, Duplicate, Fuse, Logger};
use slog_async::{Async, OverflowStrategy};
use slog_json::Json;
use slog_term::{FullFormat, TermDecorator};

mod compression;
mod data;
mod cache;
mod logging;
mod configuration;
mod macros;

use crate::logging::logging::initialize_logging;
use crate::configuration::config::Config;

#[macro_use]
extern crate slog;
extern crate slog_term;
extern crate slog_async;
extern crate slog_json;
extern crate lazy_static;
extern crate regex;

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("chunky_logs_"));
}

fn main() {
    println!("Hello, world!");
    let mut properties: Config = Config::new("config/config.properties");
    properties.read();
}
