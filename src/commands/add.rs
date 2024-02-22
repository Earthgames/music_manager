use crate::{normalize, read_pattern, Result};
use log::error;
use std::{env::current_dir, path::PathBuf};

pub fn add(files: &str, category: &str, quiet: bool, force: bool) -> Result<()> {
    let files = read_pattern(files)?;

    add_to_lib(&files, category, quiet, force)
}

pub fn add_to_lib(files: &Vec<String>, category: &str, quiet: bool, force: bool) -> Result<()> {
    for file in files {
        match normalize::normalize(&current_dir()?, &PathBuf::from(file), quiet, force) {
            Ok(_) => {}
            Err(err) => {
                error!("Could not normalize with loudgain because {err}")
            }
        };
    }
    super::move_to_category(category, files)
}
