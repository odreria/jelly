use crate::adapters::pom::pom::DependencyDetail;


pub trait DependencySearch {
    fn enqueue(&mut self, dependency_details: &Vec<DependencyDetail>);
}