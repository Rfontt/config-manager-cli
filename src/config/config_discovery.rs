use crate::config::{ConfigFile, ToolRegistry};
use crate::error::Result;
use std::path::PathBuf;

pub struct ConfigDiscovery {
    registry: ToolRegistry,
}

impl ConfigDiscovery {
    pub fn new() -> Self {
        ConfigDiscovery {
            registry: ToolRegistry::new(),
        }
    }

    pub fn discover_all(&self) -> Result<Vec<ConfigFile>> {
        let mut configs = Vec::new();

        for tool in self.registry.all_tools() {
            if let Some(paths) = self.registry.get_paths(&tool) {
                for path_str in paths {
                    if let Ok(path) = expand_path(&path_str) {
                        if path.exists() {
                            if let Ok(config) = ConfigFile::new(tool.clone(), path, tool.clone()) {
                                configs.push(config);
                            }
                        }
                    }
                }
            }
        }

        configs.sort_by(|a, b| a.name.cmp(&b.name));
        Ok(configs)
    }

    pub fn discover_tool(&self, tool: &str) -> Result<Vec<ConfigFile>> {
        let mut configs = Vec::new();

        if let Some(paths) = self.registry.get_paths(tool) {
            for path_str in paths {
                if let Ok(path) = expand_path(&path_str) {
                    if path.exists() {
                        if let Ok(config) =
                            ConfigFile::new(tool.to_string(), path, tool.to_string())
                        {
                            configs.push(config);
                        }
                    }
                }
            }
        }

        Ok(configs)
    }

    pub fn list_tools(&self) -> Vec<String> {
        self.registry.all_tools()
    }
}

impl Default for ConfigDiscovery {
    fn default() -> Self {
        Self::new()
    }
}

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
