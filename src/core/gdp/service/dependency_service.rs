
use crate::{core::gdp::dependency::{pom_donwloader::PomDownloader, pom_managment::PomManagment}, errors::{JellyError, Result}};

use super::pom_service::PomService;

pub struct DependencyService<V: PomManagment, D: PomDownloader> {
    pub pom_service: PomService<V, D>,
}

impl<V: PomManagment, D: PomDownloader> DependencyService<V, D> {
    pub fn new(pom_service: PomService<V, D>) -> Self {
        DependencyService {
            pom_service,
        }
    }

    pub async fn start(&mut self) -> Result<()> {

        let dependencies_from_toml =
            self.pom_service.get_init_pom("jelly.toml");

        let dependencies = dependencies_from_toml?.values_to_vec();

        for dep_toml in dependencies {

            self
            .pom_service
            .get_pom_details(dep_toml)
            .await
                .map_err(|e|
                    JellyError::repository(format!("Error downloading pom.xml details {}", e))
                )?;
        }


        Ok(())

    }

}