pub mod add;
pub mod category;
pub mod download;

use crate::{
    category_config, config, move_file, music_tag::get_music_tag, read_dir, search, Result,
};
use log::{error, info, warn};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process,
};

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

/// Move a files to a category
pub fn move_to_category(category: &str, files: &Vec<String>) -> Result<()> {
    let config = config::get_config()?;
    // search for dir so short names are possible. otherwise try to use the default directory
    let category_dir = match search_category(category) {
        Ok(dir) => Path::new(&dir).to_owned(),
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
                "The files where moved to {} because the category was not found",
                default_dir.to_str().unwrap()
            );
            default_dir
        }
    };

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
                            "moved {} to Untagged directory in {}",
                            file.display(),
                            category_dir.display()
                        ),

                        // if we can't move a file we won't try the rest
                        Err(err) => {
                            error!(
                                "could not move {} to Untagged directory because {err}",
                                file.display()
                            );
                            return Err(err);
                        }
                    };
                }
                continue;
            }
        };

        // create artist and album directories if they do not exist
        let artist_dir = category_dir.join(change_forbidden_chars(&music_tag.artist_name));
        if !artist_dir.is_dir() {
            //if we can't create a directory we won't try the rest
            fs::create_dir(&artist_dir)?;
            info!("Created {} artist directory", artist_dir.display());
        }
        let album_dir = artist_dir.join(change_forbidden_chars(&music_tag.album_title));
        if !album_dir.is_dir() {
            fs::create_dir(&album_dir)?;
            info!("Created {} album directory", album_dir.display());
        }

        move_file(&file, &album_dir)?;
        info!("Moved {} to {}", file.display(), album_dir.display());
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
