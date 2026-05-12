use crate::error::{ConfigManagerError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub editor: Option<String>,
}

impl FileConfig {
    pub fn default_config_dir() -> Result<PathBuf> {
        let home = dirs::home_dir().ok_or_else(|| {
            ConfigManagerError::DirectoryError("Could not determine home directory".to_string())
        })?;

        Ok(home.join(".config-manager"))
    }

    pub fn config_file_path() -> Result<PathBuf> {
        let config_dir = Self::default_config_dir()?;
        Ok(config_dir.join("config.toml"))
    }

    pub fn load() -> Result<Self> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            return Ok(Self::default());
        }

        let content = fs::read_to_string(&config_path)?;
        let config: FileConfig = toml::from_str(&content)?;

        Ok(config)
    }

    pub fn editor(&self) -> Option<String> {
        self.editor
            .clone()
            .or_else(|| std::env::var("EDITOR").ok())
            .or_else(|| std::env::var("VISUAL").ok())
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            editor: None,
        }
    }
}
