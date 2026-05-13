use serde::{Deserialize, Serialize};
use std::path::{PathBuf, Path};

use super::file_metadata::FileMetadata;
use super::filesystem_entity::FilesystemEntity;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProjectFile {
    pub name: String,
    pub path: PathBuf,
    pub exists: bool,
    pub size_bytes: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
    pub markers: Vec<String>,
}

impl ProjectFile {
    pub fn new(name: String, path: PathBuf, markers: Vec<String>) -> Self {
        let metadata = FileMetadata::from_path(&path);

        ProjectFile {
            name,
            path,
            exists: metadata.exists,
            size_bytes: metadata.size_bytes,
            last_modified: metadata.last_modified,
            markers,
        }
    }
}

impl FilesystemEntity for ProjectFile {
    fn path(&self) -> &Path {
        &self.path
    }
}
