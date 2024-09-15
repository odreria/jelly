use jelly::core::gdp::models::dependency::*;
use quick_xml::de::from_str;
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use toml::de::Error as TomlError;
use tokio;

#[derive(Debug, Deserialize)]
struct DependencyDetail {
    file_name: String,
    url_jar: String,
    url_pom: String,
}

#[derive(Debug, Deserialize)]
struct TomlDependencies {
    dependencies: HashMap<String, String>,
}

impl TomlDependencies {
    fn new() -> Self {
        let new_map = HashMap::new();
        TomlDependencies { dependencies: new_map}
    }

    fn values_to_vec(&self) -> Vec<DependencyDetail> {
        let mut vec: Vec<DependencyDetail> = Vec::new();

        for (artifact, version) in &self.dependencies {
            vec.push(Self::parse_dependency(&artifact, &version));
        }

        vec
    }

    fn parse_dependency(artifact: &str, version: &str) -> DependencyDetail {
        let parts: Vec<&str> = artifact.split(':').collect();
        let group_id = parts[0].replace('.', "/");
        let artifact_id = parts[1];
        let file_name = format!("{}-{}.jar", artifact_id, version);
        let url_jar = get_url_maven_format(&group_id, &artifact_id, &version, "jar");
        let url_pom = get_url_maven_format(&group_id, &artifact_id, &version, "pom");

        DependencyDetail {
            file_name,
            url_jar,
            url_pom,
        }
    }
}

fn read_toml_file(file_path: &str) -> Result<TomlDependencies, TomlError> {
    let content = fs::read_to_string(file_path).expect("No se logro leer el archivo");
    let dependencies: TomlDependencies = toml::de::from_str(&content)?;
    Ok(dependencies)
}

fn get_url_maven_format(group_id: &str, artifact_id: &str, version: &str, extension: &str) -> String {
    let file_name = format!("{}-{}.{}", artifact_id, version, extension);
    format!(
        "https://repo1.maven.org/maven2/{}/{}/{}/{}",
        group_id,
        artifact_id,
        version,
        file_name
    )
}

fn parse_pom(xml: &str) -> Project {
    from_str(xml).unwrap()
}

#[tokio::main]
async fn main() {
    let dependencies: TomlDependencies = read_toml_file("jelly.toml").unwrap();

    let dependency_details: Vec<DependencyDetail> = dependencies.values_to_vec();

    for detail in &dependency_details {
        println!("File: {} ", detail.file_name);
        println!("POM: {}", detail.url_pom);

        let content_req = get(&detail.url_pom).await.expect("error al consultar el pom");
        let pom_content = content_req.text().await.expect("error al obtener el texto");
        //let pom_content = download_pom(&detail.url_pom);

        let project_xml = parse_pom(&pom_content);

        let pom_dependencies: &Dependencies = match &project_xml.dependencies {
            Some(value) => value,
            None => &Dependencies {
                dependencies: None
            }
        };

        let mut mikoki : TomlDependencies = TomlDependencies::new();
        
        if let Some(dep) = &pom_dependencies.dependencies {
            for element in dep {
                let group_id: String = match &element.group_id {
                    Some(value) => value.clone(),
                    None => String::from("")
                };
    
                let artifact: String = match &element.artifact_id {
                    Some(value) => value.clone(),
                    None => String::from("")
                };

                let opcional_dependency: String = match &element.optional {
                    Some(value) => value.clone(),
                    None => String::from("")
                };

                let scope_dependency: String = match &element.scope {
                    Some(value) => value.clone(),
                    None => String::from("")
                };

                if opcional_dependency.is_empty() || opcional_dependency.eq("false") {
                    if scope_dependency.is_empty() || !scope_dependency.eq("test") {
                        mikoki.dependencies.insert(format!("{}:{}", group_id, artifact), "4.5.10".to_string());
  
                    }
                }
            }
        } else {
            print!("No data to iterate over.")
        }


        // Investigar en la busqueda en profundids bfs
        print!("{:?}", mikoki.values_to_vec());
        
       // for (artifact, version) in verified_pom_dependencies.dependencies {
          //  dependency_details.push(parse_dependency(&artifact, &version));
      //  }

    }
/*

    let temp_path = &format!("tmp/artifacts/{}", file_name);

    let path = Path::new(&temp_path);

    if let Err(e) = download_dependencies(&url_jar, &path).await {
        eprintln!("Failed to download {}: {}", url_jar, e);
    }
    */
}
