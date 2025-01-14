# Rosetta Installer and File Linker

This tool is designed to install Rosetta on macOS systems (for Apple Silicon Macs) and link files from a source directory to a specified target home directory.

---

## Features

1. **Rosetta Installation**:
   - Automatically installs Rosetta on Apple Silicon Macs if it's not already installed.

2. **File Linking**:
   - Links files and directories from the `files_link` directory to the root of a given home directory.

---

## Prerequisites

- Rust installed on your system (for building the binary).
- A Unix-like operating system (macOS, Ubuntu, Debian, Red Hat, or CentOS).

---

## Usage

### Command-Line Argument

- **Home Directory (Required)**: The directory where files and directories from `files_link` will be linked.

### Command Format

```bash
./your_binary <home_directory>
```

#### Example:

To link files to `/Users/username`, run:

```bash
./your_binary /Users/username
```

---

## What It Does

1. **Detects the OS**:
   - Determines the operating system and adjusts behavior accordingly.

2. **Installs Rosetta**:
   - For macOS on Apple Silicon, installs Rosetta if needed.

3. **Links Files**:
   - Iterates through files and directories in the `files_link` directory.
   - Ensures all directories from `files_link` exist in the target home directory.
   - Creates symbolic links for all files and directories.

---

## Configuration

Configuration files are located in the `config/` directory and are specific to the operating system. The `macos.toml` file, for example, specifies:

```toml
[settings]
install_rosetta = true
link_files = true

[file_locations]
textfiles_dir = "files_link"   # Directory containing files to be linked
link_target_dir = "<home_directory>" # Target directory for symbolic links at the root of the home directory
```

---

## Notes

- If a file or directory already exists in the target directory, the program will skip creating the link and issue a warning.
- Ensure the specified home directory has write permissions.

---

## Building the Tool

1. Clone the repository:

   ```bash
   git clone <repository-url>
   cd <repository>
   ```

2. Build the binary:

   ```bash
   cargo build --release
   ```

3. The binary will be located in `target/release/your_binary_name`.

---

## License

This project is licensed under the Apache License, Version 2.0. See the `LICENSE` file for details.
