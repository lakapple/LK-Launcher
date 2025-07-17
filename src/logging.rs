use fern::colors::{Color, ColoredLevelConfig};
use log::LevelFilter;
use std::fs;

use crate::globals;


pub fn initialize() -> Result<(), fern::InitError> {
    // Logging base configuration
    let base_config = fern::Dispatch::new()
        .level(LevelFilter::Debug)
        .level_for("slint::platform", LevelFilter::Info);

    // Console output configuration
    let console_colors = ColoredLevelConfig::new()
        .info(Color::Green)
        .warn(Color::Yellow)
        .error(Color::Red)
        .debug(Color::Cyan)
        .trace(Color::BrightBlack);

    let console_config = fern::Dispatch::new()
        .format( move |out, message, record| {
            out.finish(format_args!(
                "[{}] {}",
                console_colors.color(record.level()),
                message
            ))
        })
        .chain(std::io::stdout());

    // File output configuration
    let logs_directory = &*globals::LAUNCHER_DIRECTORY.join("logs");

    fs::create_dir_all(logs_directory)?;

    let log_file_name = format!(
        "{}.log",
        chrono::Local::now().format("%Y-%m-%d-%H-%M-%S")
    );

    let log_file_path = logs_directory.join(log_file_name);

    let file_config = fern::Dispatch::new()
        .format(move |out, message, record| {
            out.finish(format_args!(
                "[{}] [{}] {}",
                chrono::Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                message
            ))
        })
        .chain(fern::log_file(log_file_path)?);

    // Combine all configurations
    base_config
        .chain(console_config)
        .chain(file_config)
        .apply()?;

    Ok(())
}