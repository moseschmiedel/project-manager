use anyhow::{Context, Result};
use clap::{Args, Parser, Subcommand};
use std::{fs, path::PathBuf, process::Command};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Directory where projects are stored
    #[arg(short, long = "projects")]
    project_dir_path: std::path::PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Cd(CdArgs),
    New(NewArgs),
}

#[derive(Args)]
#[command(author, version, about="Create new project", long_about=None)]
struct NewArgs {
    /// Name of new project
    project_name: String,

    /// Generator used for creating new project (default=git)
    #[arg(short, long = "generator", default_value = "git")]
    generator: String,
}

#[derive(Args)]
#[command(author, version, about = "Change directory to specified project root", long_about = None)]
struct CdArgs {
    /// Project to switch to
    project_name: String,
}

fn main() -> Result<()> {
    let cli = Cli::parse();
    let project_dir_path = PathBuf::from(cli.project_dir_path).canonicalize()?;

    match &cli.command {
        Some(Commands::Cd(args)) => {
            let project_home_dir = project_dir_path.read_dir()?;
            for dir_entry in project_home_dir {
                let path = dir_entry.unwrap().path().to_owned();
                if path.is_dir() {
                    let name = path.file_name().unwrap().to_str().unwrap().to_owned();
                    if name == args.project_name {
                        println!("{}", path.as_path().display());
                    }
                }
            }
            Ok(())
        }
        Some(Commands::New(args)) => {
            let project_dir = project_dir_path.join(&args.project_name);
            fs::create_dir(&project_dir)
                .with_context(|| format!("Cannot create project '{}'", &args.project_name))?;
            if args.generator == "git" {
                Command::new("git")
                    .arg("init")
                    .arg(project_dir)
                    .output()
                    .and(Ok(()))
            } else {
                Ok(())
            }
            .with_context(|| format!("Generator {} could not be executed", args.generator))?;
            Ok(())
        }
        None => Ok(()),
    }
}
