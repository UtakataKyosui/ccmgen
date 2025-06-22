# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

ccmgen is a Rust CLI tool that intelligently detects project types and generates Claude Code command templates. It supports Rust (Normal/WASM), JavaScript, TypeScript, and Node.js projects with automatic project detection and language-specific template generation.

## Build and Development Commands

```bash
# Build the project
cargo build

# Check compilation without building
cargo check

# Run the CLI tool
./target/debug/ccmgen

# Test CLI commands
./target/debug/ccmgen detect     # Analyze current project
./target/debug/ccmgen init       # Generate commands with auto-detection
./target/debug/ccmgen list       # Show created commands
./target/debug/ccmgen config     # Initialize configuration
```

## Architecture

### Core Modules
- **`main.rs`** - CLI entry point using clap with derive macros
- **`project.rs`** - Project detection engine with intelligent file analysis
- **`templates.rs`** - Language-specific template management
- **`commands.rs`** - CLI command implementations
- **`config.rs`** - TOML-based configuration system

### Key Data Flow
1. Project detection analyzes filesystem (Cargo.toml, package.json, etc.)
2. ProjectDetector determines ProjectType (RustNormal, RustWasm, JavaScript, TypeScript, NodeJs)
3. TemplateManager selects appropriate templates based on detected features
4. Commands are generated in `~/.claude/commands/` with project context

### Project Detection Logic
- **Rust Projects**: Cargo.toml presence + WASM detection via wasm-bindgen deps, cdylib crate-type, or wasm-pack metadata
- **JS/TS Projects**: package.json + TypeScript config files or Node.js-specific dependencies
- **Feature Detection**: Extracts dependencies, dev-dependencies, scripts, and project-specific configurations

### Template System
Each project type has specialized templates:
- **Rust Normal**: Performance analysis, test generation, documentation, async conversion
- **Rust WASM**: wasm-bindgen wrappers, memory optimization, browser integration
- **JavaScript**: ES6+ modernization, Promise conversion, bundle analysis
- **TypeScript**: Type annotation, interface design, strict mode fixes
- **Node.js**: Express middleware, API endpoints, authentication, Docker setup

Templates are enhanced with project context including name, type, detected features, and file paths.

## Configuration

- **Global Config**: `~/.claude/ccmgen.toml` for custom templates and settings
- **Generated Commands**: `~/.claude/commands/*.md` files
- **Default Settings**: Auto-detection enabled, TypeScript preference, includes tests/docs

## Development Notes

- Uses Rust 2024 edition with clap for CLI, dialoguer for interactive prompts
- File operations use cross-platform dirs crate for home directory detection
- TOML/JSON parsing for configuration and project file analysis
- Japanese language support in CLI messages and help text
- Extensible architecture supports custom templates and project types

## Integration

Tool integrates with Claude Code's MCP server configuration and generates commands following Claude Code patterns. Templates include project-specific context for more accurate and relevant code assistance.