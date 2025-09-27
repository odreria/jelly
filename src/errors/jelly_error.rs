use thiserror::Error;

#[derive(Error, Debug)]
pub enum JellyError {
    #[error("Network error: {0}")]
    Network(#[from] reqwest::Error),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Configuration parsing error: {0}")]
    ConfigParsing(#[from] toml::de::Error),

    #[error("XML parsing error: {0}")]
    XmlParsing(#[from] quick_xml::DeError),

    #[error("Dependency resolution failed: {message}")]
    DependencyResolution { message: String },

    #[error("Dependency not found: {dependency}")]
    DependencyNotFound { dependency: String },

    #[error("Version conflict for {dependency}: requested {requested}, but {conflicting} is already resolved")]
    VersionConflict {
        dependency: String,
        requested: String,
        conflicting: String,
    },

    #[error("Invalid dependency specification: {spec} - {reason}")]
    InvalidDependencySpec { spec: String, reason: String },

    #[error("Missing required configuration: {field}")]
    MissingConfiguration { field: String },

    #[error("Repository error: {message}")]
    Repository { message: String },

    #[error("Project initialization failed: {reason}")]
    ProjectInit { reason: String },

    #[error("Invalid project structure: {issue}")]
    InvalidProject { issue: String },

    #[error("Validation error: {message}")]
    Validation { message: String },
}

impl JellyError {
    pub fn dependency_resolution<S: Into<String>>(message: S) -> Self {
        Self::DependencyResolution {
            message: message.into(),
        }
    }

    pub fn dependency_not_found<S: Into<String>>(dependency: S) -> Self {
        Self::DependencyNotFound {
            dependency: dependency.into(),
        }
    }

    pub fn version_conflict<S: Into<String>>(
        dependency: S,
        requested: S,
        conflicting: S,
    ) -> Self {
        Self::VersionConflict {
            dependency: dependency.into(),
            requested: requested.into(),
            conflicting: conflicting.into(),
        }
    }

    pub fn invalid_dependency_spec<S: Into<String>>(spec: S, reason: S) -> Self {
        Self::InvalidDependencySpec {
            spec: spec.into(),
            reason: reason.into(),
        }
    }

    pub fn missing_configuration<S: Into<String>>(field: S) -> Self {
        Self::MissingConfiguration {
            field: field.into(),
        }
    }

    pub fn repository<S: Into<String>>(message: S) -> Self {
        Self::Repository {
            message: message.into(),
        }
    }

    pub fn project_init<S: Into<String>>(reason: S) -> Self {
        Self::ProjectInit {
            reason: reason.into(),
        }
    }

    pub fn invalid_project<S: Into<String>>(issue: S) -> Self {
        Self::InvalidProject {
            issue: issue.into(),
        }
    }

    pub fn validation<S: Into<String>>(message: S) -> Self {
        Self::Validation {
            message: message.into(),
        }
    }
}

pub type Result<T> = std::result::Result<T, JellyError>;