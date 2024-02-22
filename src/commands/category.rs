use crate::{
    category_description, config,
    music_tag::{get_music_tag, MusicTag},
    read_dir, Result,
};
use colored::Colorize;
use log::{error, warn};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
};

/// Print details about categories
pub fn category(category: &Option<String>) -> Result<()> {
    if let Some(category) = category {
        let category_path = match super::search_category(category) {
            Ok(path) => Path::new(&path).to_owned(),
            Err(_) => {
                warn!(
                    "Could not find category/type, don't use any arguments to print all categories"
                );
                return Ok(());
            }
        };

        let (name, description) =
            super::category_description::get_category_description(category_path.as_path())?;

        let music_files = read_dir(category_path.as_path(), Some(".opus"))?;

        let mut music_tags: Vec<MusicTag> = vec![];
        for file in music_files {
            music_tags.push(get_music_tag(&PathBuf::from(file))?)
        }

        let big_tags: bool = if music_tags.len() > 15 {
            music_tags.sort_by(|a, b| a.album_title.cmp(&b.album_title));
            music_tags.dedup_by(|a, b| a.album_title == b.album_title);
            music_tags.sort_by(|a, b| a.artist_name.cmp(&b.artist_name));
            true
        } else {
            music_tags.sort_by(|a, b| a.album_title.cmp(&b.album_title));
            music_tags.sort_by(|a, b| a.artist_name.cmp(&b.artist_name));
            false
        };

        println!("{}: {}", "Name".bold().purple(), name.bold());
        println!("{}: {}", "Description".bold().blue(), description);
        println!();

        if music_tags.is_empty() {
            println!("{}", "No music found".red().bold());
            return Ok(());
        }

        for music_tag in music_tags {
            println!("{}: {}", "Artist".bold().purple(), music_tag.artist_name);
            println!("{}: {}", "Album".bold().blue(), music_tag.album_title);
            if !big_tags {
                println!("{}: {}", "Song".bold().green(), music_tag.song_title)
            }
            println!();
        }
        Ok(())
    } else {
        // print all categories and their description
        let music_dir = config::get_config()?.music_dir;
        let category_dirs = read_dir(music_dir.as_path(), None)?;

        for category_dir in category_dirs {
            let (name, description) = match category_description::get_category_description(
                Path::new(&category_dir),
            ) {
                Ok(cont) => cont,
                Err(err) => {
                    match err.kind() {
                        ErrorKind::NotFound => {
                            warn!("Could not find description for folder {category_dir}, skipping")
                        }
                        _ => error!("skipping {category_dir} because of error: {err}"),
                    }
                    continue;
                }
            };
            println!("{}: {}", "Name".bold().purple(), name.bold());
            println!("{}: {}", "Description".bold().blue(), description);
            println!();
        }
        Ok(())
    }
}

pub fn mk_category(category_name: &String, category_description: &String) -> Result<()> {
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let category_dir = music_dir.join(category_name);
    // checks if the category directory already exists, makes it if it does not
    if !category_dir.is_dir() {
        fs::create_dir(&category_dir)?
    }
    let untagged_dir = music_dir.join("Untagged");
    if !untagged_dir.is_dir() {
        fs::create_dir(&untagged_dir)?
    }

    category_description::create_category_description(
        &category_dir,
        Some(category_name),
        Some(category_description),
    )?;
    println!("Made description file for category {}", &category_name);

    Ok(())
}
