use std::collections::VecDeque;
use crate::{adapters::pom::pom::DependencyDetail, core::gdp::dependency::dependency_search::DependencySearch};

#[derive(Hash, Eq, PartialEq, Debug)]
pub struct Discovers {
    name: String,
    color: String
}

pub struct BreadFirstSearch{
    pub queue: VecDeque<DependencyDetail>,
    pub discovers: Discovers,
}

impl DependencySearch for BreadFirstSearch {

    fn enqueue(&mut self, dependency_details: &Vec<DependencyDetail>) {
        for dependency in dependency_details.iter() {
            self.queue.push_back(dependency.clone());
        }
     }

     fn dequeue(&mut self) -> Option<DependencyDetail> {
        self.queue.pop_front()
     }

}