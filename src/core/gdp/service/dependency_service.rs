use crate::{core::gdp::dependency::{dependency_search::DependencySearch, pom_managment::PomManagment}, errors::beetle_error::BeetleError};
use super::pom_service::PomService;

pub struct DependencyService<T: DependencySearch, V: PomManagment> {
    pub search: T,
    pub pom_service: PomService<V>,
}

impl<T: DependencySearch, V: PomManagment> DependencyService<T, V> {
    pub fn new(search: T, pom_service: PomService<V>) -> Self {
        DependencyService {
            search,
            pom_service,
        }
    }

    pub async fn start(&mut self) -> Result<(), BeetleError> {
        let mut end: bool = false;
        let dependencies_from_toml =
            self.pom_service.get_init_pom("jelly.toml");

        let toml =
            self
            .pom_service
            .get_pom_details(&dependencies_from_toml)
            .await
            .map_err(
                |e| {
                    BeetleError::MissingValue(
                        format!("Error downloading pom.xml details {}", e)
                    )
                }
            )?;

        self.search.enqueue(&toml.values_to_vec());

        while end == false {
            let u = self.search.dequeue();
            if let Some(dep) = u {
                println!("Downloading {}", dep.file_name);
                // add instruction to download the jar file

                let mut vec_dep = Vec::new();
                vec_dep.push(dep);

               let internal_toml =  self.pom_service.get_pom_details(&vec_dep).await.map_err(BeetleError::from)?;
                
                if internal_toml.dependencies.len() != 0 {
                    self.search.enqueue(&internal_toml.values_to_vec());
                } else {
                    end = true;
                }

            }

        }

        Ok(())

    }

}