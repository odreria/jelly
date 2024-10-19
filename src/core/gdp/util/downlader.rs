
use reqwest::get;
use std::io::copy;
use std::fs::File;
use std::path::Path;

use crate::core::gdp::models::dependency::Dependency;
use crate::errors::beetle_error::BeetleError;
use crate::core::gdp::util::helper::extract_value;

const ERROR_MESSAGE_GROUP_ID: &str = "groupd id must be available for downloading the POM file.";
const ERROR_MESSAGE_ARTIFACT_ID: &str = "artifact id must be available for downloading the POM file.";
const ERROR_MESSAGE_VERSION: &str = "version must be available for downloading the POM file.";

pub async fn download_dependencies(url: &str, path: &Path) -> Result<(), reqwest::Error> {
    let response = get(url).await?;
    let mut file = File::create(path).expect("");
    copy(&mut response.bytes().await?.as_ref(), &mut file).expect("Dependency cannot be copied.");
    Ok(())
}

pub async fn download_pom(dep: &Dependency) -> Result<String, BeetleError> {
    let base_url = "https://repo1.maven.org/maven2";
    let opt_group_id =  dep.group_id.clone();
    let opt_artifact_id = dep.artifact_id.clone();
    let opt_version = dep.version.clone();

    let group_path =
        extract_value(opt_group_id, ERROR_MESSAGE_GROUP_ID)?
        .replace(".", "/");

    let artifact_id = extract_value(opt_artifact_id, ERROR_MESSAGE_ARTIFACT_ID)?;

    let version= extract_value(opt_version, ERROR_MESSAGE_VERSION)?;
    
    let url = format!(
        "{}/{}/{}/{}/{}-{}.pom",
        base_url,
        group_path,
        artifact_id,
        version,
        artifact_id,
        version,
        );

    let content_req = get(url).await?;
    let content = content_req.text().await?;
    Ok(content)
}