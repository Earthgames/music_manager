use crate::{config, genre_description, music_tag, Result};
use colored::Colorize;
use log::error;
use log::info;
use log::warn;
use std::fs;
use std::io::ErrorKind;
use std::path::Path;

/// Print details about genres
pub fn genres(genre: &Option<String>) -> Result<()> {
    if let Some(genre) = genre {
        let genre_path = match super::search_genre(genre.clone()) {
            Ok(path) => Path::new(&path).to_owned(),
            Err(_) => {
                warn!("Could not find genre/type, don't use any arguments to print all genres");
                return Ok(());
            }
        };

        let (name, description) =
            super::genre_description::get_genre_description(genre_path.as_path())?;

        let music_files = super::read_dir(genre_path.as_path(), Some(".mp3"))?;
        let mut music_tags: Vec<music_tag::MusicTag> = music_tag::get_music_tags(music_files)?;

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
        // print all genres and their description
        let music_dir = config::get_config()?.music_dir;
        let genre_dirs = super::read_dir(music_dir.as_path(), None)?;

        for genre_dir in genre_dirs {
            let (name, description) =
                match genre_description::get_genre_description(Path::new(&genre_dir)) {
                    Ok(cont) => cont,
                    Err(err) => {
                        match err.kind() {
                            ErrorKind::NotFound => {
                                warn!("Could not find description for folder {genre_dir}, skipping")
                            }
                            _ => error!("skipping {genre_dir} because of error: {err}"),
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

pub fn create_genre(genre_name: &String, genre_description: &String) -> Result<()> {
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let genre_dir = music_dir.join(genre_name);

    // checks if de genre directory already exists, makes it if it does not
    if !genre_dir.is_dir() {
        match fs::create_dir(&genre_dir) {
            Ok(_t) => info!("made genre directory {}", &genre_dir.display()),

            Err(err) => {
                error!("Could not make genre directory");
                return Err(err.into());
            }
        }
    }

    genre_description::create_genre_description(
        &genre_dir,
        Some(genre_name),
        Some(genre_description),
    )?;
    println!("made description file for genre {}", &genre_name);

    Ok(())
}
