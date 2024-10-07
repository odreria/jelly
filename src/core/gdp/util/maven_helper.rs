use std::collections::HashMap;

use regex::Regex;

use crate::errors::beetle_error::BeetleError;


pub fn get_url_maven_format(group_id: &str, artifact_id: &str, version: &str, extension: &str) -> String {
    let file_name = format!("{}-{}.{}", artifact_id, version, extension);
    format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}",
        group_id,
        artifact_id,
        version,
        file_name
    )
}

pub fn get_raw_version (
    version: &str,
    properties: &HashMap<String, String>,
    project_version: Option<String>,
) -> Result<String, BeetleError> {

    let version_regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let semver_regex = Regex::new(r"^\d+(\.\d+){0,2}(-[a-zA-Z0-9\-.]*)?$").unwrap(); 

    let mut raw_version = String::new();

    if let Some(caps) = version_regex.captures(&version) {  
        if let Some(match_version) = caps.get(1) {
            raw_version = match_version.as_str().to_string();
        }
    }

    if raw_version.is_empty() {
       raw_version = version.to_string();
    }

    if semver_regex.is_match(&raw_version) {
        return Ok(raw_version);
    }

    match raw_version.as_str() {
        "project.version" => {
            let raw_project_version =
                project_version.ok_or(
                    BeetleError::MissingValue(
                        "project version value is None. It must be sent as param.".to_string()))?;

            Ok(raw_project_version)
        }
        _ => {
            let tmp_version =
                properties
                .get(&raw_version)
                .ok_or(
                    BeetleError::MissingValue("Property not found".to_string()))?;

            Ok(tmp_version.clone())
        }
    }
}
