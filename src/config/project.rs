use std::fs;
use std::path::Path;

use crate::errors::*;
use serde::Deserialize;

static CARGO_TOML: &str = "Cargo.toml";
static PACKAGE_JSON: &str = "package.json";

#[derive(Debug, Deserialize)]
pub struct ProjectConfig {
    pub name: String,
    pub description: String,
    pub homepage: Option<String>,
}

impl ProjectConfig {
    pub fn load() -> Result<Option<ProjectConfig>> {
        if let Some(ptype) = ProjectConfig::detect() {
            match ptype {
                Type::JavaScript(project) => Ok(Some(project.read()?)),
                Type::Rust(project) => Ok(Some(project.read()?)),
            }
        } else {
            Ok(None)
        }
    }

    fn detect() -> Option<Type> {
        if Rust::config().exists() {
            Some(Type::Rust(Rust {}))
        } else if JavaScript::config().exists() {
            Some(Type::JavaScript(JavaScript {}))
        } else {
            None
        }
    }
}

enum Type {
    Rust(Rust),
    JavaScript(JavaScript),
}

struct Rust {}
impl Rust {
    fn read(&self) -> Result<ProjectConfig> {
        let cargo_toml = fs::read_to_string(CARGO_TOML)?;
        let data: ProjectConfig = toml::from_str(&cargo_toml)?;
        Ok(data)
    }

    fn config() -> &'static Path {
        Path::new(CARGO_TOML)
    }
}

struct JavaScript {}
impl JavaScript {
    fn read(&self) -> Result<ProjectConfig> {
        let package_json = fs::read_to_string(PACKAGE_JSON)?;
        let data: ProjectConfig = serde_json::from_str(&package_json)?;
        Ok(data)
    }

    fn config() -> &'static Path {
        Path::new(PACKAGE_JSON)
    }
}
