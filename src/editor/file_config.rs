use crate::error::{ConfigManagerError, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileConfig {
    pub editor: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectsSettings {
    pub path: Option<String>,
    pub project_markers: Option<Vec<String>>,
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

    pub fn projects_settings() -> Result<ProjectsSettings> {
        let config_path = Self::config_file_path()?;

        if !config_path.exists() {
            return Ok(ProjectsSettings::default());
        }

        let content = fs::read_to_string(&config_path)?;

        #[derive(Deserialize)]
        struct ConfigWithProjects {
            projects: Option<ProjectsSettings>,
        }

        let config: ConfigWithProjects = toml::from_str(&content)?;
        Ok(config.projects.unwrap_or_default())
    }
}

impl Default for FileConfig {
    fn default() -> Self {
        FileConfig {
            editor: None,
        }
    }
}

impl Default for ProjectsSettings {
    fn default() -> Self {
        ProjectsSettings {
            path: None,
            project_markers: None,
        }
    }
}

impl ProjectsSettings {
    pub fn get_path(&self) -> String {
        self.path
            .clone()
            .unwrap_or_else(|| "~/Documents/projects".to_string())
    }

    pub fn get_markers(&self) -> Vec<String> {
        self.project_markers
            .clone()
            .unwrap_or_else(|| {
                vec![
                    "Cargo.toml".to_string(),
                    "package.json".to_string(),
                    "pyproject.toml".to_string(),
                    ".git".to_string(),
                ]
            })
    }
}
