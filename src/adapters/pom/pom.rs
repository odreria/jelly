use crate::core::gdp::dependency:: pom_managment::PomManagment;
use crate::core::gdp::models::dependency::Project;

use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml::de::Error as TomlError;
use quick_xml::de::from_str;

pub struct Pom;

impl PomManagment for Pom {
    
    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies, TomlError> {
        let content = fs::read_to_string(file_path).expect("No se logro leer el archivo");
        let dependencies: TomlDependencies = toml::de::from_str(&content)?;
        Ok(dependencies)
    }

    fn parse_pom(&self, xml: &str) -> Project {
        from_str(xml).unwrap()
    }
}


#[derive(Debug, Deserialize, PartialEq)]
pub struct DependencyDetail {
    pub file_name: String,
    pub url_jar: String,
    pub url_pom: String,
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

    pub fn values_to_vec(&self) -> Vec<DependencyDetail> {
        let mut vec: Vec<DependencyDetail> = Vec::new();

        for (artifact, version) in &self.dependencies {
            vec.push(Self::parse_dependency(&artifact, &version));
        }

        vec
    }

    pub fn parse_dependency(artifact: &str, version: &str) -> DependencyDetail {
        let parts: Vec<&str> = artifact.split(':').collect();
        let group_id = parts[0].replace('.', "/");
        let artifact_id = parts[1];
        let file_name = format!("{}-{}.jar", artifact_id, version);
        let url_jar = Self::get_url_maven_format(&group_id, &artifact_id, &version, "jar");
        let url_pom = Self::get_url_maven_format(&group_id, &artifact_id, &version, "pom");

        DependencyDetail {
            file_name,
            url_jar,
            url_pom,
        }
    }

    pub fn get_url_maven_format(group_id: &str, artifact_id: &str, version: &str, extension: &str) -> String {
        let file_name = format!("{}-{}.{}", artifact_id, version, extension);
        format!(
            "https://repo1.maven.org/maven2/{}/{}/{}/{}",
            group_id,
            artifact_id,
            version,
            file_name
        )
    }
}