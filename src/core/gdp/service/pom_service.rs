use std::thread::scope;

use crate::{adapters::pom::pom::{DependencyDetail, TomlDependencies}, core::gdp::{dependency::pom_managment::PomManagment, models::dependency::{Dependencies, DependencyManagment}}};
use quick_xml::de;
use regex::Regex;
use reqwest::get;
pub struct PomService<R: PomManagment> {
    pub managment: R,
 }

impl<R: PomManagment> PomService<R> {
    pub fn new(managment: R) -> Self {
         PomService { managment }
    }

    pub async fn get_pom_details(&self, dependency_details: &Vec<DependencyDetail>) -> Result<TomlDependencies, reqwest::Error> {

        let mut toml_dependencies : TomlDependencies = TomlDependencies::new();

        for detail in dependency_details {
            let content_req = get(&detail.url_pom).await.expect("error al consultar el pom");
            let pom_content = content_req.text().await.expect("error al obtener el texto");
    
            let project_xml = self.managment.parse_pom(&pom_content);

            let dep_managment = match project_xml.dependency_managment {
                Some(value) => value,
                None => DependencyManagment {
                    dependencies: None
                }
            };


            if let Some(dep) = dep_managment.dependencies {

                let version = dep.dependency.expect("msg").version.expect("msg");
                let properties = project_xml.properties.expect("msg");

                let rep = Regex::new(r"\$\{([^}]+)\}").unwrap();
                let caps = rep.captures(&version).unwrap();        
                let raw_version = caps.get(1).map_or("", |m| m.as_str());

                println!("{:?}", &raw_version);
                println!("{:?}", properties.get(raw_version));

                // Here we should go to pom dependencies URL to get the sbom value,
                // after that, we need to go to "library"-SBOM to get the correct version.
            }

    
            let pom_dependency: Dependencies = match project_xml.dependencies {
                Some(value) => value.clone(),
                None => Dependencies {
                    dependencies: None
                }
            };
        
            if let Some(dep) = &pom_dependency.dependencies {
                for element in dep {
                    let group_id: String = match &element.group_id {
                        Some(value) => value.clone(),
                        None => String::from("")
                    };
        
                    let artifact: String = match &element.artifact_id {
                        Some(value) => value.clone(),
                        None => String::from("")
                    };
    
                    let opcional_dependency: String = match &element.optional {
                        Some(value) => value.clone(),
                        None => String::from("")
                    };
    
                    let scope_dependency: String = match &element.scope {
                        Some(value) => value.clone(),
                        None => String::from("")
                    };

                    let version_dependency: String = match &element.version {
                        Some(value) => value.clone(),
                        None => String::from(""),
                    };

                    let version_final = if version_dependency.is_empty() { 
                        "4.5.10".to_string() } else { version_dependency};
    
                    if opcional_dependency.is_empty() || opcional_dependency.eq("false") {
                        if scope_dependency.is_empty() || !scope_dependency.eq("test") {
                            toml_dependencies.dependencies.insert(
                                format!("{}:{}", 
                                group_id, artifact),
                                version_final);
      
                        }
                    }
                }
            } else {
                print!("No data to iterate over.")
            }
        }

        Ok(toml_dependencies)

    }

    pub fn get_init_pom(&self, file_path: &str) -> Vec<DependencyDetail> {
        self.managment.read_toml_file(file_path).unwrap().values_to_vec()
    }
}