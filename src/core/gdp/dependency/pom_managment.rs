use crate::{adapters::pom::pom::TomlDependencies, core::gdp::models::dependency::Project, errors::Result};

pub trait PomManagment {
    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies>;
    fn parse_pom(&self, xml: &str) -> Project;
}