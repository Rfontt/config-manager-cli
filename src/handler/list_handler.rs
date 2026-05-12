use crate::config::config_discovery::ConfigDiscovery;
use crate::error::Result;

pub fn handle_list(tool: Option<String>, detailed: bool) -> Result<()> {
    let discovery = ConfigDiscovery::new();

    let configs = if let Some(tool_name) = tool {
        discovery.discover_tool(&tool_name)?
    } else {
        discovery.discover_all()?
    };

    if configs.is_empty() {
        println!("No configuration files discovered.");
        return Ok(());
    }

    if detailed {
        println!(
            "\n{:<20} {:<50} {:<10} {:<15}",
            "Tool", "Path", "Format", "Size"
        );

        println!("{}", "-".repeat(95));

        for config in configs {
            let size = if config.exists {
                format!("{} B", config.size_bytes)
            } else {
                "N/A".to_string()
            };

            let modified = config
                .last_modified
                .map(|t| t.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            println!(
                "{:<20} {:<50} {:<10} {:<15}",
                config.tool,
                config.display_path(),
                config.format.as_str(),
                size
            );
            println!("  Modified: {}", modified);
            println!();
        }
    } else {
         println!(
             "{:<20} {:<40} {:<10} {:<15}",
             "Tool", "Path", "Format", "Size"
         );
         println!("{}", "-".repeat(85));

         for config in configs {
             let size = if config.exists {
                 format!("{} B", config.size_bytes)
             } else {
                 "N/A".to_string()
             };
             println!(
                 "{:<20} {:<40} {:<10} {:<15}",
                 config.tool,
                 config.display_path(),
                 config.format.as_str(),
                 size
             );
         }
    }

    Ok(())
}
