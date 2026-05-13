use crate::config::discovery::files_discovery::ConfigDiscovery;
use crate::config::ProjectDiscovery;
use crate::config::models::FilesystemEntity;
use crate::error::Result;
use crate::editor::file_config::FileConfig;

pub fn handle_list(tool: Option<String>, detailed: bool, projects: bool) -> Result<()> {
    if projects {
        return handle_list_projects(detailed);
    }
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

fn handle_list_projects(detailed: bool) -> Result<()> {
    let discovery = ProjectDiscovery::new()?;
    let settings = FileConfig::projects_settings()?;
    let projects_path = settings.get_path();

    let projects = discovery.discover_all()?;

    if projects.is_empty() {
        println!("No projects discovered in: {}", projects_path);
        return Ok(());
    }

    println!("\nProjects discovered at: {}\n", projects_path);

    if detailed {
        println!(
            "{:<30} {:<50} {:<20}",
            "Project", "Path", "Markers"
        );

        println!("{}", "-".repeat(100));

        for project in projects {
            let modified = project
                .last_modified
                .map(|t| t.format("%Y-%m-%d %H:%M").to_string())
                .unwrap_or_else(|| "Unknown".to_string());

            println!(
                "{:<30} {:<50} {:<20}",
                project.name,
                project.display_path(),
                project.markers.join(", ")
            );
            println!("  Modified: {}", modified);
            println!();
        }
    } else {
        println!(
            "{:<30} {:<50} {:<20}",
            "Project", "Path", "Markers"
        );
        println!("{}", "-".repeat(100));

        for project in projects {
            println!(
                "{:<30} {:<50} {:<20}",
                project.name,
                project.display_path(),
                project.markers.join(", ")
            );
        }
    }

    Ok(())
}
