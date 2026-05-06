# Config Manager 🔧

A powerful macOS CLI tool written in Rust for discovering and managing configuration files.

## Features

- 🔍 **Auto-discovery**: Finds 25+ common config files across your system
- 📝 **Interactive TUI**: Full-featured terminal UI for browsing and managing configs
- 🔧 **Powerful CLI**: Scriptable commands for automation
- 🎯 **Smart Validation**: Syntax checking for JSON, YAML, TOML, CONF, and Shell files

## Installation

### From Source (macOS, Linux, Windows)
```bash
git clone https://github.com/yourusername/config-manager-cli
cd config-manager
cargo build --release
./target/release/config-manager --help
```

### Requirements
- Rust 1.70+ ([Install Rust](https://rustup.rs))
- macOS (primary target), Linux or Windows

## Quick Start

### 1. Initialize
```bash
config-manager init
```

### 2. Discover Your Configs
```bash
config-manager list
```

### 3. Edit a Config
```bash
config-manager edit zsh
# Launches $EDITOR with your config file
```

## Supported Tools (25+)

| Shells | Editors | Tools | Other |
|--------|---------|-------|-------|
| bash | neovim | git | aerospace |
| fish | vim | ssh | alacritty |
| zsh | | tmux | docker |
| | | kitty | homebrew |
| | | wezterm | node |
| | | | python |
| | | | ruby |
| | | | rust |

_And more! See `config-manager list` for complete list._

## Command Reference

### List Configs
```bash
config-manager list              # All configs
config-manager list --tool git   # Specific tool
config-manager list --detailed   # With timestamps
```

## Project Architecture

```
config-manager/
├── src/
│   ├── main.rs              # CLI entry point & handlers
│   ├── lib.rs               # Library exports
│   ├── error.rs             # Error handling
│   ├── cli.rs               # Command parser
│   ├── config/              # Config discovery
│   │   ├── mod.rs           # Core types
│   │   └── discovery.rs     # Discovery engine
│   ├── editor/              # File operations
│   │   ├── file_ops.rs      # Read/write/backup
│   │   ├── diff.rs          # Diff generation
│   │   └── validation.rs    # Syntax validation
│   ├── storage/             # Configuration
│   │   └── mod.rs           # AppConfig
├── tests/
│   └── integration_test.rs  # E2E tests
├── completions/             # Shell completions
│   ├── config-manager.bash  # Bash
│   └── config-manager.zsh   # Zsh
└── Cargo.toml               # Manifest
```

## Testing

Run all tests:
```bash
cargo test --all
```

Run specific test:
```bash
cargo test editor::file_ops
```

With output:
```bash
cargo test -- --nocapture
```

Integration tests:
```bash
cargo test --test integration_test
```

## Development

See [CONTRIBUTING.md](CONTRIBUTING.md) for:
- Development setup
- Code style guidelines
- Testing requirements
- Pull request process
- Release process

## Security

- No external API calls
- All data stored locally
- File permissions preserved
- Atomic writes prevent corruption

## License

MIT License - See LICENSE file for details

---

**Made with ❤️ in Rust**
