use crate::core::gdp::search::DependencySearch;

pub struct BreadFirstSearch;

impl DependencySearch for BreadFirstSearch {
    fn download_dependencies(url: &str, path: &Path) -> Result<(), reqwest::Error> {
        let response = get(url).await?;
        let mut file = File::create(path).expect("error al crear el archivo");
        copy(&mut response.bytes().await?.as_ref(), &mut file).expect("No logro copiar el archivo.");
        Ok(())
    }
}