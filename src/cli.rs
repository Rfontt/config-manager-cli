use clap::{Parser, Subcommand};

#[derive(Parser, Debug)]
#[command(name = "config-manager")]
#[command(version = "0.1.0")]
#[command(about = "Discover, manage, and version configuration files", long_about = None)]
pub struct Cli {

    #[command(subcommand)]
    pub command: Option<Command>,

    #[arg(short, long)]
    pub verbose: bool,

    #[arg(short, long)]
    pub config_dir: Option<String>
}

#[derive(Subcommand, Debug)]
pub enum Command {
    List {
        #[arg(value_name = "TOOL")]
        tool: Option<String>,

        #[arg(short, long)]
        detailed: bool
    },

    Edit {
        #[arg[value_name = "CONFIG"]]
        config: String,

        #[arg[long]]
        code: bool,

        #[arg[long, alias = "vs"]]
        vscode: bool,

        #[arg[long]]
        vim: bool,

        #[arg[long, alias = "neovim"]]
        neovim: bool,

        #[arg(short, long)]
        editor: Option<String>
    }
}

impl Cli {
    pub fn parse_args() -> Self {
        Parser::parse()
    }
}