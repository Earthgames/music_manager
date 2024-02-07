use crate::{config, normalize, Result};
use log::{error, info, warn};
use std::{fs, io::ErrorKind, path::Path, process::Command};

/// The download sub command
/// this will try to download with yt-dlp and normalize with loudgain
pub fn download(web_address: &str, genre_type: &str, quiet: bool) -> Result<()> {
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

    let tmp_dir_content = super::read_dir(&tmp_music_dir, None)?;

    // download from yt with yt-dlp
    let downloader = match Command::new("yt-dlp")
        .args([
            "--extract-audio",
            "-f",
            "bestaudio",
            "--audio-format",
            "opus",
            "--split-chapters",
            match quiet {
                true => "-q",
                false => "--no-quiet",
            },
        ])
        .arg(web_address)
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
    let tmp_dir_content_after = super::read_dir(&tmp_music_dir, None)?;
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

    match normalize::normalize(&tmp_music_dir, &opus_files) {
        Ok(_) => {}
        Err(err) => {
            error!("{}\n", err.to_string());
            print!("Could not normalize with loudgain")
        }
    };

    // search for dir so short names are possible. otherwise try to use the other directory
    let genre_dir = match super::search_genre(genre_type.to_string()) {
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

    super::move_files(opus_files, genre_dir.to_str().unwrap())?;
    Ok(())
}
