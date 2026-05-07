use clap::Parser;
use config_manager::cli::{Cli, Command};
use config_manager::config::config_discovery::ConfigDiscovery;
use config_manager::error::Result;

mod handlers {
    // TODO
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
                config_manager::ConfigManagerError::ConfigNotFound(config_name.clone())
            })?;

        if !config.exists {
            return Err(config_manager::ConfigManagerError::FileOperation(format!(
                "Config file does not exist: {}",
                config.display_path()
            )));
        }

        let editor = if let Some(custom_editor) = editor_opt {
            custom_editor
        } else if code || vscode {
            "code".to_string()
        } else if vim {
            "vim".to_string()
        } else if neovim {
            "nvim".to_string()
        } else {
            let app_config = AppConfig::load()?;
            app_config.editor().ok_or_else(|| {
                config_manager::ConfigManagerError::CliError(
                    "No editor configured. Set EDITOR environment variable.".to_string(),
                )
            })?
        };

        std::process::Command::new(&editor)
            .arg(&config.path)
            .status()
            .map_err(|e| {
                config_manager::ConfigManagerError::CliError(format!(
                    "Failed to launch editor '{}': {}",
                    editor, e
                ))
            })?;

        println!("Edited: {}", config.display_path());
        Ok(())
    }
}

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    if cli.verbose {
        log::set_max_level(log::LevelFilter::Debug);
    }

    match cli.command {
        Some(Command::List { tool, detailed }) => handlers::handle_list(tool, detailed)?,
        Some(Command::Edit {
            config,
            code,
            vscode,
            vim,
            neovim,
            editor,
        }) => handlers::handle_edit(config, code, vscode, vim, neovim, editor)?,
        Some(Command::Init { versioning_dir }) => handlers::handle_init(versioning_dir)?,
    }

    Ok(())
}
