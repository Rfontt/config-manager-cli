
use serde::{Deserialize, Serialize};
use std::path::{Path};

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
