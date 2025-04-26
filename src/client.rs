use current_platform::{COMPILED_ON, CURRENT_PLATFORM};
use slint::ComponentHandle;
use slint::quit_event_loop;

use crate::comm;
use crate::log;
slint::include_modules!();

pub fn main() {
    println!("Running: {}\nCompiled: {}", CURRENT_PLATFORM, COMPILED_ON);

    let log = log::Log::new();
    log.show();
    //log.to_file();
    //get_gui_data(log);
    match comm::send_local(&log.to_json()) {
        Ok(_) => {}
        Err(e) => {
            eprintln!("Data was not send. {:?}", e)
        }
    }
    println!("Client closed.")
}
#[allow(dead_code)]
fn get_gui_data(log: log::Log) -> log::Log {
    let ui = AppWindow::new().expect("msg");
    let ui_weak = ui.as_weak();

    ui.on_got_clicked(move |edited| println!("{}", edited));
    ui.on_close(move || {
        quit_event_loop().expect("lol");
        ()
    });
    ui.on_go_fullscreen(move || {
        if ui_weak.unwrap().get_is_fullscreen() {
            ui_weak.unwrap().set_is_fullscreen(false);
            dbg!(ui_weak.unwrap().window().set_maximized(true));
        } else {
            ui_weak.unwrap().set_is_fullscreen(true);
        };
        ()
    });

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
