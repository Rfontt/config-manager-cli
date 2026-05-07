use serde::{Deserialize, Serialize};
use std::path::{PathBuf};

use crate::error::Result;
use crate::config::config_format::ConfigFormat;

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
        let metadata = std::fs::metadata(&path);
        let (exists, size_bytes, last_modified) = match metadata {
            Ok(m) => {
                let modified = m.modified().ok().and_then(|t| {
                    let elapsed = t.elapsed().ok()?;
                    let duration = std::time::SystemTime::now()
                        .checked_sub(elapsed)
                        .and_then(|t| t.duration_since(std::time::UNIX_EPOCH).ok())?;
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