# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview
Jelly is a simplified package manager for Java and Groovy applications, inspired by npm, poetry, and cargo. The goal is to make dependency management more accessible for developers coming from other ecosystems (Python, JavaScript, Rust) who find Maven and Gradle complex for getting started.

### Vision
- **Simplicity**: Provide an npm-like experience for Java/Groovy development
- **Accessibility**: Lower the barrier to entry for new Java developers
- **Familiarity**: Use patterns familiar to developers from other language ecosystems
- **Integration**: Work with existing Maven infrastructure while abstracting complexity

The project leverages Maven's repository system but provides a simpler configuration format (jelly.toml) and streamlined workflow.

## Development Commands

### Rust Development
- `cargo build` - Build the project
- `cargo run` - Build and run the application
- `cargo build --release` - Build optimized release version
- `cargo test` - Run all tests
- `cargo check` - Quick syntax and type checking
- `cargo clippy` - Run linting

### Jelly CLI (Target Implementation)
- `jelly init` - Initialize new Java/Groovy project with jelly.toml
- `jelly add <dependency>` - Add dependency to project
- `jelly install` - Download and install all dependencies
- `jelly run [script]` - Execute project or custom script

## Architecture

### Current State (Legacy)
The current codebase is incomplete and needs refactoring. It contains:
- Basic POM parsing functionality in `src/core/gdp/`
- Maven repository downloading logic
- TOML configuration reading
- Mixed responsibilities and inconsistent error handling

### Target Architecture (Refactored Design)

The new architecture follows clean architecture principles with clear separation of concerns:

```
src/
├── cli/                    # Command Line Interface Layer
│   ├── commands/           # Individual CLI commands
│   │   ├── init.rs        # jelly init command
│   │   ├── add.rs         # jelly add <dependency> command
│   │   ├── install.rs     # jelly install command
│   │   ├── run.rs         # jelly run [script] command
│   │   └── mod.rs
│   ├── app.rs             # CLI application setup (clap)
│   └── mod.rs
├── core/                   # Business Logic Layer
│   ├── project/           # Project management
│   │   ├── config.rs      # Project configuration models
│   │   ├── manifest.rs    # jelly.toml parsing/writing
│   │   └── mod.rs
│   ├── resolver/          # Dependency resolution engine
│   │   ├── dependency.rs  # Dependency models and logic
│   │   ├── resolver.rs    # Resolution algorithm
│   │   ├── graph.rs       # Dependency graph management
│   │   └── mod.rs
│   ├── repository/        # Maven repository abstraction
│   │   ├── maven_repo.rs  # Maven repository client
│   │   ├── pom_parser.rs  # POM.xml parsing logic
│   │   ├── metadata.rs    # Maven metadata handling
│   │   └── mod.rs
│   ├── installer/         # Artifact installation
│   │   ├── downloader.rs  # Artifact downloading
│   │   ├── installer.rs   # Local installation logic
│   │   └── mod.rs
│   └── mod.rs
├── domain/                # Domain Models
│   ├── dependency.rs      # Core dependency representation
│   ├── artifact.rs        # Maven artifact models
│   ├── version.rs         # Version handling and comparison
│   └── mod.rs
├── infrastructure/        # Infrastructure Layer
│   ├── http.rs           # HTTP client abstraction
│   ├── filesystem.rs     # File system operations
│   └── mod.rs
├── errors/                # Error Handling
│   ├── jelly_error.rs    # Unified error types
│   └── mod.rs
├── main.rs               # CLI application entry point
└── lib.rs                # Library entry point
```

### Design Patterns Applied

#### 1. Command Pattern (CLI Commands)
Each CLI command implements a common interface:
```rust
#[async_trait]
pub trait Command {
    async fn execute(&self, args: &CommandArgs) -> Result<(), JellyError>;
}
```

#### 2. Repository Pattern (Maven Access)
Abstracts Maven repository operations:
```rust
#[async_trait]
pub trait Repository {
    async fn fetch_pom(&self, artifact: &Artifact) -> Result<Pom, JellyError>;
    async fn download_artifact(&self, artifact: &Artifact) -> Result<Vec<u8>, JellyError>;
}
```

