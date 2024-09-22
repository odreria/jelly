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

    pub fn start(&self) {
        // Start coding the BFS algoritm.
        self.search.enqueue(dependency_details);
    }

}