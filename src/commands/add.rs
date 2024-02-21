use log::error;
use std::env::current_dir;

use crate::{normalize, Result};

pub fn add(files: &str, category: &str, quiet: bool, force: bool) -> Result<()> {
    let files = super::read_pattern(files)?;

    add_to_lib(&files, category, quiet, force)
}

pub fn add_to_lib(files: &Vec<String>, category: &str, quiet: bool, force: bool) -> Result<()> {
    match normalize::normalize(&current_dir()?, files, quiet, force) {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err.to_string());
            println!("Could not normalize with loudgain")
        }
    };
    super::move_to_category(category, files)
}
