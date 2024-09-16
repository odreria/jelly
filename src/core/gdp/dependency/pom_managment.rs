use std::path::Path;
use toml::de::Error as TomlError;

use crate::{adapters::pom::pom::TomlDependencies, core::gdp::models::dependency::Project};


pub trait PomManagment {
    async fn download_dependencies(url: &str, path: &Path) -> Result<(), reqwest::Error>;
    async fn download_pom(url: &str) -> Result<String, reqwest::Error>;
    fn read_toml_file(file_path: &str) -> Result<TomlDependencies, TomlError>;
    fn parse_pom(xml: &str) -> Project;
}