use crate::{adapters::pom::pom::DependencyDetail, core::gdp::dependency::pom_managment::PomManagment};

pub struct PomService<R: PomManagment> {
    pub managment: R,
 }

impl<R: PomManagment> PomService<R>
{
    pub fn new(managment: R) -> Self {
         PomService { managment }
    }

    pub fn read_toml_file(&self, file_path: &str) -> Vec<DependencyDetail> {
        self.managment.read_toml_file(file_path).unwrap().values_to_vec()

    }

}