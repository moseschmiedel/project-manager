use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about=None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    Cd(CdArgs),
    ListProjects(ListProjectsArgs),
    Clone(CloneArgs),
    New(NewArgs),
    ListCommands,
    SupportedVersion(SupportedVersionArgs),
}

impl Commands {
    pub const NAMES: &'static [&'static str] = &[
        "cd",
        "list-projects",
        "clone",
        "new",
        "list-commands",
        "supported-version",
    ];
}

#[derive(Args)]
#[command(author, version, about="List all available projects", long_about=None)]
pub struct ListProjectsArgs {
    /// Directory where projects are stored
    #[arg(short, long = "projects-root")]
    pub project_dir_path: std::path::PathBuf,
}

#[derive(Args)]
#[command(author, version, about="Create new project", long_about=None)]
pub struct NewArgs {
    /// Name of new project
    pub project_name: String,

    /// Directory where projects are stored
    #[arg(short, long = "projects-root")]
    pub project_dir_path: std::path::PathBuf,

    /// Generator used for creating new project
    #[arg(short, long = "generator", default_value = "git")]
    pub generator: String,
}

#[derive(Args)]
#[command(author, version, about = "Change directory to specified project root", long_about = None)]
pub struct CdArgs {
    /// Project to switch to
    pub project_name: String,

    /// Directory where projects are stored
    #[arg(short, long = "projects-root")]
    pub project_dir_path: std::path::PathBuf,
}

#[derive(Args)]
#[command(author, version, about = "Clone project from specified git URL", long_about = None)]
pub struct CloneArgs {
    /// git URL to clone from
    pub url: String,

    /// Directory where projects are stored
    #[arg(short, long = "projects-root")]
    pub project_dir_path: std::path::PathBuf,

    /// Parent directory to clone project into
    #[arg(short, long = "project-name")]
    pub project_name: Option<String>,
    /// Parent directory to clone project into
    #[arg(short, long = "parent-dir")]
    pub directory: Option<std::path::PathBuf>,
}

#[derive(Args)]
#[command(author, version, about = "Check if specified semver string is supported", long_about = None)]
pub struct SupportedVersionArgs {
    /// Semver string to check
    pub version_requirement: String,
}
