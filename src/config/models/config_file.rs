use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Path};

use crate::error::Result;
use crate::config::config_format::ConfigFormat;
use super::file_metadata::FileMetadata;
use super::filesystem_entity::FilesystemEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ConfigFile {
    pub name: String,
    pub path: PathBuf,
    pub format: ConfigFormat,
    pub tool: String,
    pub exists: bool,
    pub size_bytes: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

impl ConfigFile {
    pub fn new(name: String, path: PathBuf, tool: String) -> Result<Self> {
        let format = ConfigFormat::from_path(&path);
        let metadata = FileMetadata::from_path(&path);

        Ok(ConfigFile {
            name,
            path,
            format,
            tool,
            exists: metadata.exists,
            size_bytes: metadata.size_bytes,
            last_modified: metadata.last_modified,
        })
    }

    pub fn short_name(&self) -> String {
        self.path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or(&self.name)
            .to_string()
    }
}

impl FilesystemEntity for ConfigFile {
    fn path(&self) -> &Path {
        &self.path
    }
}
