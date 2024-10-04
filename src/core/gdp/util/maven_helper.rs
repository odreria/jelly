use std::collections::HashMap;

use regex::Regex;

const EMPTY: &str = "";


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
    properties: HashMap<String, String>,
    stack_version: Option<String>
) -> String {

    let version_regex = Regex::new(r"\$\{([^}]+)\}").unwrap();
    let caps = version_regex.captures(&version).unwrap();        
    let raw_version = caps.get(1).map_or_else( || EMPTY, |m| m.as_str());
    
    let result: String;
    if raw_version.eq(EMPTY) {
        result = raw_version.to_owned();
    } else {
        if let Some(v) = stack_version {
            result = v;
        } else {
            result = properties.get(raw_version).expect("msg").to_string();   
        }
    }

    result.clone()
}
