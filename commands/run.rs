use anyhow::{anyhow, Result};
use colored::Colorize;
use std::path::Path;
use std::process::Command;

use crate::toml_parser::read_sop_toml;
use crate::utils::{file_exists, get_sop_toml_path};

/// Execute the run command
pub fn execute(script_path: &Option<String>) -> Result<()> {
    // Check if sop.toml exists
    let sop_toml_path = get_sop_toml_path();
    if !file_exists(&sop_toml_path) {
        return Err(anyhow!(
            "sop.toml not found. Are you in a Soplang project directory? Run 'sop init' to create a new project."
        ));
    }

    // Read the sop.toml file
    let config = read_sop_toml(&sop_toml_path)?;

    // Determine which script to run
    let script_to_run = match script_path {
        Some(path) => path.clone(),
        None => config.project.entry,
    };

    // Check if the script exists
    let script_file = Path::new(&script_to_run);
    if !file_exists(script_file) {
        return Err(anyhow!("Script file not found: {}", script_to_run));
    }

    println!("Running Soplang script: {}", script_to_run);

    // In a real implementation, this would call the Soplang interpreter
    // For the mock implementation, we'll just print the script contents
    println!("{}", "=".repeat(40));
    println!("{}", std::fs::read_to_string(script_file)?);
    println!("{}", "=".repeat(40));

    // Simulate running the Soplang script
    println!("\n{} Script executed successfully", "✓".green().bold());

    // In a real implementation, this would look something like:
    // let status = Command::new("soplang")
    //     .arg(script_file)
    //     .status()?;
    //
    // if !status.success() {
    //     return Err(anyhow!("Script execution failed"));
    // }

    Ok(())
}
