use std::{env::current_dir, io::Error, path::PathBuf};

use log::error;

use crate::{normalize, Result};

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
            error!("{} is not a file", file.display());
            return Err(Box::new(Error::new(
                std::io::ErrorKind::InvalidInput,
                "not a file",
            )));
        }
        match normalize::normalize(&current_dir()?, &file, quiet, force) {
            Ok(_) => {}
            Err(err) => {
                error!("Could not normalize with loudgain because {err}")
            }
        };
    }
    if *album {
        super::move_album_to_category(category, files, true)
    } else {
        super::move_to_category(category, files)
    }
}
