use chrono::Local;
use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use serde::{Deserialize, Serialize};
use serde_json::{from_str, to_string_pretty};
use std::{fs, path};

#[derive(Serialize, Deserialize)]
pub struct Log {
    name: String,
    creation_time: String,
    compiled_on: String,
}

impl Log {
    pub fn new() -> Self {
        let local_time = Local::now();

        Log {
            name: format!("{}-{CURRENT_PLATFORM}", local_time.format("%Y-%m-%d")),
            creation_time: local_time.format("%Y-%m-%d %H:%M:%S").to_string(),
            compiled_on: COMPILED_ON.to_string(),
        }
    }

    pub fn to_file(&self) {
        let logs_dir = "./logs";
        let log_file_name = format!("{}_log.json", self.name);
        match path::Path::new(logs_dir).exists() {
            true => {}
            false => match fs::create_dir(logs_dir) {
                Ok(()) => (),
                Err(err) => println!("Directory Creation Error: {}", err),
            },
        }
        fs::write(format!("{}/{}", logs_dir, log_file_name), self.to_json()).unwrap();
    }
    pub fn to_json(&self) -> String {
        to_string_pretty(&self).expect("Something went wrong turning the Log into Json")
    }

    pub fn from_json(json: &str) -> Self {
        from_str(json).unwrap()
    }
    pub fn show(&self) {
        println!("{}", self.to_json());
    }
}
