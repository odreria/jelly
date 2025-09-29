use crate::cli::output::JellyOutput;
use crate::errors::jelly_error::JellyError;

pub async fn execute() -> Result<(), JellyError> {
    JellyOutput::header("Installing dependencies")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    // TODO: Read jelly.toml
    // TODO: Resolve dependencies
    // TODO: Download JAR files to libs/
    // TODO: Create classpath

    JellyOutput::step("Reading jelly.toml...")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::step("Resolving dependency graph...")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::step("Downloading artifacts...")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::warning("Command not yet implemented")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    JellyOutput::info("This will be implemented in Phase 2")
        .map_err(|e| JellyError::OutputError(e.to_string()))?;

    Ok(())
}