use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;

use crate::toml_parser::{read_sop_toml, write_sop_toml};
use crate::utils::{ensure_dir_exists, file_exists, get_sop_modules_path, get_sop_toml_path};

/// Execute the add command
pub fn execute(package: &str, version: &Option<String>) -> Result<()> {
    // Resolve the version
    let version_str = match version {
        Some(v) => v.clone(),
        None => "latest".to_string(), // Default to latest version
    };

    println!("Adding package: {} ({})", package, version_str);

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
        config.dependencies = Some(std::collections::HashMap::new());
    }

    // Check if the package is already in dependencies
    let dependencies = config.dependencies.as_mut().unwrap();
    if dependencies.contains_key(package) {
        return Err(anyhow!(
            "Package '{}' is already in your dependencies.",
            package
        ));
    }

    // Add the package to dependencies
    dependencies.insert(package.to_string(), version_str.clone());

    // Write updated config back to sop.toml
    write_sop_toml(&sop_toml_path, &config)?;

    // Install the package
    let modules_dir = get_sop_modules_path();
    ensure_dir_exists(&modules_dir)?;
    install_package(package, &version_str, &modules_dir)?;

    println!(
        "{} Added {} ({}) to dependencies",
        "✓".green().bold(),
        package,
        version_str
    );

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

    std::fs::write(metadata_file, metadata_content)?;

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

    std::fs::write(lib_file, lib_content)?;

    println!("  {} {}", "✓".green(), package);

    Ok(())
}
