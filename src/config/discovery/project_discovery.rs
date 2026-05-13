use crate::config::models::ProjectFile;
use crate::editor::file_config::ProjectsSettings;
use crate::error::Result;
use std::path::Path;
use super::common::expand_path;

pub struct ProjectDiscovery {
    settings: ProjectsSettings,
}

impl ProjectDiscovery {
    pub fn new() -> Result<Self> {
        let settings = crate::editor::file_config::FileConfig::projects_settings()?;
        Ok(ProjectDiscovery { settings })
    }

    pub fn discover_all(&self) -> Result<Vec<ProjectFile>> {
        let path_str = self.settings.get_path();
        let markers = self.settings.get_markers();
        let path = expand_path(&path_str)?;

        if !path.exists() {
            return Ok(Vec::new());
        }

        let mut projects = Vec::new();

        if let Ok(entries) = std::fs::read_dir(&path) {
            for entry in entries.flatten() {
                if let Ok(metadata) = entry.metadata() {
                    if metadata.is_dir() {
                        if let Some(dir_name) = entry.file_name().to_str() {
                            let dir_path = entry.path();
                            if Self::is_project(&dir_path, &markers) {
                                let found_markers = Self::find_markers(&dir_path, &markers);
                                let project = ProjectFile::new(
                                    dir_name.to_string(),
                                    dir_path,
                                    found_markers,
                                );
                                projects.push(project);
                            }
                        }
                    }
                }
            }
        }

        projects.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(projects)
    }

    fn is_project(dir_path: &Path, markers: &[String]) -> bool {
        markers.iter().any(|marker| {
            let marker_path = dir_path.join(marker);
            marker_path.exists()
        })
    }

    fn find_markers(dir_path: &Path, markers: &[String]) -> Vec<String> {
        markers
            .iter()
            .filter(|marker| dir_path.join(marker).exists())
            .cloned()
            .collect()
    }
}

impl Default for ProjectDiscovery {
    fn default() -> Self {
        Self::new().unwrap_or_else(|_| {
            let settings = ProjectsSettings::default();
            ProjectDiscovery { settings }
        })
    }
}
