// Prevent console window in addition to Slint window in Windows release builds when, e.g., starting the app via file manager. Ignored on other platforms.
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::error::Error;

mod globals;
mod logging;
mod constants;

slint::include_modules!();


fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the logger
    if let Err(e) = logging::initialize() {
        eprintln!("FATAL: Failed to initialize logger: {}", e);
        return Err(Box::new(e));
    }

    // Initialize the main window
    let ui = match MainWindow::new() {
        Ok(ui) => ui,
        Err(e) => {
            log::error!("FATAL: Failed to create main window: {}", e);
            return Err(Box::new(e));
        }
    };

    log::info!("Starting LK Launcher");

    // Show the main window
    match ui.run() {
        Ok(_) => log::info!("Terminated LK Launcher"),
        Err(e) => log::error!("FATAL: Failed to run LK Launcher: {}", e),
    }

    Ok(())
}
