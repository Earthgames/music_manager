pub mod commands;
pub mod config;
pub mod genre_description;
pub mod music_tag;

use std::fs::File;
use std::os::unix::fs::FileExt;
use std::path::Path;

pub fn create_file(path: &Path, content: String) -> Result<()> {
    // create a file
    let description_file = File::create(path)?;

    // write to the file
    description_file.write_at(content.as_bytes(), 0)?;

    Ok(())
}

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
