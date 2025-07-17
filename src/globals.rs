use std::path::PathBuf;
use std::sync::LazyLock;

pub static LAUNCHER_DIRECTORY: LazyLock<PathBuf> = LazyLock::new(|| {
    std::env::current_exe().unwrap().parent().unwrap().to_path_buf()
});