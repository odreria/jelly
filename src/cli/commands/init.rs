use crate::cli::output::JellyOutput;
use crate::errors::jelly_error::JellyError;
use std::path::Path;

pub async fn execute(name: Option<String>, package: Option<String>) -> Result<(), JellyError> {
    let project_name = name.unwrap_or_else(|| {
        std::env::current_dir()
            .ok()
            .and_then(|path| path.file_name().map(|n| n.to_string_lossy().to_string()))
            .unwrap_or_else(|| "my-jdk-app".to_string())
    });
    
    let mut package_name = package.unwrap_or_else(|| "com.myjdkapp".to_string());
    package_name.push_str(".Main");
    
    JellyOutput::header(&format!("setting package name: {}", package_name));

    JellyOutput::header(&format!("Initializing Jelly project: {}", project_name))
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    // Check if jelly.toml already exists
    if Path::new("jelly.toml").exists() {
        JellyOutput::error("jelly.toml already exists in this directory")
            .map_err(|e| JellyError::OutputError(e.to_string()))?;
        return Err(JellyError::ProjectInitError(
            "Project already initialized".to_string()
        ));
    }

    // Create basic jelly.toml template
    JellyOutput::step("Creating jelly.toml configuration...")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    let toml_content = create_jelly_toml_template(&project_name, &package_name);

    std::fs::write("jelly.toml", toml_content)
        .map_err(|e| JellyError::FileSystemError(format!("Failed to create jelly.toml: {}", e)))?;

    JellyOutput::success("Created jelly.toml")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    // Create libs directory for dependencies
    JellyOutput::step("Creating libs/ directory for dependencies...")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    std::fs::create_dir_all("libs")
        .map_err(|e| JellyError::FileSystemError(format!("Failed to create libs directory: {}", e)))?;

    JellyOutput::success("Created libs/ directory")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::newline()
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::success("Project initialized successfully!")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::newline()
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::info("Next steps:")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::plain("  jelly add <dependency>    Add dependencies")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::plain("  jelly install             Download all dependencies")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    Ok(())
}

fn create_jelly_toml_template(name: &str, package_name: &str) -> String {
    format!(r#"[package]
name = "{}"
version = "1.0.0"
main-class = "{}"

[dependencies]
# Add your dependencies here
# Example: "org.springframework:spring-core" = "6.0.0"

[dev-dependencies]
# Development dependencies
# Example: "junit:junit" = "4.13.2"

[scripts]
# Custom scripts
start = "java -cp 'libs/*' ${{main-class}}"
test = "java -cp 'libs/*:.' org.junit.runner.JUnitCore"
"#, name, package_name)
}