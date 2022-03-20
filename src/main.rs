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

use lazy_static::lazy_static;
use slog::Logger;

use crate::logging::logging::initialize_logging;
use crate::configuration::config::Config;

lazy_static! {
    static ref LOGGER: Logger = initialize_logging(String::from("chunky_logs_"));
}

fn main() {
    println!("Hello, world!");
    let mut properties: Config = Config::new("config/config.properties");
    properties.read();
}
