use jelly::core::gdp::models::dependency::*;
use quick_xml::de::from_str;
use reqwest::get;
use serde::Deserialize;
use std::collections::HashMap;
use std::fs;
use std::fs::File;
use std::io::copy;
use std::path::Path;
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

async fn download_dependencies(url: &str, path: &Path) -> Result<(), reqwest::Error> {
    let response = get(url).await?;
    let mut file = File::create(path).expect("error al crear el archivo");
    copy(&mut response.bytes().await?.as_ref(), &mut file).expect("No logro copiar el archivo.");
    Ok(())
}

async fn download_pom(url: &str) -> Result<String, reqwest::Error> {
    let content_req = get(url).await?;
    let content = content_req.text().await?;
    Ok(content)
}

#[tokio::main]
async fn main() {
    let dependencies: TomlDependencies = read_toml_file("grocket.toml").unwrap();

    let mut dependency_details: Vec<DependencyDetail> = Vec::new();

    for (artifact, version) in dependencies.dependencies {
        dependency_details.push(parse_dependency(&artifact, &version));
    }

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

        let mut verified_pom_dependencies: Vec<TomlDependencies> = vec![];
        
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

                        let mut dependenecy_reviews = HashMap::new();
                        dependenecy_reviews.insert(format!("{}:{}", group_id, artifact), "4.5.10".to_string());

                        verified_pom_dependencies.push(TomlDependencies {
                            dependencies: dependenecy_reviews,
                        });  
                    }
                }
            }
        } else {
            print!("No data to iterate over.")
        }


        let map = verified_pom_dependencies.iter().map(| d | {
            
             let mut verified_dependency_details: Vec<DependencyDetail> = Vec::new();
            for (artifact, version) in &d.dependencies {
                verified_dependency_details.push(parse_dependency(&artifact, &version));
            }
            verified_dependency_details
            
        });

        // Investigar en la busqueda en profundids bfs
        print!("{:?}", map);
        
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
