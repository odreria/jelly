use std::collections::HashMap;

use crate::{
    adapters::pom::pom::TomlDependencies,
    core::gdp::{
        dependency::{pom_donwloader::PomDownloader, pom_managment::PomManagment},
        models::dependency::{
            Dependencies, Dependency, Project
        },
        util::maven_helper::get_raw_version,
    },
    errors::{JellyError, Result},
};

use crate::core::gdp::util::helper::extract_value;

pub struct PomService<R: PomManagment, D: PomDownloader> {    
    pub managment: R,
    pub dep_managment_version_map: HashMap<String, String>,
    pub downloader: D,
}

impl<R: PomManagment, D: PomDownloader> PomService<R, D> {
    pub fn new(managment: R, downloader: D) -> Self {
        PomService {
            managment,
            dep_managment_version_map: HashMap::new(),
            downloader,
        }
    }

    pub async fn get_pom_details(
        &mut self,
        dep: Dependency,
    ) -> Result<()> {
        let mut toml_dependencies: TomlDependencies = TomlDependencies::new();
        println!("checking {:?}", dep.to_string());
        let project = self.downloader.download_pom(&dep).await?;
        self.check_dependency_managment(&project);
        self.process_project_dependencies(&project, &mut toml_dependencies);
        Ok(())
    }

    async fn process_project_dependencies(
        &self,
        project: &Project,
        toml_dependencies: &mut TomlDependencies,
    ) -> Result<()> {
        
        let pom_dependency: Dependencies = match project.dependencies.clone() {
            Some(value) => value.clone(),
            None => Dependencies { dependencies: None },
        };

        if let Some(dep) = &pom_dependency.dependencies {
            for element in dep {
                let group_id: String =  element.group_id.clone().unwrap_or_default();

                let artifact: String = element.artifact_id.clone().unwrap_or_default();

                let opcional_dependency: String = element.optional.clone().unwrap_or_default();

                let scope_dependency: String = element.scope.clone().unwrap_or_default();

                let version_dependency: String = element.version.clone().unwrap_or_default();

                if opcional_dependency.is_empty() || opcional_dependency.eq("false") {
                    if scope_dependency.is_empty() || !scope_dependency.eq("test") {
                        let key = format!("{:}:{:}", group_id, artifact);
                        
                        let mut version_final = self.dep_managment_version_map.get(&key).cloned().unwrap_or(version_dependency);

                        if version_final.is_empty() {

                            let parent = extract_value(project.parent.clone(), "No tiene Parent")?;
                            let parent_version = extract_value(parent.version, "version no tiene valor")?;                            
                            version_final = parent_version.clone();
                        };

                        toml_dependencies
                            .dependencies
                            .insert(format!("{}:{}", group_id, artifact), version_final);
                    }
                }
            }
        } else {
            print!("No data to iterate over.")
        }

        Ok(())
    }

    ///
    /// Primero necesito verificar si para cada depencias el pom tiene un DependencyManagmanet.
    /// Si es asi, este debe asegurarse de ir a recuperar las versiones correctas para estas
    /// dependencias. Este va hasta los pom de tipo bom.
    /// 
    /// Si no tiene dependency managmanet el programa debe seguir su ejecucion tratando de encontrar las versiones
    /// para todas las otras dependencias.
    /// 
    async fn check_dependency_managment(&mut self, project: &Project) -> Result<()> {

        let dep_managment= project.dependencies_managment.clone().unwrap_or_default();

        if let Some(dpm) = &dep_managment.dependencies {

            let prop = project.properties.clone();

            let properties = prop
                .ok_or_else(||
                    JellyError::missing_configuration("Dependency Management has no properties")
                )?;

            let opt_dpm =
                self.get_pom_dependencies_managment(dpm, &properties)?;

            let dpm =
                extract_value(
                    opt_dpm,
                    "It was not possible to get Dependencies from Depedency Managment")?;

            
            for dep in dpm {
                self.get_version_from_dependency_managment(&dep);
            }
          
        }
        
        Ok(())
    }

