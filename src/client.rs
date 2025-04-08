use current_platform::{COMPILED_ON, CURRENT_PLATFORM};

use crate::comm;
use crate::log;
slint::include_modules!();

pub fn client_job() {
    println!("Running: {}\nCompiled: {}", CURRENT_PLATFORM, COMPILED_ON);

    let log = log::Log::new();

    comm::send_local(&log.to_json()).unwrap();
    // let ui = AppWindow::new().expect("msg");

    // ui.on_got_clicked(move |edited| println!("{}", edited));

    // ui.run().expect("msg");
    println!("Client closed.")
}
