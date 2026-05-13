use std::path::Path;

pub trait FilesystemEntity {
    fn path(&self) -> &Path;

    fn display_path(&self) -> String {
        self.path()
            .to_str()
            .unwrap_or("???")
            .replace(
                &dirs::home_dir()
                    .and_then(|p| p.to_str().map(|s| s.to_string()))
                    .unwrap_or_default(),
                "~",
            )
    }
}
