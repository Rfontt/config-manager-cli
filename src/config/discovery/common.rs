use crate::error::Result;
use std::path::PathBuf;

pub fn expand_path(path: &str) -> Result<PathBuf> {
    if path.starts_with("~") {
        let home = dirs::home_dir().ok_or_else(|| {
            crate::error::ConfigManagerError::DirectoryError(
                "Could not determine home directory".to_string(),
            )
        })?;

        let path_without_tilde = if path.len() > 1 && path.starts_with("~/") {
            &path[2..]
        } else if path == "~" {
            ""
        } else {
            &path[1..]
        };
        Ok(home.join(path_without_tilde))
    } else {
        Ok(PathBuf::from(path))
    }
}
