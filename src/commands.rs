use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

use log::{error, info, warn};

use crate::{
    category::get_category_config, config, move_file, music_tag::get_music_tag, read_dir,
    Result, search,
};

pub mod add;
pub mod cat;
pub mod down;
pub mod check;

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
    let config = config::get_config()?;
    // search for dir so short names are possible. otherwise try to use the default directory
    let category_dir = match find_category(category) {
        Ok(dir) => dir,
        Err(_) => {
            warn!("category {category} not found");

            // try moving to the default directory
            let default_dir = config.default_dir;

            if !Path::new(&default_dir).is_dir() {
                warn!("The default_dir is not in {}", default_dir.display());

                return Err(Box::new(std::io::Error::new(
                    ErrorKind::NotFound,
                    "Could not find the category and default directory",
                )));
            }

            info!(
                "The files where moved to \"{}\" because the category was not found",
                default_dir.to_str().unwrap()
            );
            default_dir
        }
    };

    let category_config = get_category_config(&category_dir)?;

    for file in files {
        let file = PathBuf::from(file);

        // get music tags
        let music_tag = match get_music_tag(&file) {
            Ok(tag) => tag,
            Err(err) => {
                warn!("could not find music tag because {err}");

                // try the untagged directory
                let untagged_dir = category_dir.join("Untagged");
                if untagged_dir.is_dir() {
                    match move_file(&file, &untagged_dir) {
                        Ok(_) => info!(
                            "moved \"{}\" to Untagged directory in \"{}\"",
                            file.display(),
                            category_dir.display()
                        ),

                        // if we can't move a file we won't try the rest
                        Err(err) => {
                            error!(
                                "could not move \"{}\" to Untagged directory because error: {err}",
                                file.display()
                            );
                            return Err(err);
                        }
                    };
                }
                continue;
            }
        };

        let album_dir;

        // check if the category is one artist only
        if category_config.artist_category.unwrap_or(false) {
            album_dir = category_dir.join(change_forbidden_chars(&music_tag.album_title));
            if !album_dir.is_dir() {
                fs::create_dir(&album_dir)?;
                info!("Created \"{}\" album directory", album_dir.display());
            }
        } else {
            // create artist and album directories if they do not exist
            let artist_dir = category_dir.join(change_forbidden_chars(&music_tag.album_artist));
            if !artist_dir.is_dir() {
                // if we can't create a directory we won't try the rest
                fs::create_dir(&artist_dir)?;
                info!("Created \"{}\" artist directory", artist_dir.display());
            }

            album_dir = artist_dir.join(change_forbidden_chars(&music_tag.album_title));
            if !album_dir.is_dir() {
                fs::create_dir(&album_dir)?;
                info!("Created \"{}\" album directory", album_dir.display());
            }
        }
        move_file(&file, &album_dir)?;
        info!(
            "Moved \"{}\" to \"{}\"",
            file.display(),
            album_dir.display()
        );
    }

    Ok(())
}

fn change_forbidden_chars(input: &str) -> String {
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
            ':' => '：',
            _ => char,
        };
        output.push(char);
    }
    output
}
