use crate::{normalize::normalize, Result};
use log::{error, info};
use std::{io::ErrorKind, path::PathBuf, process::Command};

use super::find_category;

/// The download sub command
/// this will try to download with yt-dlp and normalize with loudgain
pub fn download(web_address: &str, category: &str, quiet: bool) -> Result<()> {
    // get directory
    let category_dir = find_category(category)?.join("Untagged");

    let category_dir_content = super::read_dir(&category_dir, None)?;

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
        .current_dir(&category_dir)
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
    let category_dir_content_after = super::read_dir(&category_dir, None)?;
    if !category_dir_content.is_empty() {
        for content in category_dir_content_after.iter() {
            if !category_dir_content.contains(content)
                && content.extension().unwrap_or_default() == "opus"
            {
                opus_files.push(content);
            }
        }
    } else {
        for content in category_dir_content_after.iter() {
            if content.extension().unwrap_or_default() == "opus" {
                opus_files.push(content)
            }
        }
    }

    info!("Normalizing files");

    for file in opus_files {
        normalize(&category_dir, file, quiet, true)?;
    }
    Ok(())
}
