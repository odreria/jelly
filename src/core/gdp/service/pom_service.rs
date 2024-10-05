use std::collections::HashMap;

use crate::{
    adapters::pom::pom::{DependencyDetail, TomlDependencies},
    core::gdp::{
        dependency::pom_managment::PomManagment,
        models::dependency::{
            Dependencies, DependenciesManagment, Dependency, DependencyManagment, Project,
        },
        util::maven_helper::{get_raw_version, get_url_maven_format},
    }, errors::beetle_error::BeetleError,
};
use reqwest::{get, Response};
pub struct PomService<R: PomManagment> {
    pub managment: R,
    pub artifact_map: HashMap<String, String>,
}

impl<R: PomManagment> PomService<R> {
    pub fn new(managment: R) -> Self {
        let artifact_map: HashMap<String, String> = HashMap::new();
        PomService {
            managment,
            artifact_map,
        }
    }

    pub async fn get_pom_details(
        &mut self,
        dependency_details: &Vec<DependencyDetail>,
    ) -> Result<TomlDependencies, BeetleError> {
        let mut toml_dependencies: TomlDependencies = TomlDependencies::new();

        for detail in dependency_details {
            let project_xml =
                self.get_project_pom(&detail.url_pom).await.map_err(BeetleError::from)?;

            let dep_managment = match project_xml.dependency_managment {
                Some(value) => value,
                None => DependencyManagment { dependencies: None },
            };

            if let Some(dpm) = dep_managment.dependencies {
                let properties =
                    project_xml
                    .properties
                    .ok_or_else(
                        || BeetleError::MissingValue("Properties not found".to_string()))?;

                let url_dpm =
                    self.get_pom_dependencies_managment(dpm, &properties)?;

                let project_dpm =
                    self
                    .get_project_pom(&url_dpm[0])
                    .await
                    .map_err(BeetleError::from)?;

                let dependencies_dpm =
                    project_dpm
                    .dependencies
                    .ok_or_else(
                     || BeetleError::MissingValue(
                            "Dependecies section not found from DependencyManagment".to_string()))?;
                
                let dependency_dpm_vec =
                    dependencies_dpm
                    .dependencies
                    .ok_or_else(
                        || BeetleError::MissingValue(
                                "Dependency not found from DependencyManagment".to_string()))?;

                // It should return a Ma<String, String> With key = full dependency name concatenated
                // and value = raw_Version
                for d in dependency_dpm_vec {
                    match &d.type_dep {
                        Some(value) => {
                            if "pom".eq(value) {
                                let url_sbom = self.get_pom_dependencies_from_sbom(d, &properties);
                                let project_sbom = self.get_project_pom(&url_sbom).await;

                                let dependencies_sbom = match project_sbom.dependencies {
                                    Some(value) =>  value.dependencies,
                                    None => None,
                                };

                                let dep_sbom = dependencies_sbom.expect("msg");
                                println!("{:?}", dep_sbom);
                                
                                // we need to go to "library"-SBOM to get the correct version.
                                // get all dependencies and version with stack.version or filter them.
                                
                            } else {
                                self.populate_artifact_map(d, &properties)
                            }
                        }
                        None => self.populate_artifact_map(d, &properties),
                    }
                }
            }

            let pom_dependency: Dependencies = match project_xml.dependencies {
                Some(value) => value.clone(),
                None => Dependencies { dependencies: None },
            };

            if let Some(dep) = &pom_dependency.dependencies {
                for element in dep {
                    let group_id: String = match &element.group_id {
                        Some(value) => value.clone(),
                        None => String::from(""),
                    };

                    let artifact: String = match &element.artifact_id {
                        Some(value) => value.clone(),
                        None => String::from(""),
                    };

                    let opcional_dependency: String = match &element.optional {
                        Some(value) => value.clone(),
                        None => String::from(""),
                    };

                    let scope_dependency: String = match &element.scope {
                        Some(value) => value.clone(),
                        None => String::from(""),
                    };

                    let version_dependency: String = match &element.version {
                        Some(value) => value.clone(),
                        None => String::from(""),
                    };

                    let version_final = if version_dependency.is_empty() {
                        "4.5.10".to_string()
                    } else {
                        version_dependency
                    };

                    if opcional_dependency.is_empty() || opcional_dependency.eq("false") {
                        if scope_dependency.is_empty() || !scope_dependency.eq("test") {
                            toml_dependencies
                                .dependencies
                                .insert(format!("{}:{}", group_id, artifact), version_final);
                        }
                    }
                }
            } else {
                print!("No data to iterate over.")
            }
        }

        Ok(toml_dependencies)
    }

