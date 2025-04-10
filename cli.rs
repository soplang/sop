use crate::commands;
use anyhow::Result;
use clap::{Parser, Subcommand};

#[derive(Parser)]
#[command(name = "sop")]
#[command(author, version, about = "Official package manager for Soplang", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Initialize a new Soplang project
    Init {
        /// Skip interactive prompts and use default values
        #[arg(short = 'y', long)]
        yes: bool,
    },

    /// Install dependencies from sop.toml
    Setup,

    /// Add a package to the project
    Add {
        /// Package name to add
        package: String,

        /// Specific version to install
        #[arg(short, long)]
        version: Option<String>,
    },

    /// Remove a package from the project
    Remove {
        /// Package name to remove
        package: String,
    },

    /// Run a Soplang script
    Run {
        /// Path to the script (defaults to entry in sop.toml)
        script: Option<String>,
    },

    /// Update project dependencies
    Update {
        /// Specific package to update (updates all if not specified)
        package: Option<String>,
    },

    /// List installed packages
    List,

    /// Show information about a package
    Info {
        /// Package name
        package: String,
    },

    /// Clean project by removing sop_modules directory
    Clean,

    /// Validate sop.toml file
    Check,
}

impl Cli {
    pub fn execute(&self) -> Result<()> {
        match &self.command {
            Some(Commands::Init { yes }) => commands::init::execute(*yes),
            Some(Commands::Setup) => commands::setup::execute(),
            Some(Commands::Add { package, version }) => commands::add::execute(package, version),
            Some(Commands::Remove { package }) => commands::remove::execute(package),
            Some(Commands::Run { script }) => commands::run::execute(script),
            Some(Commands::Update { package }) => commands::update::execute(package),
            Some(Commands::List) => {
                println!("Command 'list' not yet implemented");
                // Will call commands::list::execute() once implemented
                Ok(())
            }
            Some(Commands::Info { package }) => {
                println!("Command 'info' not yet implemented");
                // Will call commands::info::execute(package) once implemented
                Ok(())
            }
            Some(Commands::Clean) => {
                println!("Command 'clean' not yet implemented");
                // Will call commands::clean::execute() once implemented
                Ok(())
            }
            Some(Commands::Check) => {
                println!("Command 'check' not yet implemented");
                // Will call commands::check::execute() once implemented
                Ok(())
            }
            None => {
                println!("No command specified. Run 'sop --help' for usage information.");
                Ok(())
            }
        }
    }
}
