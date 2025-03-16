use anyhow::{anyhow, Result};
use std::fs;
use std::path::{Path, PathBuf};

/// Check if a file exists at the specified path
pub fn file_exists(path: &Path) -> bool {
    path.exists() && path.is_file()
}

/// Check if a directory exists at the specified path
pub fn dir_exists(path: &Path) -> bool {
    path.exists() && path.is_dir()
}

/// Create a directory if it doesn't exist
pub fn ensure_dir_exists(path: &Path) -> Result<()> {
    if !path.exists() {
        fs::create_dir_all(path)?;
    }
    Ok(())
}

/// Get the path to the sop.toml file in the current directory
pub fn get_sop_toml_path() -> PathBuf {
    PathBuf::from("sop.toml")
}

/// Check if the current directory is a Soplang project by checking for sop.toml
pub fn is_soplang_project() -> bool {
    file_exists(&get_sop_toml_path())
}

/// Ensure we're in a Soplang project directory
pub fn ensure_in_project() -> Result<()> {
    if !is_soplang_project() {
        return Err(anyhow!(
            "Not in a Soplang project. Run 'sop init' to create a new project."
        ));
    }
    Ok(())
}

/// Get the path to the sop_modules directory
pub fn get_sop_modules_path() -> PathBuf {
    PathBuf::from("sop_modules")
}

/// Get the path to the src directory
pub fn get_src_path() -> PathBuf {
    PathBuf::from("src")
}

/// Create a pretty printed error message
pub fn format_error(msg: &str) -> String {
    format!("Error: {}", msg)
}

/// Create a pretty printed success message
pub fn format_success(msg: &str) -> String {
    format!("Success: {}", msg)
}

/// Create a pretty printed info message
pub fn format_info(msg: &str) -> String {
    format!("Info: {}", msg)
}
