use crate::{adapters::pom::pom::{DependencyDetail, TomlDependencies}, core::gdp::{dependency::pom_managment::PomManagment, models::dependency::Dependencies}};
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
    
                    if opcional_dependency.is_empty() || opcional_dependency.eq("false") {
                        if scope_dependency.is_empty() || !scope_dependency.eq("test") {
                            toml_dependencies.dependencies.insert(format!("{}:{}", group_id, artifact), "4.5.10".to_string());
      
                        }
                    }
                }
            } else {
                print!("No data to iterate over.")
            }
        }

        Ok(toml_dependencies)

    }

    pub fn collect_all_pom(&self, file_path: &str) -> Vec<DependencyDetail> {
        self.managment.read_toml_file(file_path).unwrap().values_to_vec()
    }
}