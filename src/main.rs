use jelly::{adapters::pom::pom::{DependencyDetail, TomlDependencies}, core::gdp::models::dependency::*};
use tokio;

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
