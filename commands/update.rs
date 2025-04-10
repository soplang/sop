use anyhow::{anyhow, Result};
use colored::Colorize;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

use crate::toml_parser::{read_sop_toml, write_sop_toml};
use crate::utils::{ensure_dir_exists, file_exists, get_sop_modules_path, get_sop_toml_path};

/// Execute the update command
pub fn execute(package: &Option<String>) -> Result<()> {
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
    if config.dependencies.is_none() || config.dependencies.as_ref().unwrap().is_empty() {
        println!("{}", "No dependencies specified in sop.toml.".yellow());
        return Ok(());
    }

    let dependencies = config.dependencies.as_mut().unwrap();
    let modules_dir = get_sop_modules_path();
    ensure_dir_exists(&modules_dir)?;

    // If a specific package is specified, only update that package
    if let Some(pkg_name) = package {
        if !dependencies.contains_key(pkg_name) {
            return Err(anyhow!(
                "Package '{}' not found in your dependencies.",
                pkg_name
            ));
        }

        let version = dependencies.get(pkg_name).unwrap().clone();
        println!(
            "Checking for updates for {} (current: {})",
            pkg_name, version
        );

        // Get the latest version (in a real implementation, this would check a registry)
        let latest_version = check_latest_version(pkg_name, &version)?;

        if latest_version == version {
            println!("  {} {} is already up to date", "✓".green(), pkg_name);
        } else {
            // Update the dependency in sop.toml
            dependencies.insert(pkg_name.clone(), latest_version.clone());

            // Remove old version
            let package_dir = modules_dir.join(pkg_name);
            if package_dir.exists() {
                fs::remove_dir_all(&package_dir)?;
            }

            // Install new version
            install_package(pkg_name, &latest_version, &modules_dir)?;
            println!(
                "  {} Updated {} to version {}",
                "✓".green(),
                pkg_name,
                latest_version
            );
        }
    } else {
        // Update all dependencies
        println!("Checking for updates for all dependencies...");
        let mut updated_count = 0;

        // Create a copy of dependencies to iterate through
        let deps_to_update: HashMap<String, String> = dependencies.clone();

        for (pkg_name, version) in deps_to_update {
            println!("Checking {} (current: {})", pkg_name, version);

            // Get the latest version
            let latest_version = check_latest_version(&pkg_name, &version)?;

            if latest_version == version {
                println!("  {} {} is already up to date", "✓".green(), pkg_name);
            } else {
                // Update the dependency in sop.toml
                dependencies.insert(pkg_name.clone(), latest_version.clone());

                // Remove old version
                let package_dir = modules_dir.join(&pkg_name);
                if package_dir.exists() {
                    fs::remove_dir_all(&package_dir)?;
                }

                // Install new version
                install_package(&pkg_name, &latest_version, &modules_dir)?;
                println!(
                    "  {} Updated {} to version {}",
                    "✓".green(),
                    pkg_name,
                    latest_version
                );

                updated_count += 1;
            }
        }

        if updated_count > 0 {
            println!(
                "\n{} Updated {} packages",
                "✓".green().bold(),
                updated_count
            );
        } else {
            println!("\n{} All packages are up to date", "✓".green().bold());
        }
    }

    // Write updated config back to sop.toml
    write_sop_toml(&sop_toml_path, &config)?;

    Ok(())
}

/// Check for the latest version of a package
fn check_latest_version(package: &str, current_version: &str) -> Result<String> {
    // In a real implementation, this would check a registry
    // For simulation, we'll just increment the version number

    // Parse the version (assuming semver format: major.minor.patch)
    let version_parts: Vec<&str> = current_version.split('.').collect();

    // For simplicity, if the current version is "latest", we'll return a specific version
    if current_version == "latest" {
        return Ok("1.0.0".to_string());
    }

    if version_parts.len() != 3 {
        // If the version doesn't match semver format, just return a simulated new version
        return Ok("1.0.0".to_string());
    }

    // Try to parse each part as a number
    let major: u32 = version_parts[0].parse().unwrap_or(0);
    let minor: u32 = version_parts[1].parse().unwrap_or(0);
    let patch: u32 = version_parts[2].parse().unwrap_or(0);

    // Increment the patch version for the simulation
    let new_patch = patch + 1;

    // For simulation, 50% chance of having an update
    if rand::random() {
        Ok(format!("{}.{}.{}", major, minor, new_patch))
    } else {
        Ok(current_version.to_string())
    }
}

/// Install a single package
fn install_package(package: &str, version: &str, modules_dir: &Path) -> Result<()> {
    println!("Installing {} v{}", package, version);

    // Create a directory for the package
    let package_dir = modules_dir.join(package);
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

    Ok(())
}
