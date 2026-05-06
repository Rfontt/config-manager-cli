pub mod discovery;

use crate::error::Result;
use serde::{Deserialize, Serialize};
use std::path::{Path, PathBuf};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum ConfigFormat {
    Json,
    Yaml,
    Toml,
    Conf,
    Shell,
    Unknown,
}

impl ConfigFormat {
    pub fn from_path(path: &Path) -> Self {
        match path.extension().and_then(|s | s.to_str()) {
            Some("json") => ConfigFormat::Json,
            Some("yaml") | Some("yml") => ConfigFormat::Yaml,
            Some("toml") => ConfigFormat::Toml,
            Some("conf") | Some("config") => ConfigFormat::Conf,
            Some("sh") => ConfigFormat::Shell,
            _ => ConfigFormat::Unknown,
        }
    }

    pub fn as_str(&self) -> &str {
        match self {
            ConfigFormat::Json => "json",
            ConfigFormat::Yaml => "yaml",
            ConfigFormat::Toml => "toml",
            ConfigFormat::Conf => "conf",
            ConfigFormat::Shell => "shell",
            ConfigFormat::Unknown => "unknown",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub name: String,
    pub path: PathBuf,
    pub format: ConfigFormat,
    pub tool: String,
    pub exists: bool,
    pub size_bytes: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>
}

impl ConfigFile {
    pub fn new(name: String, path: PathBuf, tool: String) -> Result<Self> {
        let format = ConfigFormat::from_path(&path);
        let metadata = std:fs:metadata(&path);
        let (exists, size_bytes, last_modified) = match metadata {
            Ok(m) => {
                let modified = m.modified().ok
                ().and_then(|t | {
                    let elapsed = t.elapsed().ok()?;
                    let duration = std::time::SystemTime::now
                    ().checked_sub(elapsed)
                    .and_then(|t | t.duration_since(std::time::UNIX_EPOCH).ok())?;
                Some(chrono::DateTime::<chrono::Utc>::from(
                    std::time::UNIX_EPOCH + duration,
                ))
                });
                (true, m.len(), modified)
            }
            Err(_) => (false, 0, None),
        };

        Ok(ConfigFile {
            name,
            path,
            format,
            tool,
            exists,
            size_bytes,
            last_modified,
        })
    }

    pub fn short_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.name)
            .to_string()
    }

    pub fn display_path(&self) -> String {
        self.path.to_str().unwrap_or("???").replace(
            &dirs::home_dir()
                .and_then(|p| p.to_str().map(|s| s.to_string()))
                .unwrap_or_default(),
            "~",
        )
    }
}

pub struct ToolRegistry {
    tools: Vec<(String, Vec<String>)>
}

impl ToolRegistry {
    pub fn new() -> Self {
         let tools = vec![
            (
                "aerospace".to_string(),
                vec!["~/.config/aerospace/aerospace.toml".to_string()],
            ),
            (
                "alacritty".to_string(),
                vec![
                    "~/.config/alacritty/alacritty.toml".to_string(),
                    "~/.config/alacritty/alacritty.yml".to_string(),
                ],
            ),
            (
                "bash".to_string(),
                vec![
                    "~/.bashrc".to_string(),
                    "~/.bash_profile".to_string(),
                    "~/.bashenv".to_string(),
                ],
            ),
            (
                "zsh".to_string(),
                vec![
                    "~/.zshrc".to_string(),
                    "~/.zshenv".to_string(),
                    "~/.zprofile".to_string(),
                ],
            ),
            (
                "neovim".to_string(),
                vec![
                    "~/.config/nvim/init.lua".to_string(),
                    "~/.config/nvim/init.vim".to_string(),
                ],
            ),
            (
                "vim".to_string(),
                vec!["~/.vimrc".to_string(), "~/.vim/vimrc".to_string()],
            ),
            ("git".to_string(), vec!["~/.gitconfig".to_string()]),
            (
                "gitignore".to_string(),
                vec!["~/.gitignore_global".to_string()],
            ),
            ("ssh".to_string(), vec!["~/.ssh/config".to_string()]),
            ("tmux".to_string(), vec!["~/.tmux.conf".to_string()]),
            (
                "homebrew".to_string(),
                vec!["~/.config/homebrew/brewfile".to_string()],
            ),
            (
                "espanso".to_string(),
                vec![
                    "~/Library/Application Support/espanso/config/default.yml".to_string(),
                    "~/Library/Application Support/espanso/config/default.yaml".to_string(),
                ],
            ),
        ];

        ToolRegistry { tools }
    }

    pub fn get_paths(&self, tool: &str) -> Option<Vec<String>> {
        self.tools
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(tool))
            .map(|(_, paths)| paths.clone())
    }

    pub fn all_tools(&self) -> Vec<String> {
        self.tools.iter().map(|(name, _)| name.clone()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}