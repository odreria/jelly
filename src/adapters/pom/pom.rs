use crate::core::gdp::dependency:: pom_managment::PomManagment;
use crate::core::gdp::models::dependency::Project;

use serde::Deserialize;
use std::collections::HashMap;
use std::future::Future;
use std::pin::Pin;
use reqwest::get;
use std::io::copy;
use std::fs::File;
use std::path::Path;
use std::fs;
use toml::de::Error as TomlError;
use quick_xml::de::from_str;

pub struct Pom;

impl PomManagment for Pom {

    fn download_dependencies(url: &str, path: &Path) -> Pin<Box<dyn Future<Output = Result<(), reqwest::Error>> + Send>> {

        Box::pin(async move { 

            let response = get(url).await?;
            let mut file = File::create(path).expect("");
            copy(&mut response.bytes().await?.as_ref(), &mut file).expect("Dependency cannot be copied.");
            Ok(())
            
        })
    }

    fn download_pom(url: &str) -> Pin<Box<dyn Future<Output = Result<String, reqwest::Error>> + Send>> {

        Box::pin(async move {

            let content_req = get(url).await?;
            let content = content_req.text().await?;
            Ok(content)

        })
    }

    fn read_toml_file(&self, file_path: &str) -> Result<TomlDependencies, TomlError> {
        let content = fs::read_to_string(file_path).expect("No se logro leer el archivo");
        let dependencies: TomlDependencies = toml::de::from_str(&content)?;
        Ok(dependencies)
    }

    fn parse_pom(&self, xml: &str) -> Project {
        from_str(xml).unwrap()
    }
}


#[derive(Debug, Deserialize)]
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