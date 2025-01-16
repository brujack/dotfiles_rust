use std::fs;
use anyhow::{Context, Error}; // Import the Context trait for `context` or `with_context`
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub settings: Settings,
    pub file_locations: FileLocations,
}

#[derive(Debug, Deserialize)]
pub struct Settings {
    pub install_rosetta: bool,
    pub link_files: bool,
}

#[derive(Debug, Deserialize)]
pub struct FileLocations {
    pub textfiles_dir: String,
    pub link_target_dir: String,
}

pub fn detect_os_and_load_config() -> Config {
    let hostname = detect_hostname();

    // Hostname-specific config file
    let hostname_specific_config_path = format!("config/{}-custom.toml", hostname);

    // Start with an empty base config
    let mut config = Config {
        settings: Settings {
            install_rosetta: false,
            link_files: false,
        },
        file_locations: FileLocations {
            textfiles_dir: String::new(),
            link_target_dir: String::new(),
        },
    };

    // Load hostname-specific configuration if it exists
    if let Ok(hostname_config) = load_config(&hostname_specific_config_path) {
        merge_configs(&mut config, hostname_config);
    }

    config
}

fn detect_hostname() -> String {
    hostname::get()
        .map(|hostname| {
            let full_hostname = hostname.to_string_lossy();
            full_hostname
                .split('.')
                .next()
                .unwrap_or("unknown")
                .to_string()
        })
        .unwrap_or_else(|_| "unknown".to_string())
}

fn load_config(path: &str) -> Result<Config, Error> {
    let content = fs::read_to_string(path)
        .with_context(|| format!("Failed to read config file: {}", path))?;
    let config = toml::from_str(&content)
        .with_context(|| format!("Failed to parse config file: {}", path))?;
    Ok(config)
}

fn merge_configs(base: &mut Config, override_config: Config) {
    base.settings = override_config.settings;
    base.file_locations = override_config.file_locations;
}
