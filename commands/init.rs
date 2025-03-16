use anyhow::{anyhow, Result};
use dialoguer::{Confirm, Input};
use std::fs;
use std::path::{Path, PathBuf};

use crate::toml_parser::{write_sop_toml, ProjectConfig, SopToml};
use crate::utils::{ensure_dir_exists, file_exists, get_sop_toml_path, get_src_path};

/// Execute the init command
pub fn execute(yes: bool) -> Result<()> {
    // Check if sop.toml already exists
    let sop_toml_path = get_sop_toml_path();
    if file_exists(&sop_toml_path) {
        if !yes
            && !Confirm::new()
                .with_prompt("A sop.toml file already exists. Overwrite?")
                .default(false)
                .interact()?
        {
            return Err(anyhow!("Initialization aborted."));
        }
    }

    // Create the project configuration
    let config = if yes {
        // Use default values if -y flag is provided
        create_default_project()?
    } else {
        // Ask for project details interactively
        create_interactive_project()?
    };

    // Create directories
    let src_path = get_src_path();
    ensure_dir_exists(&src_path)?;

    // Create sop.toml
    write_sop_toml(&sop_toml_path, &config)?;

    // Create src/main.so
    let main_file_path = src_path.join("main.so");
    create_main_file(&main_file_path)?;

    println!(
        "Successfully initialized a new Soplang project: {}",
        config.project.name
    );
    println!("Created:");
    println!("  sop.toml");
    println!("  src/main.so");

    Ok(())
}

/// Create a default project configuration
fn create_default_project() -> Result<SopToml> {
    // Use current directory name as project name
    let current_dir = std::env::current_dir()?;
    let dir_name = current_dir
        .file_name()
        .ok_or_else(|| anyhow!("Unable to determine current directory name"))?
        .to_string_lossy()
        .to_string();

    Ok(SopToml {
        project: ProjectConfig {
            name: dir_name,
            version: "0.1.0".to_string(),
            description: Some("A Soplang project".to_string()),
            author: None,
            entry: Some("src/main.so".to_string()),
        },
        dependencies: Some(std::collections::HashMap::new()),
    })
}

/// Create a project configuration interactively
fn create_interactive_project() -> Result<SopToml> {
    // Use current directory name as default project name
    let current_dir = std::env::current_dir()?;
    let dir_name = current_dir
        .file_name()
        .ok_or_else(|| anyhow!("Unable to determine current directory name"))?
        .to_string_lossy()
        .to_string();

    let name: String = Input::new()
        .with_prompt("Project name")
        .default(dir_name)
        .interact_text()?;

    let version: String = Input::new()
        .with_prompt("Version")
        .default("0.1.0".to_string())
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("Description")
        .default("A Soplang project".to_string())
        .interact_text()?;

    let author: String = Input::new()
        .with_prompt("Author")
        .allow_empty(true)
        .interact_text()?;

    let entry: String = Input::new()
        .with_prompt("Entry file")
        .default("src/main.so".to_string())
        .interact_text()?;

    Ok(SopToml {
        project: ProjectConfig {
            name,
            version,
            description: Some(description),
            author: if author.is_empty() {
                None
            } else {
                Some(author)
            },
            entry: Some(entry),
        },
        dependencies: Some(std::collections::HashMap::new()),
    })
}

/// Create the main.so file with a simple hello world program
fn create_main_file(path: &Path) -> Result<()> {
    let content = r#"// This is the main entry point for your Soplang project

fn main() {
    println("Hello, Soplang world!");
}
"#;

    fs::write(path, content)?;
    Ok(())
}
