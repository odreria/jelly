use crate::{adapters::pom::pom::TomlDependencies, core::gdp::models::dependency::Project};

use std::{future::Future, path::Path, pin::Pin};
use toml::de::Error as TomlError;

pub trait PomManagment {
    fn download_dependencies(url: &str, path: &Path) ->  Pin<Box<dyn Future<Output = Result<(), reqwest::Error>> + Send>>;
    fn download_pom(url: &str) -> Pin<Box<dyn Future<Output = Result<String, reqwest::Error>> + Send>>;
    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies, TomlError>;
    fn parse_pom(&self, xml: &str) -> Project;
}