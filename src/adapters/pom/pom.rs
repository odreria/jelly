use crate::core::gdp::dependency:: pom_managment::PomManagment;
use crate::core::gdp::models::dependency::{Dependency, Project};
use crate::errors::beetle_error::BeetleError;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use quick_xml::de::from_str;

pub struct Pom;

impl PomManagment for Pom {

    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies, BeetleError> {
        let content =
            fs::read_to_string(file_path).map_err(BeetleError::from)?;
        let dependencies: TomlDependencies = toml::de::from_str(&content)?;
        Ok(dependencies)
    }

    fn parse_pom(&self, xml: &str) -> Project {
        from_str(xml).unwrap()
    }
}

#[derive(Debug, Deserialize)]
pub struct TomlDependencies {
    pub dependencies: HashMap<String, String>,
}

impl TomlDependencies {
    pub fn new() -> Self {
        let new_map = HashMap::new();
        TomlDependencies { dependencies: new_map}
    }

    pub fn values_to_vec(&self) -> Vec<Dependency> {
        let mut vec: Vec<Dependency> = Vec::new();

        for (artifact, version) in &self.dependencies {
            vec.push(Self::parse_dependency(&artifact, &version));
        }

        vec
    }

    fn parse_dependency(artifact: &str, version: &str) -> Dependency {
        let parts: Vec<&str> = artifact.split(':').collect();
        let group_id = parts[0];
        let artifact_id = parts[1];

        Dependency::new(
            Some(group_id.to_string()),
            Some(artifact_id.to_string()),
            Some(version.to_string()))
    }

}