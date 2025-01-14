use std::fs;
use std::os::unix::fs::symlink;
use std::path::Path;

pub fn link_files(home_directory: &str) {
    let source_dir = "files_link";
    let target_dir = home_directory; // Use the passed home directory as the target

    println!("Linking files from '{}' to '{}'", source_dir, target_dir);

    // Ensure the target directory exists
    if !Path::new(target_dir).exists() {
        if let Err(e) = fs::create_dir_all(target_dir) {
            eprintln!("Failed to create target directory '{}': {}", target_dir, e);
            return;
        }
    }

    // Iterate through the files and directories in the source directory
    if let Ok(entries) = fs::read_dir(source_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let source_path = entry.path();
                let file_name = match source_path.file_name() {
                    Some(name) => name,
                    None => {
                        eprintln!("Invalid file name in source directory: {:?}", source_path);
                        continue;
                    }
                };

                let target_path = Path::new(target_dir).join(file_name);

                if source_path.is_dir() {
                    // If the source is a directory, ensure it exists in the target
                    if !target_path.exists() {
                        if let Err(e) = fs::create_dir_all(&target_path) {
                            eprintln!(
                                "Failed to create directory '{:?}' in target: {}",
                                target_path, e
                            );
                            continue;
                        }
                        println!("Created directory '{:?}'", target_path);
                    }
                } else {
                    // If the source is a file, create a symbolic link
                    if let Err(e) = symlink(&source_path, &target_path) {
                        eprintln!(
                            "Failed to link '{:?}' to '{:?}': {}",
                            source_path, target_path, e
                        );
                    } else {
                        println!("Linked '{:?}' to '{:?}'", source_path, target_path);
                    }
                }
            }
        }
    } else {
        eprintln!("Failed to read the source directory: '{}'. Ensure it exists and is accessible.", source_dir);
    }
}
