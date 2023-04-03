use std::{fs, str::pattern::Pattern};
use clap::Parser;

#[derive(Parser)]
struct Cli {
    project_name: String,
    #[arg(short, long="projects")]
    project_dir_path: std::path::PathBuf,
}

fn main() {
    let args = Cli::parse();
    let project_dir = fs::read_dir(args.project_dir_path).unwrap();

    for dir_entry in project_dir {
        let path = dir_entry.unwrap().path().to_owned();
        if path.is_dir() {
            let name = path.file_name().unwrap().to_str().unwrap().to_owned();
            if name == args.project_name {
                println!("{}", path.as_path().display());
            }
        }
    }
}
