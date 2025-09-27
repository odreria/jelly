use std::collections::HashMap;

use regex::Regex;

use crate::errors::{JellyError, Result};

pub fn get_raw_version (
    version: &str,
    properties: &HashMap<String, String>,
    project_version: Option<String>,
) -> Result<String> {

    let version_regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let semver_regex = Regex::new(r"^\d+(\.\d+){0,2}(\.[a-zA-Z0-9\-.]*)?$").unwrap(); 

    let mut raw_version = version.to_string();

    while let Some(caps) = version_regex.captures(&raw_version) {  
        if let Some(match_version) = caps.get(1) {
            let placeholder = match_version.as_str();
            if placeholder == "project.version" {
                raw_version = project_version.clone().ok_or_else(||
                    JellyError::missing_configuration("project version value is None. It must be sent as param.")
                )?;
            } else {
                raw_version = properties.get(placeholder).ok_or_else(||
                    JellyError::missing_configuration("Property not found")
                )?.clone();
            }
        }
    }

    if semver_regex.is_match(&raw_version) {
        return Ok(raw_version);
    }
    println!("{:?}", raw_version);

    Err(JellyError::validation("Final version is not a valid semver."))
}
