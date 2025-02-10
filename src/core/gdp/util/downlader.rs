
use reqwest::get;
use std::io::copy;
use std::fs::File;
use std::path::Path;


pub async fn download_dependencies(url: &str, path: &Path) -> Result<(), reqwest::Error> {
    let response = get(url).await?;
    let mut file = File::create(path).expect("");
    copy(&mut response.bytes().await?.as_ref(), &mut file).expect("Dependency cannot be copied.");
    Ok(())
}
