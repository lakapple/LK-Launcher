use std::path::PathBuf;
use std::sync::LazyLock;
use std::fs;

use crate::constants;

pub static LAUNCHER_DIRECTORY: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_exe().unwrap().parent().unwrap().to_path_buf()
});

pub static LOGS_DIRECTORY: LazyLock<PathBuf> = LazyLock::new(|| {
    let path = LAUNCHER_DIRECTORY.join(constants::LOGS_DIRECTORY);

    fs::create_dir_all(&path).unwrap();

    path
});