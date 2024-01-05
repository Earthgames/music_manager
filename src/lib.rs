use colored::Colorize;
use glob::glob;
use log::{error, info, warn};
use std::fs;
use std::io::ErrorKind;
use std::path::Path;
use std::process;
use std::process::Command;
pub mod config;
pub mod genre_description;
pub mod music_tag;

// gives a string with all the files in that match a path pattern
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

fn search(query: &str, content: Vec<String>) -> Vec<String> {
    let mut results: Vec<String> = Vec::new();

    for line in content {
        if line.contains(query) {
            results.push(line);
        }
    }

    results
}

// will be removed?
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
        match fs::remove_file(file) {
            Ok(_) => {}
            Err(_) => {}
        };
    }
}

pub fn download(webadress: &String, genre_type: &str) -> Result<()> {
    // get user config directory
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let tmp_music_dir = music_dir.join("tmp/");

    // checks if de temporary directory exists, makes it if it does not
    if !Path::new(&tmp_music_dir).is_dir() {
        info!(
            "there is no temporary directory in {}, trying to make it",
            &tmp_music_dir.display()
        );
        fs::create_dir(&tmp_music_dir)?
    }

    let tmp_dir_content = read_dir(&tmp_music_dir, None)?;

    // download from yt with yt-dlp
    let downloader = match Command::new("yt-dlp")
        .args([
            "--extract-audio",
            "-f",
            "bestaudio",
            "--audio-format",
            "opus",
            "--split-chapters",
        ])
        .arg(webadress)
        .current_dir(&tmp_music_dir)
        .status()
    {
        Ok(e) => e,
        Err(err) => {
            error!("Could not use yt-dlp command \n is it installed?");
            return Err(err.into());
        }
    };

    if !downloader.success() {
        error!("yt-dlp {}", downloader);
        return Err(Box::new(std::io::Error::new(
            ErrorKind::Other,
            "Failed to download with yt-dlp",
        )));
    };

    // creates a vector with only the newly created opus files
    let mut opus_files: Vec<String> = Vec::new();
    let tmp_dir_content_after = read_dir(&tmp_music_dir, None)?;
    if !tmp_dir_content.is_empty() {
        for content in tmp_dir_content_after.iter() {
            if !tmp_dir_content.contains(content) && content.ends_with(".opus") {
                opus_files.push(content.to_string());
            }
        }
    } else {
        for content in tmp_dir_content_after.iter() {
            if content.ends_with(".opus") {
                opus_files.push(content.to_string())
            }
        }
    }

    // normalize opus files
    let normalizer = match Command::new("loudgain")
        .current_dir(&tmp_music_dir)
        .arg("-r")
        .args(&opus_files)
        .status()
    {
        Ok(e) => e,
        Err(err) => {
            error!("Could not use loudgain command \n is it installed?");
            return Err(err.into());
        }
    };

    if !normalizer.success() {
        error!(
            "loudgain {}\nFailed to normalize audio with loudgain, please do it yourself",
            normalizer
        );
    };

    // search for dir so short names are possible. otherwise try to use the other directory
    let genre_dir = match search_genre(genre_type.to_string()) {
        Ok(dir) => Path::new(&dir).to_owned(),
        Err(_) => {
            // could this be different ??
            info!("genre_type not found");
            let default_dir = config.default_dir;
            if !Path::new(&default_dir).is_dir() {
                warn!("The default_dir is not in {}", default_dir.display());
                return Ok(());
            }
            default_dir
        }
    };

    move_files(opus_files, genre_dir.to_str().unwrap())?;
    Ok(())
}

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

// print details about genres
pub fn genres(genre: &Option<String>) -> Result<()> {
    if let Some(genre) = genre {
        let genre_path = match search_genre(genre.clone()) {
            Ok(path) => Path::new(&path).to_owned(),
            Err(_) => {
                warn!("Could not find genre/type, don't use any arguments to print all genres");
                return Ok(());
            }
        };

        let (name, description) = genre_description::get_genre_description(genre_path.as_path())?;

        let music_files = read_dir(genre_path.as_path(), Some(".mp3"))?;
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
        let genre_dirs = read_dir(music_dir.as_path(), None)?;

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

use std::fs::File;
use std::os::unix::fs::FileExt;

pub fn create_file(path: &Path, content: String) -> Result<()> {
    // create a file
    let description_file = File::create(path)?;

    // write to the file
    description_file.write_at(content.as_bytes(), 0)?;

    Ok(())
}

pub type Result<T, E = Box<dyn std::error::Error>> = std::result::Result<T, E>;
