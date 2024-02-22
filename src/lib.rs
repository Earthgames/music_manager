pub mod category_description;
pub mod commands;
pub mod config;
pub mod music_tag;
pub mod normalize;

use glob::glob;
use log::{info, warn};
use std::{
    fs::{self, File},
    io::ErrorKind,
    os::unix::fs::FileExt,
    path::Path,
};

/// Create a file with the given content
pub fn create_file(path: &Path, content: String) -> Result<()> {
    // create a file
    let description_file = File::create(path)?;

    // write to the file
    description_file.write_at(content.as_bytes(), 0)?;

    Ok(())
}

/// Gives a string with all the files in that are in a directory,
/// with an option to only include files with a certain file extension
pub fn read_dir(dir: &Path, file_ext: Option<&str>) -> Result<Vec<String>> {
    let search = match file_ext {
        Some(ext) => dir.join(format!("*{ext}")),
        None => dir.join("*"),
    };

    let dir = match search.to_str() {
        Some(dir) => dir,
        None => {
            return Err(Box::new(std::io::Error::new(
                ErrorKind::NotFound,
                "Could not find directory",
            )))
        }
    };
    read_pattern(dir)
}

/// Gives a string with all the files in that match a path pattern
pub fn read_pattern(pattern: &str) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
    for entry in match glob(pattern) {
        Ok(paths) => paths,
        Err(err) => return Err(Box::new(err)),
    } {
        match entry {
            Ok(entry) => result.push(entry.display().to_string()),
            Err(err) => return Err(Box::new(err)),
        }
    }
    Ok(result)
}

/// Searches a vector for the given content
fn search(query: &str, content: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Searches for a category, and returns the is the full category name
fn search_category(category: &str) -> Result<String> {
    // get config
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let category_type_dirs = read_dir(&music_dir, None)?;

    let category_dir = search(category, category_type_dirs);

    // Checking if the directory exists, otherwise it checks if the other directory,
    // if not it creates it
    if category_dir.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No directory found",
        )));
    }

    Ok(category_dir[0].to_string())
}

/// Move some files
pub fn move_files(
    target_files: &Vec<String>,
    target_dir: &str,
) -> Result<(), Box<dyn std::error::Error>> {
    for file in target_files {
        let file_name = match Path::new(&file).file_name() {
            Some(name) => match name.to_str() {
                Some(name) => name,
                None => continue,
            },
            None => continue,
        };
        fs::copy(file, format!("{target_dir}/{file_name}"))?;
        match fs::remove_file(file) {
            Ok(_) => info!("moved {file} to {target_dir}"),
            Err(e) => {
                warn!(
                    "copied {} to {}, could not remove the original",
                    file, target_dir
                );

                return Err(Box::new(e));
            }
        };
    }
    Ok(())
}

/// Shorthand for Result
pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
