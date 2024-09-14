use std::path::Path;
use reqwest;

pub trait DependencySearch {
    fn download_dependencies(url: &str, path: &Path) -> Result<(), reqwest::Error>;
}