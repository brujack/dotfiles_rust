use std::fs;
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
    let os_name = detect_os();
    let hostname = detect_hostname();

    let default_config_path = "config/default.toml";
    let os_specific_config_path = format!("config/{}.toml", os_name);
    let hostname_specific_config_path = format!("config/{}-custom.toml", hostname);

    let mut config = load_config(default_config_path).expect("Failed to load default configuration");

    if let Ok(os_config) = load_config(&os_specific_config_path) {
        merge_configs(&mut config, os_config);
    }

    if let Ok(hostname_config) = load_config(&hostname_specific_config_path) {
        merge_configs(&mut config, hostname_config);
    }

    config
}

fn detect_os() -> String {
    if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "linux") {
        match fs::read_to_string("/etc/os-release") {
            Ok(contents) => {
                if contents.contains("Ubuntu") {
                    "ubuntu".to_string()
                } else if contents.contains("Debian") {
                    "debian".to_string()
                } else if contents.contains("Red Hat") {
                    "redhat".to_string()
                } else if contents.contains("CentOS") {
                    "centos".to_string()
                } else {
                    "unknown".to_string()
                }
            }
            Err(_) => "unknown".to_string(),
        }
    } else {
        "unknown".to_string()
    }
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

fn load_config(path: &str) -> Result<Config, toml::de::Error> {
    let content = fs::read_to_string(path).expect(&format!("Failed to read config file: {}", path));
    toml::from_str(&content)
}

fn merge_configs(base: &mut Config, override_config: Config) {
    base.settings = override_config.settings;
    base.file_locations = override_config.file_locations;
}
