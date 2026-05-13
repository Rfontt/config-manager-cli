use clap::Parser;
use config_manager::cli::{Cli, Command};
use config_manager::error::Result;
use config_manager::handler::handle_list;
use config_manager::handler::handle_edit;

fn main() -> Result<()> {
    env_logger::init();

    let cli = Cli::parse();

    if cli.verbose {
        log::set_max_level(log::LevelFilter::Debug);
    }

    match cli.command {
        Some(Command::List { tool, detailed, projects }) => handle_list(tool, detailed, projects)?,
        Some(Command::Edit {
            config,
            code,
            vscode,
            vim,
            neovim,
            editor,
        }) => handle_edit(config, code, vscode, vim, neovim, editor)?,
        None => {}
    }

    Ok(())
}
