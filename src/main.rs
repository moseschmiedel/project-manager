use anyhow::{anyhow, Context, Result};
use clap::{Args, Parser, Subcommand};
use project_manager::error::Error;
use std::{ffi::OsString, fs, io, path::PathBuf, process};

const CONFIG_NAME: &str = "project-manager";

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
struct Cli {
    /// Directory where projects are stored
    #[arg(short, long = "projects-root")]
    project_dir_path: std::path::PathBuf,

    #[command(subcommand)]
    command: Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Cd(CdArgs),
    ListProjects,
    Clone(CloneArgs),
    New(NewArgs),
}

#[derive(Args)]
#[command(author, version, about="Create new project", long_about=None)]
struct NewArgs {
    /// Name of new project
    project_name: String,

    /// Generator used for creating new project
    #[arg(short, long = "generator", default_value = "git")]
    generator: String,
}

#[derive(Args)]
#[command(author, version, about = "Change directory to specified project root", long_about = None)]
struct CdArgs {
    /// Project to switch to
    project_name: String,
}

#[derive(Args)]
#[command(author, version, about = "Clone project from specified git URL", long_about = None)]
struct CloneArgs {
    /// git URL to clone from
    url: String,
    /// Parent directory to clone project into
    #[arg(short, long = "project-name")]
    project_name: Option<String>,
    /// Parent directory to clone project into
    #[arg(short, long = "parent-dir")]
    directory: Option<std::path::PathBuf>,
}

fn try_init_config_dir() -> Result<()> {
    // Priority which directory should be used for config
    // 1. $XDG_CONFIG_HOME/<CONFIG_NAME>
    // 2. $HOME/.config/<CONFIG_NAME>

    // config_dir/
    //      - config.yaml
    //      - projects/
    //          - coocook/
    //              - backup/
    //                  -
    //              - publish/
    //                  -
    //      - commands/ <---- maybe create DSL
    //          - publish-git/
    //              - danger.sh
    //              - cleanup.sh
    //          - publish-mycustom
    //          - new-git
    //          - new-perl
    //              - 00-mkdir.sh
    //              - 01-git-init.sh
    //              - 02-init-cpanfile.sh
    //      =============================
    //      - publish/
    //          - git/
    //          - dockerhub/
    //          - gitrelease/
    //      - generate/
    //          - git/
    //                  - 00-init.sh

    fn empty_os_string_to_none(os_str: OsString) -> Option<PathBuf> {
        if os_str.is_empty() {
            None
        } else {
            Some(os_str.into())
        }
    }

    let config_location: PathBuf = std::env::var_os("XDG_CONFIG_HOME")
        .and_then(empty_os_string_to_none)
        .or(std::env::var_os("HOME").and_then(empty_os_string_to_none).map(|path| path.join(".config")))
        .map(|path| path.join(CONFIG_NAME))
        .ok_or(Error::CouldNotDetermineConfigLocation(vec![
            format!("$XDG_CONFIG_HOME/{}", CONFIG_NAME),
            format!("$HOME/.config/{}", CONFIG_NAME),
        ]))?;

    match fs::metadata(&config_location) {
        Ok(_) => Ok(()),
        Err(ref err) if err.kind() == io::ErrorKind::PermissionDenied => {
            Err(anyhow!("No Permission for '{}'", config_location.display()))
        }
        Err(ref err) if err.kind() == io::ErrorKind::NotFound => fs::create_dir(&config_location)
            .with_context(|| {
                format!(
                    "Failed to create config directory at '{}'",
                    config_location.display()
                )
            }),
        Err(_) => Err(anyhow!("")),
    }
}

fn main() -> Result<()> {
    try_init_config_dir()?;
    let cli = Cli::parse();
    let project_dir_path = PathBuf::from(cli.project_dir_path).canonicalize()?;

    match &cli.command {
        Some(Commands::Cd(args)) => {
            let project_home_dir = project_dir_path.read_dir()?;
            for dir_entry in project_home_dir {
                let path = dir_entry.unwrap().path().to_owned();
                if path.is_dir() {
                    let name = path.file_name().unwrap().to_str().unwrap().to_owned();
                    if name == args.project_name || format!("{name}/") == args.project_name {
                        println!("{}", path.as_path().display());
                    }
                }
            }
            Ok(())
        }
        Some(Commands::ListProjects) => {
            let project_home_dir = project_dir_path.read_dir()?;
            for dir_entry in project_home_dir {
                let path = dir_entry.unwrap().path().to_owned();
                if path.is_dir() {
                    println!("{}", path.as_path().file_name().unwrap().to_string_lossy());
                }
            }

            Ok(())
        }
        Some(Commands::New(args)) => {
            let project_dir = project_dir_path.join(&args.project_name);
            fs::create_dir(&project_dir)
                .with_context(|| format!("Cannot create project '{}'", &args.project_name))?;
            if args.generator == "git" {
                process::Command::new("git")
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
        Some(Commands::Clone(args)) => {
            // TODO: Maybe do some checks on the specified URL before passing it
            // to git
            let mut git = process::Command::new("git");
            git.current_dir(args.directory.clone().unwrap_or(project_dir_path))
                .arg("clone")
                .arg(args.url.clone());

            match &args.project_name {
                None => (),
                Some(project_name) => {
                    git.arg(project_name);
                }
            };

            let mut handle = git.spawn()?;
            handle.wait()?;
            Ok(())
        }
        None => Ok(()),
    }
}
