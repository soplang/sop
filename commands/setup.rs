use anyhow::{anyhow, Result};
use colored::Colorize;
use std::fs;
use std::path::Path;

use crate::toml_parser::read_sop_toml;
use crate::utils::{ensure_dir_exists, file_exists, get_sop_modules_path, get_sop_toml_path};

/// Execute the setup command
pub fn execute() -> Result<()> {
    // Check if sop.toml exists
    let sop_toml_path = get_sop_toml_path();
    if !file_exists(&sop_toml_path) {
        return Err(anyhow!(
            "sop.toml not found. Are you in a Soplang project directory? Run 'sop init' to create a new project."
        ));
    }

    // Read the sop.toml file
    let config = read_sop_toml(&sop_toml_path)?;

    // Print project info
    println!("{} {}", "Project:".green().bold(), config.project.name);

    // Create sop_modules directory if it doesn't exist
    let modules_dir = get_sop_modules_path();
    ensure_dir_exists(&modules_dir)?;

    // Install dependencies if there are any
    match &config.dependencies {
        Some(dependencies) if !dependencies.is_empty() => {
            println!("{}", "Installing dependencies...".blue().bold());

            for (package, version) in dependencies {
                install_package(&package, &version, &modules_dir)?;
            }

            println!(
                "{} Successfully installed all dependencies.",
                "✓".green().bold()
            );
        }
        _ => {
            println!("{}", "No dependencies specified in sop.toml.".yellow());
        }
    }

    Ok(())
}

/// Install a single package
fn install_package(package: &str, version: &str, modules_dir: &Path) -> Result<()> {
    println!("Installing {} v{}", package, version);

    // Create a directory for the package
    let package_dir = modules_dir.join(package);

    // If package already exists, check if it's the right version
    if package_dir.exists() {
        println!("  {} {} is already installed", "✓".yellow(), package);
        // In a real implementation, we would check version compatibility here
        return Ok(());
    }

    ensure_dir_exists(&package_dir)?;

    // For now, we'll just create a placeholder file
    // In a real implementation, this would download the package from a registry
    let metadata_file = package_dir.join("sop.toml");
    let metadata_content = format!(
        r#"[package]
name = "{}"
version = "{}"
description = "A Soplang package"
"#,
        package, version
    );

    fs::write(metadata_file, metadata_content)?;

    // Create a simple placeholder .so file
    let lib_file = package_dir.join("lib.so");
    let lib_content = format!(
        r#"// This is a placeholder for the {} library

export fn hello() {{
    println("Hello from {}!");
}}
"#,
        package, package
    );

    fs::write(lib_file, lib_content)?;

    println!("  {} {}", "✓".green(), package);

    Ok(())
}
