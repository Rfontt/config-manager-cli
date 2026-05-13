use crate::config::{ConfigFile, ToolRegistry};
use crate::error::Result;
use super::common::expand_path;

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
