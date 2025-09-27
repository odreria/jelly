use reqwest::get;

use crate::{core::gdp::{models::dependency::{Dependency, Project}, util::helper::extract_value}, errors::{JellyError, Result}};

const ERROR_MESSAGE_GROUP_ID: &str = "groupd id must be available for downloading the POM file.";
const ERROR_MESSAGE_ARTIFACT_ID: &str =
    "artifact id must be available for downloading the POM file.";
const ERROR_MESSAGE_VERSION: &str = "version must be available for downloading the POM file.";

pub trait PomDownloader {
    async fn download_pom(&self, dep: &Dependency) -> Result<Project>;
}

pub struct MavenPomDownloader;

impl PomDownloader for MavenPomDownloader {

    async fn download_pom(&self, dep: &Dependency) -> Result<Project> {
        let base_url = "https://repo1.maven.org/maven2";
        let opt_group_id = dep.group_id.clone();
        let opt_artifact_id = dep.artifact_id.clone();
        let opt_version = dep.version.clone();

        let group_path = extract_value(opt_group_id, ERROR_MESSAGE_GROUP_ID)?.replace(".", "/");

        let artifact_id = extract_value(opt_artifact_id, ERROR_MESSAGE_ARTIFACT_ID)?;

        let version = extract_value(opt_version, ERROR_MESSAGE_VERSION)?;

        let url = format!(
            "{}/{}/{}/{}/{}-{}.pom",
            base_url, group_path, artifact_id, version, artifact_id, version,
        );

        let content_req = get(url).await?;
        let content = content_req.text().await?;

        let project: Project = todo!(); //self.managment.parse_pom(&content);

        Ok(project)
    }
}