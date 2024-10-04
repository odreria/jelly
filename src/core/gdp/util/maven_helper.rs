use std::collections::HashMap;

use regex::Regex;


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
    version: &String,
    properties: HashMap<String, String>) -> String {

    let stack_version_regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let caps = stack_version_regex.captures(&version).unwrap();        
    let raw_version = caps.get(1).map_or("", |m| m.as_str());
    
    let raw_version = properties.get(raw_version).expect("msg");

    raw_version.clone()
}
