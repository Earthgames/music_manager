use crate::{commands::add::add, config, Result};
use log::{error, info};
use std::{
    fs,
    io::ErrorKind,
    path::{Path, PathBuf},
    process::Command,
};

/// The download sub command
/// this will try to download with yt-dlp and normalize with loudgain
pub fn download(web_address: &str, category: &str, quiet: bool) -> Result<()> {
    // get user config directory
    let config = config::get_config()?;
    let music_dir = config.music_dir;

    let tmp_music_dir = music_dir.join("tmp/");

    // checks if de temporary directory exists, makes it if it does not
    if !Path::new(&tmp_music_dir).is_dir() {
        info!(
            "there is no temporary directory in \"{}\", trying to make it",
            &tmp_music_dir.display()
        );
        fs::create_dir(&tmp_music_dir)?
    }

    let tmp_dir_content = super::read_dir(&tmp_music_dir, None)?;

    // download from yt with yt-dlp
    //TODO use --print for yt-dlp and use that
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
            error!("Could not use yt-dlp command");
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
    let mut opus_files: Vec<&PathBuf> = Vec::new();
    let tmp_dir_content_after = super::read_dir(&tmp_music_dir, None)?;
    if !tmp_dir_content.is_empty() {
        for content in tmp_dir_content_after.iter() {
            if !tmp_dir_content.contains(content)
                && content.extension().unwrap_or_default() == "opus"
            {
                opus_files.push(content);
            }
        }
    } else {
        for content in tmp_dir_content_after.iter() {
            if content.extension().unwrap_or_default() == "opus" {
                opus_files.push(content)
            }
        }
    }

    add(
        &opus_files.iter().map(|x| x.display().to_string()).collect(),
        category,
        quiet,
        true,
    )
}
