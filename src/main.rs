// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables)]

use std::time::{Duration, Instant};

use chrono::{Local, Utc};

mod cli;
mod client;
mod log;
mod server;

fn main() {
    // match cli::is_server() {
    //     true => server::server_job(),
    //     false => client::client_job(),
    // }

    let log = log::Log::new();
    log.to_file();
}
