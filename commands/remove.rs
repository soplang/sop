use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;
use std::path::PathBuf;

use crate::toml_parser::{read_sop_toml, write_sop_toml};
use crate::utils::{file_exists, get_sop_modules_path, get_sop_toml_path};

/// Execute the remove command
pub fn execute(package: &str) -> Result<()> {
    println!("Removing package: {}", package);

    // Check if sop.toml exists
    let sop_toml_path = get_sop_toml_path();
    if !file_exists(&sop_toml_path) {
        return Err(anyhow!(
            "sop.toml not found. Are you in a Soplang project directory? Run 'sop init' to create a new project."
        ));
    }

    // Read the sop.toml file
    let mut config = read_sop_toml(&sop_toml_path)?;

    // Check if dependencies section exists
    if config.dependencies.is_none() {
        return Err(anyhow!("No dependencies found in sop.toml."));
    }

    // Check if the package is in dependencies
    let dependencies = config.dependencies.as_mut().unwrap();
    if !dependencies.contains_key(package) {
        return Err(anyhow!(
            "Package '{}' not found in your dependencies.",
            package
        ));
    }

    // Remove the package from dependencies
    dependencies.remove(package);

    // Write updated config back to sop.toml
    write_sop_toml(&sop_toml_path, &config)?;

    // Remove the package directory
    let package_dir = get_sop_modules_path().join(package);
    if package_dir.exists() {
        fs::remove_dir_all(&package_dir)?;
    } else {
        println!(
            "  {} Package directory not found, skipping deletion",
            "!".yellow()
        );
    }

    println!(
        "{} Removed {} from dependencies",
        "✓".green().bold(),
        package
    );

    Ok(())
}
