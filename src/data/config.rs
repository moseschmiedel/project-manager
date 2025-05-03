use crate::error::Error;
use anyhow::{anyhow, Context, Result};
use std::{ffi::OsString, fs, io, path::PathBuf};

const CONFIG_NAME: &str = "project-manager";

pub fn try_init_config_dir() -> Result<()> {
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
        .or(std::env::var_os("HOME")
            .and_then(empty_os_string_to_none)
            .map(|path| path.join(".config")))
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
