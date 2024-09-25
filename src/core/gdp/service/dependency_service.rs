use crate::core::gdp::dependency::{dependency_search::DependencySearch, pom_managment::PomManagment};

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

    pub async fn start(&mut self) {
        let mut end: bool = false;

        let dependencies_from_toml = self.pom_service.get_init_pom("jelly.toml");

        let toml = match self.pom_service.get_pom_details(&dependencies_from_toml).await {
            Ok(pomxml) => pomxml,
            Err(_) => panic!("Error downloading pom.xml details"),
        };

        self.search.enqueue(&toml.values_to_vec());

        while end == false {
            let u = self.search.dequeue();
            // add instruction to download the jar file
            if let Some(dep) = u {
                // find the pom dependencies for dep and add the to queue
                // mark the dependency with gray color
            }

        }

    }

}