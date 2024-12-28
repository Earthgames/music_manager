use std::{env::current_dir, path::PathBuf};

use anyhow::{anyhow, Result};
use log::error;

use crate::normalize::{self, normalize_files};

pub fn add(
    files: &Vec<String>,
    category: &str,
    quiet: &bool,
    force: &bool,
    album: &bool,
) -> Result<()> {
    for file in files {
        let file = PathBuf::from(file);
        if !file.is_file() {
            return Err(anyhow!("{} is not a file", file.display()));
        }
        if !album {
            match normalize::normalize(&current_dir()?, &file, quiet, force) {
                Ok(_) => {}
                Err(err) => {
                    error!("Could not normalize file because of: {err}")
                }
            };
        }
    }
    if *album {
        let paths: Vec<_> = files.iter().map(PathBuf::from).collect();
        normalize_files(
            &current_dir()?,
            &paths.iter().map(|path| path.as_path()).collect::<Vec<_>>(),
            quiet,
            force,
        )?;
        super::move_album_to_category(category, files, true)
    } else {
        super::move_to_category(category, files)
    }
}
