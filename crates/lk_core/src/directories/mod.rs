pub mod paths;

use std::fs; 

use crate::directories::paths::{
    LAUNCHER_DIR, CONFIG_DIR,
};

pub fn initialize() {
    fs::create_dir_all(&*LAUNCHER_DIR).unwrap();
    fs::create_dir_all(&*CONFIG_DIR).unwrap();

    std::env::set_current_dir(&*LAUNCHER_DIR).unwrap();
}