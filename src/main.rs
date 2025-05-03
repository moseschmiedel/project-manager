use anyhow::Result;
use clap::Parser;
use std::path::PathBuf;

use project_manager::cli::{self, Cli};
use project_manager::command;
use project_manager::data::config;

fn main() -> Result<()> {
    config::try_init_config_dir()?;
    let cli = Cli::parse();
    let project_dir_path = PathBuf::from(cli.project_dir_path).canonicalize()?;

    match cli.command {
        Some(cli::Commands::Cd(args)) => command::cd(project_dir_path, args),
        Some(cli::Commands::ListProjects) => command::list_projects(project_dir_path),
        Some(cli::Commands::New(args)) => command::new(project_dir_path, args),
        Some(cli::Commands::Clone(args)) => command::clone(project_dir_path, args),
        Some(cli::Commands::ListCommands) => command::list_commands(),
        None => Ok(()),
    }
}
