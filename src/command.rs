use anyhow::{Context, Result};
use std::{fs, path::PathBuf, process};

use crate::cli;
use crate::project;

fn parse_project_dir_path(project_dir_path: PathBuf) -> Result<PathBuf> {
    Ok(project_dir_path.canonicalize().with_context(|| {
        format!(
            "Cannot canonicalize project directory path: {}",
            project_dir_path.display()
        )
    })?)
}

pub fn cd(args: cli::CdArgs) -> Result<()> {
    let root = project::Detector::new(args.project_dir_path).detect();

    let projects = root.build_project_slugs();

    for project in projects {
        if project.to_string() == args.project_name {
            println!("{}", project.fmt_path());
        }
    }

    Ok(())
}

pub fn list_projects(args: cli::ListProjectsArgs) -> Result<()> {
    let root = project::Detector::new(args.project_dir_path).detect();

    let projects = root.build_project_slugs();

    for project in projects {
        println!("{}", project);
    }

    Ok(())
}

pub fn new(args: cli::NewArgs) -> Result<()> {
    let project_dir = parse_project_dir_path(args.project_dir_path)?.join(&args.project_name);
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

pub fn clone(args: cli::CloneArgs) -> Result<()> {
    let project_dir_path = parse_project_dir_path(args.project_dir_path)?;

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

pub fn supported_version(args: cli::SupportedVersionArgs) -> Result<()> {
    let version_req = semver::VersionReq::parse(&args.version_requirement).with_context(|| {
        format!(
            "Could not parse version requirement '{}'",
            args.version_requirement
        )
    })?;
    let version_str = env!("CARGO_PKG_VERSION");
    if !version_req.matches(&semver::Version::parse(version_str)?) {
        Err(anyhow::anyhow!(
            "Version requirement '{}' is not satisfied by current version '{}'",
            args.version_requirement,
            version_str
        ))?;
    }
    Ok(())
}
