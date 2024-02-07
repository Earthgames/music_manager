use log::error;
use std::env::current_dir;

use crate::{normalize, Result};

pub fn add_to_lib(files: &str, genre: &str, quiet: bool) -> Result<()> {
    let files = super::read_pattern(files)?;

    match normalize::normalize(&current_dir()?, &files, quiet) {
        Ok(_) => {}
        Err(err) => {
            error!("{}", err.to_string());
            println!("Could not normalize with loudgain")
        }
    };
    super::move_to_genre(genre, &files)
}
