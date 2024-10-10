use std::collections::HashMap;
use std::io::Error;
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use glob::Pattern;
use log::{error, info, warn};

use crate::category::CategoryConfig;
use crate::{
    category::get_category_config, config, move_file, move_files, music_tag::get_music_tag,
    read_dir, read_pattern, search, Result,
};

pub mod add;
pub mod cat;
pub mod check;
pub mod down;

/// Searches for a category, and returns the full category name
fn find_category(category: &str) -> Result<PathBuf> {
    // get config
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let mut category_type_dirs = read_dir(&music_dir, None)?;
    category_type_dirs.retain(|x| x.is_dir());

    let category_names = search(
        category,
        category_type_dirs
            .iter()
            .map(|x| x.file_name().unwrap().to_string_lossy().to_string()) // Nightmare code
            .collect(),
    );

    // check if we found anything
    if category_names.is_empty() {
        return Err(Box::new(std::io::Error::new(
            ErrorKind::NotFound,
            "No directory found",
        )));
    }

    if category_names.len() > 1 {
        info!("Found multiple categories that match search term")
    }

    let category_dir = music_dir.join(&category_names[0]);

    Ok(category_dir)
}

/// Move a files to a category
pub fn move_to_category(category: &str, files: &Vec<String>) -> Result<()> {
    let (category_dir, category_config) = move_setup(category)?;

    for file in files {
        let file = PathBuf::from(file);

        let album_dir = get_album_dir(&file, &category_dir, &category_config)?;

        move_file(&file, &album_dir)?;
        info!(
            "Moved \"{}\" to \"{}\"",
            file.display(),
            album_dir.display()
        );
    }

    Ok(())
}

/// Move folder per album to a category
pub fn move_album_to_category(category: &str, files: &Vec<String>, cover: bool) -> Result<()> {
    let (category_dir, category_config) = move_setup(category)?;

    let folder_item: HashMap<&Path, PathBuf> = HashMap::new();

    for file in files {
        let file = PathBuf::from(file);
        let parent = file.parent().unwrap();
        let album_dir = match folder_item.get(parent) {
            Some(a) => a.to_owned(),
            None => {
                let album_dir = get_album_dir(&file, &category_dir, &category_config)?;
                if cover {
                    let covers = read_pattern(
                        &Pattern::escape(parent.join("cover.*").to_str().unwrap()),
                        false,
                    )?;
                    move_files(&covers, &album_dir)?;
                    for cover in covers {
                        info!(
                            "Moved \"{}\" to \"{}\"",
                            cover.display(),
                            album_dir.display()
                        );
                    }
                }
                album_dir
            }
        };

        move_file(&file, &album_dir)?;
        info!(
            "Moved \"{}\" to \"{}\"",
            file.display(),
            album_dir.display()
        );
    }

    Ok(())
}

fn get_album_dir(
    file: &Path,
    category_dir: &Path,
    category_config: &CategoryConfig,
) -> Result<PathBuf> {
    // get music tags
    let music_tag = match get_music_tag(file) {
        Ok(tag) => tag,
        Err(err) => {
            warn!("could not find music tag because {err}");

            // try the untagged directory
            let untagged_dir = category_dir.join("Untagged");
            if untagged_dir.is_dir() {
                return Ok(untagged_dir);
            }
            return Err(Box::new(Error::new(
                ErrorKind::NotFound,
                "Could not find music tags and untagged directory",
            )));
        }
    };

    let album_dir;

    // check if the category is one artist only
    if category_config.artist_category.unwrap_or(false) {
        album_dir = category_dir.join(change_forbidden_chars(&music_tag.album_title));
        if !album_dir.is_dir() {
            // if we can't create a directory, we won't try the rest
            fs::create_dir(&album_dir)?;
            info!("Created \"{}\" album directory", album_dir.display());
        }
    } else {
        // create the artist and album directories if they do not exist
        let artist_dir = category_dir.join(change_forbidden_chars(&music_tag.album_artist));
        if !artist_dir.is_dir() {
            fs::create_dir(&artist_dir)?;
            info!("Created \"{}\" artist directory", artist_dir.display());
        }

        album_dir = artist_dir.join(change_forbidden_chars(&music_tag.album_title));
        if !album_dir.is_dir() {
            fs::create_dir(&album_dir)?;
            info!("Created \"{}\" album directory", album_dir.display());
        }
    }
    Ok(album_dir)
}

fn move_setup(category: &str) -> Result<(PathBuf, CategoryConfig)> {
    let config = config::get_config()?;
    // search for the directory, so short names are possible,
    // otherwise try to use the default directory
    let category_dir = match find_category(category) {
        Ok(dir) => dir,
        Err(_) => {
            error!("category {category} not found");

            // try moving to the default directory
            let default_dir = config.default_dir;

            if !Path::new(&default_dir).is_dir() {
                warn!("The default_dir is not in {}", default_dir.display());

                return Err(Box::new(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Could not find the category and default directory",
                )));
            }

            warn!(
                "The files where moved to \"{}\" because the category was not found",
                default_dir.to_str().unwrap()
            );
            default_dir
        }
    };
    let category_config = get_category_config(&category_dir)?;

    Ok((category_dir, category_config))
}

pub fn change_forbidden_chars(input: &str) -> String {
    let mut output = String::new();
    for char in input.chars() {
        // replace chars with chars that look like it
        let char = match char {
            '/' => '⁄',
            '>' => '＞',
            '<' => '＜',
            '"' => {
                output.push('\'');
                '\''
            } // we replace " with ''
            '\\' => '＼',
            '|' => '｜',
            '?' => '？',
            '*' => '＊',
            ':' => '꞉',
            _ => char,
        };
        output.push(char);
    }
    output
}
