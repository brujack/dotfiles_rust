use std::fs;
use std::os::unix::fs::symlink;
use std::path::{Path, PathBuf};

pub fn link_files(home_directory: &str) {
    // Ensure the home_directory path is valid
    let home_dir_path = Path::new(home_directory);
    if !home_dir_path.exists() {
        eprintln!("Error: The provided home directory '{}' does not exist.", home_directory);
        return;
    }

    // Create the target files_to_link directory in the home directory
    let target_root = home_dir_path.join("files_to_link"); // ~/files_to_link
    if !target_root.exists() {
        if let Err(e) = fs::create_dir_all(&target_root) {
            eprintln!("Failed to create directory '{:?}': {}", target_root, e);
            return;
        }
        println!("Created target directory: {:?}", target_root);
    }

    // Define the source files_to_link directory from the repo
    let source_dir = Path::new("files_to_link"); // ./files_to_link
    if !source_dir.exists() {
        eprintln!("Error: Source directory '{}' does not exist.", source_dir.display());
        return;
    }

    // Copy files and directories from source_dir to target_root
    println!("Copying files from '{}' to '{:?}'", source_dir.display(), target_root);
    if let Ok(entries) = fs::read_dir(&source_dir) {
        for entry in entries {
            if let Ok(entry) = entry {
                let source_path = entry.path();
                let relative_path = source_path.strip_prefix(&source_dir).unwrap();
                let target_path = target_root.join(relative_path);

                if source_path.is_dir() {
                    // Copy directories recursively
                    if let Err(e) = copy_dir_all(&source_path, &target_path) {
                        eprintln!(
                            "Failed to copy directory '{:?}' to '{:?}': {}",
                            source_path, target_path, e
                        );
                    } else {
                        println!("Copied directory '{:?}' to '{:?}'", source_path, target_path);
                    }
                } else {
                    // Copy files
                    if let Err(e) = fs::copy(&source_path, &target_path) {
                        eprintln!(
                            "Failed to copy file '{:?}' to '{:?}': {}",
                            source_path, target_path, e
                        );
                    } else {
                        println!("Copied file '{:?}' to '{:?}'", source_path, target_path);
                    }
                }
            }
        }
    } else {
        eprintln!(
            "Error: Failed to read the source directory '{}'. Ensure it exists.",
            source_dir.display()
        );
        return;
    }

    // Create symbolic links in the home directory
    println!("Creating symbolic links in the home directory...");
    if let Ok(entries) = fs::read_dir(&target_root) {
        for entry in entries {
            if let Ok(entry) = entry {
                let source_path = entry.path();
                let relative_path = source_path.strip_prefix(&target_root).unwrap();
                let link_path = home_dir_path.join(relative_path);

                // Replace existing files or links
                if link_path.exists() {
                    if let Err(e) = fs::remove_file(&link_path) {
                        eprintln!(
                            "Failed to remove existing file or link '{:?}': {}",
                            link_path, e
                        );
                        continue;
                    }
                    println!("Removed existing file or link: {:?}", link_path);
                }

                // Create the symbolic link
                if let Err(e) = symlink(&source_path, &link_path) {
                    eprintln!(
                        "Failed to create symbolic link from '{:?}' to '{:?}': {}",
                        source_path, link_path, e
                    );
                } else {
                    println!("Created symbolic link from '{:?}' to '{:?}'", source_path, link_path);
                }
            }
        }
    } else {
        eprintln!(
            "Error: Failed to read the target directory '{:?}'. Ensure it exists.",
            target_root
        );
    }
}

/// Recursively copy a directory and its contents
fn copy_dir_all(src: &Path, dst: &Path) -> std::io::Result<()> {
    if !dst.exists() {
        fs::create_dir_all(dst)?;
    }
    for entry in fs::read_dir(src)? {
        let entry = entry?;
        let entry_path = entry.path();
        let target_path = dst.join(entry.file_name());
        if entry_path.is_dir() {
            copy_dir_all(&entry_path, &target_path)?;
        } else {
            fs::copy(&entry_path, &target_path)?;
        }
    }
    Ok(())
}
4
