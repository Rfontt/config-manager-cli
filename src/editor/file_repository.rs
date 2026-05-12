use crate::error::{ConfigManagerError, Result};
use std::fs;
use std::path::{Path};

pub fn read_file(path: &Path) -> Result<String> {
    if !path.exists() {
        return Err(ConfigManagerError::FileOperation(format!(
            "File not found: {}",
            path.display()
        )));
    }

    fs::read_to_string(path).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            ConfigManagerError::PermissionDenied(format!(
                "Cannot read: {} (permission denied)",
                path.display()
            ))
        } else {
            ConfigManagerError::Io(e)
        }
    })
}

pub fn write_file(path: &Path, content: &str) -> Result<()> {
    if let Some(parent) = path.parent() {
        if !parent.exists() {
            fs::create_dir_all(parent)?;
        }
    }

    fs::write(path, content).map_err(|e| {
        if e.kind() == std::io::ErrorKind::PermissionDenied {
            ConfigManagerError::PermissionDenied(format!(
                "Cannot write to: {} (permission denied)",
                path.display()
            ))
        } else {
            ConfigManagerError::Io(e)
        }
    })
}
