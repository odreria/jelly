
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Project {
    #[serde(rename = "parent")]
    pub parent: Option<Parent>,
    #[serde(rename = "modelVersion")]
    pub model_version: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    #[serde(rename = "licenses")]
    pub licenses: Option<Licenses>,
    #[serde(rename = "properties")]
    pub properties: Option<Properties>,
    #[serde(rename = "dependencies")]
    pub dependencies: Option<Dependencies>,
    #[serde(rename = "build")]
    pub build: Option<Build>,
}

#[derive(Debug, Deserialize)]
pub struct Parent {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Licenses {
    #[serde(rename = "license")]
    pub licenses: Vec<License>,
}

#[derive(Debug, Deserialize)]
pub struct License {
    #[serde(rename = "name")]
    pub name: Option<String>,
    #[serde(rename = "url")]
    pub url: Option<String>,
    #[serde(rename = "distribution")]
    pub distribution: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Properties {
    #[serde(rename = "doc.skip")]
    pub doc_skip: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Dependencies {
    #[serde(rename = "dependency")]
    pub dependencies: Option<Vec<Dependency>>,
}

#[derive(Debug, Deserialize)]
pub struct Dependency {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "optional")]
    pub optional: Option<String>,
    #[serde(rename = "scope")]
    pub scope: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Build {
    #[serde(rename = "pluginManagement")]
    pub plugin_management: Option<PluginManagement>,
    #[serde(rename = "plugins")]
    pub plugins: Option<Plugins>,
}

#[derive(Debug, Deserialize)]
pub struct PluginManagement {
    #[serde(rename = "plugins")]
    pub plugins: Option<Vec<Plugin>>,
}

#[derive(Debug, Deserialize)]
pub struct Plugins {
    #[serde(rename = "plugin")]
    pub plugins: Vec<Plugin>,
}

#[derive(Debug, Deserialize)]
pub struct Plugin {
    #[serde(rename = "groupId")]
    pub group_id: Option<String>,
    #[serde(rename = "artifactId")]
    pub artifact_id: Option<String>,
    #[serde(rename = "version")]
    pub version: Option<String>,
    #[serde(rename = "configuration")]
    pub configuration: Option<Configuration>,
}

#[derive(Debug, Deserialize)]
pub struct Configuration {
    #[serde(rename = "excludes")]
    pub excludes: Option<Excludes>,
    #[serde(rename = "classpathDependencyExcludes")]
    pub classpath_dependency_excludes: Option<ClasspathDependencyExcludes>,
    #[serde(rename = "systemProperties")]
    pub system_properties: Option<SystemProperties>,
    #[serde(rename = "includes")]
    pub includes: Option<Includes>,
}

#[derive(Debug, Deserialize)]
pub struct Excludes {
    #[serde(rename = "exclude")]
    pub exclude: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct ClasspathDependencyExcludes {
    #[serde(rename = "classpathDependencyExclude")]
    pub classpath_dependency_exclude: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct SystemProperties {
    #[serde(rename = "io.vertx.web.route.param.extended-pattern")]
    pub extended_pattern: Option<String>,
}

#[derive(Debug, Deserialize)]
pub struct Includes {
    #[serde(rename = "include")]
    pub include: Vec<String>,
}
