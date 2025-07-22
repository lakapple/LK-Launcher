// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

slint::include_modules!();

fn main() {
    lk_core::directories::initialize();

    let app = App::new().unwrap();

    app.run().unwrap();
}
