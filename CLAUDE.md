# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Overview

Config Manager is a Rust CLI tool for discovering and managing configuration files across macOS, Linux, and Windows systems. It provides both a command-line interface and interactive shell access to 25+ tool configurations.

## Development Commands

**Build & Run**
```bash
cargo build                          # Debug build
cargo build --release                # Optimized release build
cargo run -- list                    # Run directly
cargo run -- edit zsh                # Run with arguments
```

**Testing**
```bash
cargo test                           # Run all tests
cargo test --lib                     # Library tests only
cargo test test_name                 # Specific test
```

**Development Tools**
```bash
cargo check                          # Fast compile check
cargo clippy                         # Lint checks
cargo fmt                            # Format code
```

## Architecture

The codebase follows a layered, modular structure:

### Core Modules

**`config/`** — Configuration discovery and management
- `config_discovery.rs`: Scans system and discovers configs using `ToolRegistry`
- `tool_registry.rs`: Registry of 25+ tool definitions and their config paths
- `tools_data.rs`: Tool metadata (shells, editors, tools, other)
- `config_file.rs`: File abstraction and metadata (path, size, format, timestamps)
- `config_format.rs`: Format detection and syntax validation (JSON, YAML, TOML, CONF, Shell)

**`handler/`** — Command execution layer
- `list_handler.rs`: Implements `config-manager list` command
- `edit_handler.rs`: Implements `config-manager edit` command with editor selection

**`editor/`** — File operations and editor integration
- `file_config.rs`: Configuration wrapper for file operations
- `file_repository.rs`: Abstraction for file I/O and editor launching

**`cli.rs`** — Command-line argument parsing (uses `clap`)

**`error.rs`** — Centralized error handling (custom `ConfigManagerError` enum with conversion impls)

### Data Flow

```
main.rs (CLI parser)
  → handler/* (commands)
    → config/config_discovery.rs (discovers configs)
      → config/tool_registry.rs (tool definitions)
      → config/config_file.rs (file abstraction)
    → editor/file_repository.rs (file operations)
      → config/config_format.rs (validation)
```

## Key Design Patterns

- **Error Handling**: All operations return `Result<T>` using `ConfigManagerError`. Conversion traits auto-implement from std types.
- **Tool Registry**: Centralized in `ToolRegistry` — adding a new tool only requires updating `tools_data.rs`.
- **Format Detection**: Automatic based on file extension; validation errors are surfaced to handlers.
- **Path Expansion**: `~` and environment variables expanded before file operations.

## Common Development Tasks

**Adding a new tool**
1. Add tool definition to `tools_data.rs` with paths
2. ToolRegistry will auto-discover it
3. `list` and `edit` commands will work immediately

**Modifying config discovery logic**
- Core logic in `config_discovery.rs` (`discover_all()`, `discover_tool()`)
- Path expansion happens in `expand_path()` function
- File existence checked before creating `ConfigFile`

**Adding a new command**
1. Add variant to `Command` enum in `cli.rs`
2. Handle in `main.rs` match statement
3. Create handler in `handler/` module
4. Export from `handler/mod.rs`

**Error handling**
- Add variant to `ConfigManagerError` enum in `error.rs`
- Use `#[from]` for automatic conversion from standard library types
- Custom variants can use `From<String>` impls for ad-hoc errors

## Testing Strategy

Tests are integration tests using `assert_cmd` and `predicates` crates for CLI testing. Most test fixtures use `tempdir` for temporary file operations.
