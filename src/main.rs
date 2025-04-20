// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
//#![allow(dead_code, unused_imports, unused_variables)]

mod cli;
mod client;
mod comm;
mod log;
mod server;

fn main() {
    if cli::is_server() {
        server::main();
    } else {
        client::main();
    }
}
