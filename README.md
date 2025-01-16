# Setup Environment (`setup_env`)

`setup_env` is a Rust-based utility for detecting the operating system, hostname, and loading appropriate configurations. It also provides options to install Rosetta on macOS, link files, and handle hostname-specific configurations.

---

## Features
- Detects the OS and hostname to load specific configuration files.
- Supports hostname-specific configuration overrides.
- Installs Rosetta on macOS (if configured).
- Links files to specified directories based on the configuration.
- Supports multiple platforms:
  - macOS (x86-64, ARM)
  - Linux (x86-64, ARM)

---

## Recent Changes
- Removed the requirement for `config/default.toml` and `config/macos.toml`. Now, only hostname-specific configuration files are used (e.g., `config/<hostname>-custom.toml`).
- Shortened the hostname detection to return the short hostname (e.g., `host` instead of `host.example.com`).
- Added support for flexible error handling using the `anyhow` crate.
- Ensured compatibility with macOS binaries by enabling code signing for macOS ARM and x86-64 binaries in GitHub Actions.
- Updated GitHub Actions to use the latest versions of artifact actions (`actions/upload-artifact@v4` and `actions/download-artifact@v4`).

---

## Dependencies
This project uses the following dependencies:

### Core Dependencies
- **`serde`**: For deserializing TOML configuration files.
- **`toml`**: To parse TOML configuration files.
- **`hostname`**: To retrieve the system's hostname.
- **`anyhow`**: For flexible error handling with detailed error context.

### Development Dependencies
- **`actions/upload-artifact@v4`**: For uploading build artifacts in GitHub Actions.
- **`actions/download-artifact@v4`**: For downloading build artifacts in GitHub Actions.
- **Apple Developer Certificate** (macOS):
  - Required for code signing macOS binaries. The certificate must be base64-encoded and stored in GitHub Secrets as `APPLE_CERTIFICATE` and `APPLE_CERT_PASSWORD`.

---

## Installation and Usage

### Prerequisites
1. Install Rust and Cargo using [rustup](https://rustup.rs/).
   ```bash
   curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
   ```
2. (Optional) Install additional dependencies for ARM builds on Linux:
   ```bash
   sudo apt-get update
   sudo apt-get install -y gcc-aarch64-linux-gnu libc6-dev-arm64-cross
   ```

---

## Build the Project
1. Clone the repository:
   ```bash
   git clone https://github.com/brujack/setup_env.git
   cd setup_env
   ```
2. Build the project for your platform:
   ```bash
   cargo build --release
   ```

## Run the Program
Run the program with:
```bash
cargo run -- /location/of/home/directory
```

---

## Configuration Files
1. Hostname-specific configuration files must be placed in the config/ directory.

   * Example: config/my-hostname-custom.toml
2. Configuration structure (TOML):
```toml
   [settings]
   install_rosetta = true
   link_files = true

   [file_locations]
   textfiles_dir = "/path/to/textfiles"
   link_target_dir = "/path/to/links"
```

---

## Code Signing for macOS Binaries

To ensure compatibility with macOS 15.x (Catalina) and later:

1. Use an Apple Developer ID certificate to sign binaries during GitHub Actions.
2. Store the certificate and its password as GitHub Secrets:
   * APPLE_CERTIFICATE: Base64-encoded .p12 certificate.
   * APPLE_CERT_PASSWORD: Password for the .p12 certificate.

---

## GitHub Actions

The project is set up with a CI/CD pipeline that:

1. Builds binaries for:
   * macOS (x86-64, ARM)
   * Linux (x86-64, ARM)
2. Signs macOS binaries using an Apple Developer Certificate.
3. Creates a GitHub release with platform-specific binaries:
   * setup_env-macos-x86-64
   * setup_env-macos-arm
   * setup_env-linux-x86-64
   * setup_env-linux-arm

---

## Troubleshooting
### Common Errors

1. Config file not found:
   * Ensure the required hostname-specific configuration file exists in the config/ directory.
2. Failed to parse config file:
   * Check the syntax of the TOML file for errors.
3. cargo run fails with context not found:
   * Ensure you have imported the anyhow::Context trait.
   ```bash
   cargo add anyhow
   ```

### Debugging Tips
* Use cargo run -- --debug for verbose output (if implemented).
* Check the logs in GitHub Actions for detailed error messages.

---

## License

This project is licensed under the Apache License, Version 2.0. See the `LICENSE` file for details.
