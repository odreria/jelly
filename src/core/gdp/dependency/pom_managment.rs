use crate::{adapters::pom::pom::TomlDependencies, core::gdp::models::dependency::Project};

use toml::de::Error as TomlError;

pub trait PomManagment {
    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies, TomlError>;
    fn parse_pom(&self, xml: &str) -> Project;
}