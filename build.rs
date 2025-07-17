use std::{collections::HashMap, path::PathBuf};

fn main() {
    let libraries_path = HashMap::from([
        ( String::from("ui"), PathBuf::from("ui") ),
        ( String::from("assets"), PathBuf::from("assets") )
    ]);

    let config = slint_build::CompilerConfiguration::new()
        .with_library_paths(libraries_path);

    slint_build::compile_with_config("ui/main.slint", config).unwrap();
}
