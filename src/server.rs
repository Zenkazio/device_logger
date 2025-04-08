use crate::comm::listen_local;
use crate::log;

pub fn server_job() {
    let log = log::Log::from_json(listen_local().unwrap().as_str());
    log.show();
    log.to_file();
    println!("Server closed.");
}
