use crate::{config, genre_description, Result};
use glob::glob;
use log::{error, info, warn};
use std::{fs, io::ErrorKind, path::Path, process};
pub mod download;
pub mod genre;

/// Gives a string with all the files in that match a path pattern
pub fn read_dir(dir: &Path, file_ext: Option<&str>) -> Result<Vec<String>> {
    let mut result: Vec<String> = Vec::new();
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
    for entry in match glob(dir) {
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

// TODO: remove
pub fn clean_tmp() {
    let music_dir = match config::get_config() {
        Ok(dirs) => dirs,
        Err(err) => {
            error!("{err}");
            process::exit(1);
        }
    }
    .music_dir;

    let tmp_music_dir = music_dir.join("tmp/*");

    let tmp_dir_content = match read_dir(&tmp_music_dir, None) {
        Ok(dirs) => dirs,
        Err(err) => {
            error!("{err}");
            process::exit(1);
        }
    };

    for file in tmp_dir_content {
        match fs::remove_file(&file) {
            Ok(_) => (),
            Err(err) => {
                error!("Could not remove file {file}\n{err}")
            }
        };
    }
}

/// searches a vector for some content
fn search(query: &str, content: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

/// Searches for a genre, and returns the is the full genre name
fn search_genre(genre: String) -> Result<String> {
    // get config
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let genre_type_dirs = read_dir(&music_dir, None)?;

    let genre_dir = search(&genre, genre_type_dirs);

    // Checking if the directory exists, otherwise it checks if the other directory,
    // if not it creates it
    if genre_dir.is_empty() {
        return Err(Box::new(std::io::Error::new(
            std::io::ErrorKind::NotFound,
            "No directory found",
        )));
    }

    Ok(genre_dir[0].to_string())
}

/// Move some files
pub fn move_files(
    target_files: Vec<String>,
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
        fs::copy(&file, format!("{target_dir}/{file_name}"))?;
        match fs::remove_file(&file) {
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
