mod env_detection;
mod tasks;

fn main() {
    // Load the configuration based on the OS and hostname
    let config = env_detection::detect_os_and_load_config();
    println!("Loaded configuration: {:?}", config);

    // Perform actions based on configuration
    if config.settings.install_rosetta && cfg!(target_os = "macos") {
        tasks::install_rosetta::run();
    }

    if config.settings.link_files {
        tasks::file_linking::link_files(&config.file_locations.link_target_dir);
    }
}
