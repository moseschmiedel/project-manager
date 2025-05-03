use anyhow::{Context, Result};
use std::{fs, path::PathBuf, process};

use crate::cli;

pub fn cd(project_dir_path: PathBuf, args: cli::CdArgs) -> Result<()> {
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

pub fn list_projects(project_dir_path: PathBuf) -> Result<()> {
    let project_home_dir = project_dir_path.read_dir()?;
    for dir_entry in project_home_dir {
        let path = dir_entry.unwrap().path().to_owned();
        if path.is_dir() {
            println!("{}", path.as_path().file_name().unwrap().to_string_lossy());
        }
    }

    Ok(())
}

pub fn new(project_dir_path: PathBuf, args: cli::NewArgs) -> Result<()> {
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

pub fn clone(project_dir_path: PathBuf, args: cli::CloneArgs) -> Result<()> {
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

pub fn list_commands() -> Result<()> {
    for &c in cli::Commands::NAMES {
        println!("{c}");
    }
    Ok(())
}
