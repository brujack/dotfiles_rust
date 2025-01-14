use std::process::Command;

pub fn run() {
    println!("Checking for Rosetta installation...");

    let check_rosetta = Command::new("/usr/bin/pgrep")
        .arg("oahd")
        .output();

    match check_rosetta {
        Ok(output) if !output.stdout.is_empty() => {
            println!("Rosetta is already installed.");
        }
        _ => {
            println!("Installing Rosetta...");
            let install_rosetta = Command::new("/usr/sbin/softwareupdate")
                .arg("--install-rosetta")
                .arg("--agree-to-license")
                .status();

            match install_rosetta {
                Ok(status) if status.success() => {
                    println!("Rosetta has been successfully installed.");
                }
                _ => {
                    eprintln!("Failed to install Rosetta. Please try again manually.");
                }
            }
        }
    }
}
