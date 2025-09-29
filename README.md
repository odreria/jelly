# Jelly

A simplified package manager for Java and Groovy applications, inspired by npm, poetry, and cargo.

## Vision

Jelly aims to make dependency management more accessible for developers coming from other ecosystems (Python, JavaScript, Rust) who find Maven and Gradle complex for getting started.

- **Simplicity**: Provide an npm-like experience for Java/Groovy development
- **Accessibility**: Lower the barrier to entry for new Java developers
- **Familiarity**: Use patterns familiar to developers from other language ecosystems
- **Integration**: Work with existing Maven infrastructure while abstracting complexity

## Current Status

**Development Phase 1** - Foundation ✅ CLI Structure Implemented

### ✅ Completed Features

- **Modern CLI Interface** with crossterm styling
- **Project Initialization** - `jelly init` command
- **Clean Architecture** foundation with proper error handling
- **TOML Configuration** parsing and generation

### 🔄 In Progress

- Core dependency resolution engine
- Maven repository integration
- Artifact downloading and installation

### 📋 Planned Features

- `jelly add <dependency>` - Add dependencies
- `jelly install` - Download and install all dependencies
- `jelly run [script]` - Execute project scripts
- Lock file support (jelly.lock)
- Development vs production dependencies

## Quick Start

### Installation

```bash
cargo build --release
```

### Initialize a New Project

```bash
jelly init my-java-app
```

This creates:
- `jelly.toml` - Project configuration file
- `libs/` - Directory for downloaded dependencies

### Example jelly.toml

```toml
[package]
name = "my-jdk-app"
version = "1.0.0"
main-class = "com.example.Main"

[dependencies]
"org.springframework:spring-core" = "6.0.0"
"junit:junit" = "4.13.2"

[scripts]
start = "java -cp 'libs/*' ${main-class}"
test = "java -cp 'libs/*:.' org.junit.runner.JUnitCore"
```

## Development

### Prerequisites

- Rust 1.70+
- Java 8+ (for testing)

### Build Commands

```bash
# Build the project
cargo build

# Run with arguments
cargo run -- init my-project

# Run tests
cargo test

# Check code
cargo check

# Lint code
cargo clippy
```

## Contributing

1. Fork the repository
2. Create a feature branch
3. Make your changes following the existing architecture
4. Add tests for new functionality
5. Submit a pull request

## License

This project is open source and available under the [MIT License](LICENSE).

## Inspiration

Jelly takes inspiration from modern package managers:
- **npm** (Node.js) - Simple configuration and commands
- **poetry** (Python) - Elegant dependency management
- **cargo** (Rust) - Straightforward project structure

But designed specifically for the Java/Groovy ecosystem while maintaining compatibility with Maven repositories.
