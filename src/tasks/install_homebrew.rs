use std::process::Command;

pub fn install_homebrew() -> Result<(), String> {
    if is_homebrew_installed() {
        println!("Homebrew is already installed.");
        return Ok(());
    }

    let os = detect_os();
    println!("Detected OS: {}", os);

    let install_script = match os.as_str() {
        "macos" => {
            "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
        }
        "linux" => {
            "https://raw.githubusercontent.com/Homebrew/install/HEAD/install.sh"
        }
        _ => return Err(format!("Unsupported OS for Homebrew installation: {}", os)),
    };

    println!("Installing Homebrew...");
    let status = Command::new("sh")
        .arg("-c")
        .arg(format!("curl -fsSL {} | bash", install_script))
        .status()
        .map_err(|e| format!("Failed to execute Homebrew install script: {}", e))?;

    if !status.success() {
        return Err("Homebrew installation failed.".to_string());
    }

    println!("Homebrew installation completed successfully.");
    Ok(())
}

fn is_homebrew_installed() -> bool {
    Command::new("brew")
        .arg("--version")
        .status()
        .map(|status| status.success())
        .unwrap_or(false)
}

fn detect_os() -> String {
    if cfg!(target_os = "macos") {
        "macos".to_string()
    } else if cfg!(target_os = "linux") {
        "linux".to_string()
    } else {
        "unknown".to_string()
    }
}
