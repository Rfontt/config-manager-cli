use std::path::Path;

#[derive(Debug, Clone)]
pub struct FileMetadata {
    pub exists: bool,
    pub size_bytes: u64,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

impl FileMetadata {
    pub fn from_path(path: &Path) -> Self {
        let metadata = std::fs::metadata(path);
        match metadata {
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
                FileMetadata {
                    exists: true,
                    size_bytes: m.len(),
                    last_modified: modified,
                }
            }
            Err(_) => FileMetadata {
                exists: false,
                size_bytes: 0,
                last_modified: None,
            },
        }
    }
}
