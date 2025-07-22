use std::env;
use std::path::PathBuf;
use std::sync::LazyLock;


pub static LAUNCHER_DIR: LazyLock<PathBuf> = LazyLock::new(|| {
    #[cfg(feature = "portable")]
    {
        let current_exe = env::current_exe().unwrap();
        return current_exe.parent().unwrap().to_path_buf();
    }

    #[cfg(target_os = "windows")]
    {
        env::var("APPDATA")
            .map(|appdata| PathBuf::from(appdata).join("LKLauncher")).unwrap()
    }

    #[cfg(target_os = "linux")]
    {
        use std::path::Path;

        env::var("HOME")
            .map(|home| PathBuf::from(home).join(".lklauncher")).unwrap()
    }

    #[cfg(target_os = "macos")]
    {
        env::var("HOME")
            .map(|home| {
                PathBuf::from(home)
                    .join("Library")
                    .join("Application Support")
                    .join("LKLauncher"
                )
            })
        .unwrap()
    }
});

pub static CONFIG_DIR: LazyLock<PathBuf> = LazyLock::new(|| LAUNCHER_DIR.join("config"));