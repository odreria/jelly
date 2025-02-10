use crate::{adapters::pom::pom::TomlDependencies, core::gdp::models::dependency::Project, errors::beetle_error::BeetleError};

pub trait PomManagment {
    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies, BeetleError>;
    fn parse_pom(&self, xml: &str) -> Project;
}