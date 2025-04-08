// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
//#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(dead_code, unused_imports, unused_variables)]

use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
mod cli;
mod client;
mod server;

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename = "dato")]
#[serde(rename_all = "camelCase")]
struct Data {
    number: u32,
    string_test: String,
}

impl Data {}

fn main() {
    // match cli::is_server() {
    //     true => server::server_job(),
    //     false => client::client_job(),
    // }
    let data = Data {
        number: 10,

        string_test: String::from("test"),
    };
    let json_string: String = to_string_pretty(&data).expect("serializing did not work :(");
    println!("the string:");
    println!("{}", &json_string);
    println!("---------");

    let data2: Data = from_str(&json_string).expect("deserializing did not work :(");

    println!("{:?}", data2);
}
