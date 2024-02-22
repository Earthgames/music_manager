pub mod add;
pub mod category;
pub mod download;

use crate::{category_description, config, move_files, read_dir, search_category, Result};
use log::{error, info, warn};
use std::{fs, path::Path, process};

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

pub fn move_to_category(category: &str, files: &Vec<String>) -> Result<()> {
    let config = config::get_config()?;
    // search for dir so short names are possible. otherwise try to use the default directory
    let category_dir = match search_category(category) {
        Ok(dir) => Path::new(&dir).to_owned(),
        Err(_) => {
            // could this be different ??
            info!("category {category} not found");
            let default_dir = config.default_dir;
            if !Path::new(&default_dir).is_dir() {
                warn!("The default_dir is not in {}", default_dir.display());
                print!(
                    "The files where not moved because the category and default directory was not found"
                );
                return Ok(());
            }
            print!(
                "The files where moved to {} because the category was not found",
                default_dir.to_str().unwrap()
            );
            default_dir
        }
    };

    move_files(files, &category_dir)
}
