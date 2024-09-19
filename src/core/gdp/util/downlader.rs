
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

pub async fn download_pom(url: &str) -> Result<String, reqwest::Error> {
    let content_req = get(url).await?;
    let content = content_req.text().await?;
    Ok(content)
}