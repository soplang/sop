use anyhow::{anyhow, Result};
use dialoguer::{Confirm, Input};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

use crate::toml_parser::{write_sop_toml, ProjectConfig, SopToml};
use crate::utils::{ensure_dir_exists, file_exists, get_sop_toml_path, get_src_path};

/// Execute the init command
pub fn execute(yes: bool) -> Result<()> {
    // Check if current directory is empty
    let current_dir = env::current_dir()?;
    let entries: Vec<_> = fs::read_dir(&current_dir)?.collect::<Result<Vec<_>, _>>()?;
    let is_empty = entries.is_empty();

    // If directory is not empty and -y is specified, throw an error
    if !is_empty && yes {
        return Err(anyhow!("Current directory is not empty. Cannot initialize with -y flag. Use interactive mode or empty the directory."));
    }

    let project_dir: PathBuf;
    let project_name: String;

    if is_empty {
        // Current directory is empty, initialize here
        project_dir = current_dir;
        project_name = project_dir
            .file_name()
            .ok_or_else(|| anyhow!("Unable to determine current directory name"))?
            .to_string_lossy()
            .to_string();
    } else {
        // Current directory is not empty, ask for a folder name
        project_name = Input::new()
            .with_prompt(
                "Current directory is not empty. Enter a new directory name for the project",
            )
            .interact_text()?;

        project_dir = current_dir.join(&project_name);

        if project_dir.exists() {
            if !Confirm::new()
                .with_prompt(format!(
                    "Directory '{}' already exists. Do you want to overwrite it?",
                    project_name
                ))
                .default(false)
                .interact()?
            {
                return Err(anyhow!("Initialization aborted."));
            }
            // If confirmed, delete the existing directory
            fs::remove_dir_all(&project_dir)?;
        }

        // Create the project directory
        fs::create_dir_all(&project_dir)?;
    }

    // Change to the project directory
    env::set_current_dir(&project_dir)?;

    // Check if sop.toml already exists (shouldn't happen in a new directory, but just in case)
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
        // Use default values if -y flag is provided (only in empty directory)
        create_default_project(&project_name)?
    } else {
        // Ask for project details interactively
        create_interactive_project(&project_name)?
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
    if !is_empty {
        println!("  {}/", project_name);
    }
    println!("  sop.toml");
    println!("  src/main.so");

    Ok(())
}

/// Create a default project configuration
fn create_default_project(project_name: &str) -> Result<SopToml> {
    Ok(SopToml {
        project: ProjectConfig {
            name: project_name.to_string(),
            version: "1.0.0".to_string(),
            status: "stable".to_string(),
            description: String::new(),
            license: String::new(),
            author: String::new(),
            repository: String::new(),
            homepage: String::new(),
            entry: "src/main.so".to_string(),
            keywords: Vec::new(),
            categories: Vec::new(),
        },
        dependencies: Some(std::collections::HashMap::new()),
    })
}

/// Create a project configuration interactively
fn create_interactive_project(default_name: &str) -> Result<SopToml> {
    let name: String = Input::new()
        .with_prompt("Project name")
        .default(default_name.to_string())
        .interact_text()?;

    let version: String = Input::new()
        .with_prompt("Version")
        .default("1.0.0".to_string())
        .interact_text()?;

    let status: String = Input::new()
        .with_prompt("Status")
        .default("stable".to_string())
        .interact_text()?;

    let description: String = Input::new()
        .with_prompt("Description")
        .allow_empty(true)
        .interact_text()?;

    let license: String = Input::new()
        .with_prompt("License")
        .allow_empty(true)
        .interact_text()?;

    let author: String = Input::new()
        .with_prompt("Author")
        .allow_empty(true)
        .interact_text()?;

    let repository: String = Input::new()
        .with_prompt("Repository")
        .allow_empty(true)
        .interact_text()?;

    let homepage: String = Input::new()
        .with_prompt("Homepage")
        .allow_empty(true)
        .interact_text()?;

    // Entry is fixed as src/main.so
    let entry = "src/main.so".to_string();

    Ok(SopToml {
        project: ProjectConfig {
            name,
            version,
            status,
            description,
            license,
            author,
            repository,
            homepage,
            entry,
            keywords: Vec::new(),
            categories: Vec::new(),
        },
        dependencies: Some(std::collections::HashMap::new()),
    })
}

/// Create the main.so file with a simple hello world program
fn create_main_file(path: &Path) -> Result<()> {
    let content = r#"// This is the main entry point for your Soplang project

howl main() {
    qor('Hello World!');
}
"#;

    fs::write(path, content)?;
    Ok(())
}
