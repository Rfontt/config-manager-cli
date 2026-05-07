use super::tools_data::get_tools_registry;

pub struct ToolRegistry {
    tools: Vec<(String, Vec<String>)>
}

impl ToolRegistry {
    pub fn new() -> Self {
        let tools = get_tools_registry();
        ToolRegistry { tools }
    }

    pub fn get_paths(&self, tool: &str) -> Option<Vec<String>> {
        self.tools
            .iter()
            .find(|(name, _)| name.eq_ignore_ascii_case(tool))
            .map(|(_, paths)| paths.clone())
    }

    pub fn all_tools(&self) -> Vec<String> {
        self.tools.iter().map(|(name, _)| name.clone()).collect()
    }
}

impl Default for ToolRegistry {
    fn default() -> Self {
        Self::new()
    }
}
