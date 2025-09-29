use clap::{Parser, Subcommand};
use crate::errors::jelly_error::JellyError;

#[derive(Parser)]
#[command(name = "jelly")]
#[command(about = "A simplified package manager for Java and Groovy applications")]
#[command(version = "0.1.0")]
pub struct JellyApp {
    #[command(subcommand)]
    pub command: JellyCommand,
}

#[derive(Subcommand)]
pub enum JellyCommand {
    #[command(about = "Initialize a new Java/Groovy project with jelly.toml")]
    Init {
        #[arg(help = "Project name")]
        name: Option<String>,
        
        #[arg(help = "Package name (e.g., com.jdkapp)")]
        package: Option<String>,
    },
    #[command(about = "Add a dependency to the project")]
    Add {
        #[arg(help = "Dependency to add (e.g., 'org.springframework:spring-core:6.0.0')")]
        dependency: String,
    },
    #[command(about = "Download and install all dependencies")]
    Install,
    #[command(about = "Execute project or custom script")]
    Run {
        #[arg(help = "Script name to run (optional)")]
        script: Option<String>,
    },
}

impl JellyApp {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub async fn execute(self) -> Result<(), JellyError> {
        match self.command {
            JellyCommand::Init { name, package } => {
                crate::cli::commands::init::execute(name, package).await
            }
            JellyCommand::Add { dependency } => {
                crate::cli::commands::add::execute(dependency).await
            }
            JellyCommand::Install => {
                crate::cli::commands::install::execute().await
            }
            JellyCommand::Run { script } => {
                crate::cli::commands::run::execute(script).await
            }
        }
    }
}