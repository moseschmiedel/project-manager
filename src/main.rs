use anyhow::Result;
use clap::Parser;

use project_manager::cli::{self, Cli};
use project_manager::command;
use project_manager::data::config;

fn main() -> Result<()> {
    config::try_init_config_dir()?;
    let cli = Cli::parse();

    match cli.command {
        // Commands that don't require project directory
        Some(cli::Commands::ListCommands) => command::list_commands(),
        Some(cli::Commands::SupportedVersion(args)) => command::supported_version(args),

        // Commands that require project directory
        Some(cli::Commands::Cd(args)) => command::cd(args),
        Some(cli::Commands::ListProjects(args)) => command::list_projects(args),
        Some(cli::Commands::New(args)) => command::new(args),
        Some(cli::Commands::Clone(args)) => command::clone(args),
        None => Ok(()),
    }
}