    async fn get_version_from_dependency_managment(&mut self, dep: &Dependency) -> Result<()> {

        let project_dpm = self.downloader.download_pom(&dep).await?;

        let dependencies_dpm = project_dpm.dependencies_managment.ok_or_else(||
            JellyError::missing_configuration("Sub Dependency Management section not found from Root Dependency Management")
        )?;        

        let dependency_dpm = dependencies_dpm.dependencies.ok_or_else(||
            JellyError::missing_configuration("Sub Dependencies not found from Sub Dependency Management")
        )?;

        let properties_dpm = &mut project_dpm.properties.clone();

        if let Some(dpm_vec) = dependency_dpm.dependencies {
       
            for d in dpm_vec {
                match &d.type_dep {
                    Some(value) => {

                        if value.contains(&"pom".to_string()) {

                            let dep_bom =
                                self.get_pom_dependencies_from_bom(d, properties_dpm)?;

                            let project_sbom =
                                self.downloader.download_pom(&dep_bom).await?;

                            let depm_bom = project_sbom.dependencies_managment.unwrap_or_default();
                                                        
                            if let Some(properties_bom) = project_sbom.properties {
                                properties_dpm
                                    .get_or_insert_with(|| HashMap::new())
                                    .extend(properties_bom);
                            }

                            if let Some(_dep_bom) = depm_bom.dependencies {
                                for k in extract_value(_dep_bom.dependencies, "Without dependencies")? {
                                    self.populate_artifact_map(
                                        k,
                                        properties_dpm,
                                        project_sbom.version.clone(),
                                    )?;
                                }
                            }

                            //
                            // TODO: populate_artifact_map data should be reused.
                            // 1. It should run first to check all depes before start loading
                            // , if dep was registered before then skip it.
                            // 2. If dep version didnot find then user project.version.
                            //
                        }
                    }
                    None => self.populate_artifact_map(d, properties_dpm, None)?,
                }
            }
        }

        Ok(())
    }

    fn populate_artifact_map(
        &mut self,
        d: Dependency,
        properties: &Option<HashMap<String, String>>,
        project_version: Option<String>,
    ) -> Result<()> {
        let mut version = d.version.expect("msg");
        if let Some(prop) = properties {
            let raw_version = get_raw_version(&version, &prop, project_version)?;
            version = raw_version.clone();
        }

        let raw_group_id = d.group_id.expect("msg").clone();
        let raw_artifact_id = d.artifact_id.expect("msg");

        self.dep_managment_version_map.insert(
            format!("{}:{}", raw_group_id, raw_artifact_id),
            version.to_string(),
        );

        Ok(())
    }

    fn get_pom_dependencies_from_bom(
        &self,
        dep: Dependency,
        properties: &Option<HashMap<String, String>>,
    ) -> Result<Dependency> {
        let mut version = dep.version.unwrap_or_default();

        if let Some(prop) = properties {
            let raw_version = get_raw_version(&version, &prop, None)?;
            version = raw_version.clone();
        }

        let dep_bom = Dependency::new(dep.group_id, dep.artifact_id, Some(version));

        Ok(dep_bom)
    }

    ///
    /// Gets the URL's POM for <DependencyManagment> section.
    ///
    /// # Arguments
    ///
    /// * `dpm` - The Dependencies struct that contains dependency information.
    /// * `properties` - Properties Map that may be used for getting the raw version of
    /// DepedencyManagment information.
    ///
    /// # Returns
    ///
    /// Result`<Option<Vec<Dependency>>, BettleError>`  where each `String` is a URL's POM or BettleError message.
    fn get_pom_dependencies_managment(
        &self,
        dpm: &Dependencies,
        properties: &HashMap<String, String>,
    ) -> Result<Option<Vec<Dependency>>> {
        
        let mut opt_dependencies: Option<Vec<Dependency>> = Some(Vec::new());

        for dep in extract_value(dpm.dependencies.clone(), "")? {

            let version = dep
                    .version
                    .ok_or_else(|| JellyError::missing_configuration("Version not found"))?;

            let raw_version = get_raw_version(&version, properties, None)?;

                let raw_group_id = dep
                    .group_id
                    .ok_or_else(|| JellyError::missing_configuration("group_id not found"))?;

                let artifact_id = dep
                    .artifact_id
                    .ok_or_else(|| JellyError::missing_configuration("artifact_id not found"))?;
                
                let new_dep = Dependency::new(Some(raw_group_id), Some(artifact_id), Some(raw_version));

            if let Some(dependency) = &mut opt_dependencies {
                dependency.push(new_dep);
            }
        }

        Ok(opt_dependencies)
    }

    pub fn get_init_pom(&self, file_path: &str) -> Result<TomlDependencies> {
        self.managment.read_toml_file(file_path)
    }

}
