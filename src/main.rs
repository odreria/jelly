use jelly::adapters::pom::pom::TomlDependencies;
use tokio;

#[tokio::main]
async fn main() {
    let dependencies: TomlDependencies = read_toml_file("jelly.toml").unwrap();

        
       // for (artifact, version) in verified_pom_dependencies.dependencies {
          //  dependency_details.push(parse_dependency(&artifact, &version));
      //  }

    
/*

    let temp_path = &format!("tmp/artifacts/{}", file_name);

    let path = Path::new(&temp_path);

    if let Err(e) = download_dependencies(&url_jar, &path).await {
        eprintln!("Failed to download {}: {}", url_jar, e);
    }
    */
}
