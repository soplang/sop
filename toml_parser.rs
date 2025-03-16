use anyhow::{anyhow, Result};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

/// Structure representing the sop.toml file
#[derive(Debug, Serialize, Deserialize)]
pub struct SopToml {
    pub project: ProjectConfig,
    pub dependencies: Option<HashMap<String, String>>,
}

/// Project configuration section of sop.toml
#[derive(Debug, Serialize, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub version: String,
    pub status: String,
    #[serde(default)]
    pub description: String,
    #[serde(default)]
    pub license: String,
    #[serde(default)]
    pub author: String,
    #[serde(default)]
    pub repository: String,
    #[serde(default)]
    pub homepage: String,
    pub entry: String,
    #[serde(default)]
    pub keywords: Vec<String>,
    #[serde(default)]
    pub categories: Vec<String>,
}

/// Read and parse a sop.toml file
pub fn read_sop_toml(path: &Path) -> Result<SopToml> {
    if !path.exists() {
        return Err(anyhow!("sop.toml file not found at {:?}", path));
    }

    let content = fs::read_to_string(path)?;
    let config: SopToml = toml::from_str(&content)?;
    Ok(config)
}

/// Write a SopToml structure to a sop.toml file
pub fn write_sop_toml(path: &Path, config: &SopToml) -> Result<()> {
    let content = toml::to_string_pretty(config)?;
    fs::write(path, content)?;
    Ok(())
}

/// Create a default SopToml configuration
pub fn create_default_config(name: &str) -> SopToml {
    SopToml {
        project: ProjectConfig {
            name: name.to_string(),
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
        dependencies: Some(HashMap::new()),
    }
}
