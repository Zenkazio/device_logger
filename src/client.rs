use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

use crate::comm;
use crate::log;
slint::include_modules!();

pub fn main() {
    println!("Running: {}\nCompiled: {}", CURRENT_PLATFORM, COMPILED_ON);

    let log = log::Log::new();
    log.show();
    log.to_file();
    //get_gui_data(log);
    comm::send_local(&log.to_json()).unwrap();
    println!("Client closed.")
}

fn get_gui_data(log: log::Log) -> log::Log {
    let ui = AppWindow::new().expect("msg");

    ui.on_got_clicked(move |edited| println!("{}", edited));

    ui.run().expect("msg");
    log
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_get_gui_data() {
        let _log = get_gui_data(log::Log::new());
    }
}