#### 3. Builder Pattern (Project Configuration)
Simplifies project creation:
```rust
ProjectBuilder::new()
    .name("my-app")
    .add_dependency(Dependency::new("junit", "4.13.2"))
    .build()
```

#### 4. Strategy Pattern (Version Resolution)
Different version resolution strategies:
```rust
pub trait VersionResolver {
    fn resolve(&self, constraint: &str, available: &[Version]) -> Option<Version>;
}
```

### Configuration Files

#### Target jelly.toml Format
```toml
[package]
name = "my-java-app"
version = "1.0.0"
main-class = "com.example.Main"

[dependencies]
"org.springframework:spring-core" = "6.0.0"
"junit:junit" = "4.13.2"

[dev-dependencies]
"org.mockito:mockito-core" = "5.0.0"

[scripts]
start = "java -cp 'libs/*' ${main-class}"
test = "java -cp 'libs/*:.' org.junit.runner.JUnitCore"
```

#### Development Files
- **`Cargo.toml`** - Rust project dependencies (clap, tokio, reqwest, serde, thiserror)
- **`jelly.lock`** - Dependency lock file (planned)
- **`libs/`** - Downloaded JAR files directory (planned)

## Implementation Roadmap

### Phase 1: Foundation (Base sólida)
1. **Refactor error handling** - Replace `BeetleError` with `JellyError` using thiserror
2. **Implement CLI structure** - Set up clap-based command line interface
3. **Create domain models** - Clean dependency, artifact, and version types
4. **Project management** - jelly.toml parsing and writing

### Phase 2: Core Functionality
1. **Repository abstraction** - Maven repository client with proper error handling
2. **Dependency resolver** - Basic dependency resolution algorithm
3. **Artifact downloader** - Download and cache JAR files to libs/ directory
4. **POM parser refactor** - Clean, testable POM.xml parsing

### Phase 3: CLI Commands
1. **`jelly init`** - Initialize new project with jelly.toml template
2. **`jelly add`** - Add dependencies and update jelly.toml
3. **`jelly install`** - Download all dependencies and create classpath
4. **Testing framework** - Unit and integration tests

### Phase 4: Advanced Features
1. **`jelly run`** - Execute scripts with proper classpath
2. **Lock file (jelly.lock)** - Reproducible dependency resolution
3. **Development vs production dependencies** - Separate dev-dependencies
4. **Conflict resolution** - Handle version conflicts gracefully

### Current Issues to Address
- **Mixed responsibilities** in existing services
- **Inconsistent error handling** (mix of unwrap, expect, proper Result)
- **Hard-coded values** for URLs and paths
- **No test coverage** for existing functionality
- **Confusing naming** (gdp, beetle_error, etc.)

## Key Dependencies

### Rust Crates (Current + Planned)
- **`clap`** - Command line argument parsing
- **`tokio`** - Async runtime
- **`reqwest`** - HTTP client for Maven repositories
- **`serde`** - Serialization/deserialization
- **`toml`** - TOML configuration parsing
- **`quick-xml`** - XML parsing for POM files
- **`thiserror`** - Error handling
- **`regex`** - Pattern matching
- **`semver`** - Semantic version handling

## Project Goals & Inspiration
This project aims to bridge the gap between complex build tools (Maven/Gradle) and the simplicity that developers expect from modern package managers:

- **Like npm**: Simple `jelly.toml` configuration vs complex `pom.xml`
- **Like poetry**: Intuitive dependency specification and management
- **Like cargo**: Straightforward commands and clear project structure
- **For Java/Groovy**: Leverages existing Maven ecosystem while hiding complexity

The target audience includes developers transitioning from Python, JavaScript, or Rust who find traditional Java tooling overwhelming for simple projects.

### Compatibility Strategy
- **Maven Repository** - Full compatibility with Maven Central and other Maven repositories
- **Existing JAR files** - Works with all existing Java/Groovy libraries
- **Classpath generation** - Automatically generates proper Java classpaths
- **No lock-in** - Projects can be migrated to/from Maven/Gradle if needed