use crate::config::discovery::files_discovery::ConfigDiscovery;
use crate::config::models::FilesystemEntity;
use crate::error::Result;

pub fn handle_edit(
    config_name: String,
    code: bool,
    vscode: bool,
    vim: bool,
    neovim: bool,
    editor_opt: Option<String>,
) -> Result<()> {
    let discovery = ConfigDiscovery::new();
    let configs = discovery.discover_all()?;


    let config = configs
        .iter()
        .find(|c| c.name == config_name || c.short_name() == config_name)
        .ok_or_else(|| {
             crate::error::ConfigManagerError::ConfigNotFound(config_name.clone())
        })?;

    if !config.exists {
        return Err(crate::error::ConfigManagerError::FileOperation(format!(
            "Config file does not exist: {}",
            config.display_path()
        )));
    }

    let editor = 
        if let Some(custom_editor) = editor_opt {
            custom_editor
        } else if code || vscode {
            "code".to_string()
        } else if vim {
            "vim".to_string()
        } else if neovim {
            "nvim".to_string()
        } else {
            let app_config = crate::editor::FileConfig::load()?;
            app_config.editor().ok_or_else(|| {
                crate::error::ConfigManagerError::CliError(
                    "No editor configured. Set EDITOR environment variable.".to_string(),
                )
            })?
        };

    std::process::Command::new(&editor)
        .arg(&config.path)
        .status()
        .map_err(|e| {
            crate::error::ConfigManagerError::CliError(format!(
                "Failed to launch editor '{}': {}",
                editor, e
            ))
        })?;

    println!("Edited: {}", config.display_path());

    Ok(())
}