    fn populate_artifact_map(
        &mut self,
        d: Dependency,
        properties: &HashMap<String, String>
    ) {
        let version = &d.version.expect("msg");
        let raw_version = get_raw_version(version, properties, None);
        let raw_group_id = d.group_id.expect("msg").clone();
        let raw_artifact_id = d.artifact_id.expect("msg");

        println!("{:}:{:}", raw_artifact_id, raw_version);

        self.artifact_map
            .insert(format!("{}:{}", raw_group_id, raw_artifact_id), raw_version);

    }

    fn get_pom_dependencies_from_sbom(
        &self,
        d: Dependency,
        properties: &HashMap<String, String>
    ) -> String {

        let version = d.version.expect("msg");
        let raw_version = get_raw_version(&version, properties, None);

        let group_id = d.group_id.expect("msg").clone();
        let raw_group_id = group_id.replace(".", "/");
        let artifact_id = d.artifact_id.expect("msg").clone();

        get_url_maven_format(&raw_group_id, &artifact_id, &raw_version, "pom")
        
    }

    /// 
    /// Gets the URL's POM for <DependencyManagment> section.
    /// 
    /// # Arguments 
    /// 
    /// * `dpm` - The DependenciesManagment struct that contains dependency information.
    /// * `properties` - Properties Map that may be used for getting the raw version of
    /// DepedencyManagment information.
    /// 
    /// # Returns
    /// 
    /// Result`<Vec<String, BettleError>>`  where each `String` is a URL's POM or BettleError message. 
    fn get_pom_dependencies_managment(
        &self,
        dpm: DependenciesManagment,
        properties: &HashMap<String, String>,
    ) -> Result<Vec<String>, BeetleError> {
        let dependencies =
            dpm.dependency
            .ok_or_else(
                || BeetleError::MissingValue("No dependencies found".to_string()))?;

        let mut urls = Vec::new();

        for dependency_managment in dependencies {
            let version =
                dependency_managment
                .version
                .ok_or_else(
                    || BeetleError::MissingValue("Version not found".to_string()))?;

            let raw_version =
                get_raw_version(
                    &version, properties,
                    None);
        
            let raw_group_id =
                dependency_managment
                .group_id
                .ok_or_else(
                    || BeetleError::MissingValue("group_id not found".to_string()))?;

            let group_id = raw_group_id.replace(".", "/");

            let artifact_id =
                dependency_managment
                .artifact_id
                .ok_or_else(
                    || BeetleError::MissingValue("artifact_id not found".to_string()))?;

            let url =
                get_url_maven_format(
                    &group_id,
                    &artifact_id,
                    &raw_version,
                    "pom");

            urls.push(url);
        }

        Ok(urls)

    }

    pub fn get_init_pom(&self, file_path: &str) -> Vec<DependencyDetail> {
        self.managment
            .read_toml_file(file_path)
            .unwrap()
            .values_to_vec()
    }

    pub async fn get_project_pom(&self, url_pom: &String) -> Result<Project, BeetleError> {
        let content_req: Response =
            get(url_pom).await.map_err(BeetleError::from)?;

        let pom_content: String =
            content_req.text().await.map_err(BeetleError::from)?;

        let project_pom: Project = self.managment.parse_pom(&pom_content);

        Ok(project_pom)
    }
}
