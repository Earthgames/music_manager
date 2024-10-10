use std::process::Stdio;
use std::{
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};

use log::{error, info};

use crate::{music_tag::file_has_replaygain_tags, Result};

pub fn normalize(dir: &Path, file: &Path, quiet: &bool, force: &bool) -> Result<()> {
    normalize_files(dir, &[file], quiet, force)
}

pub fn normalize_files(dir: &Path, files: &[&Path], quiet: &bool, force: &bool) -> Result<()> {
    if !force
        && !files
            .iter()
            .any(|f| file_has_replaygain_tags(f).unwrap_or(false))
    {
        info!(
            "{} already has replaygain tags, skipping normalizing",
            files
                .iter()
                .map(|file| format!("\"{}\"", file.display()))
                .collect::<Vec<String>>()
                .join(", ")
        );
        return Ok(());
    }

    let normalizer = match Command::new("rsgain")
        .current_dir(dir)
        .stdout(if *quiet {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .stderr(if *quiet {
            Stdio::null()
        } else {
            Stdio::inherit()
        })
        .arg("custom")
        .arg(match quiet {
            true => "-aq",
            false => "-a",
        }) // album mode, and quiet if needed
        .arg(match force {
            true => "",
            false => "-S",
        }) // Skip existing tags
        .args(["-s", "i"]) // output mode i =  write replaygain2.0 tags plus extra tags
        .args(files)
        .status()
    {
        Ok(e) => e,
        Err(err) => {
            error!("Could not execute rsgain");
            return Err(err.into());
        }
    };

    if !normalizer.success() {
        error!(
            "rsgain {}\nFailed to normalize audio with rsgain",
            normalizer
        );
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            format!("Rsgain exited with unsuccessfully with code {}", normalizer),
        )));
    };
    info!(
        "Normalized: {}",
        files
            .iter()
            .map(|file| format!("\"{}\"", file.display()))
            .collect::<Vec<String>>()
            .join(", ")
    );

    Ok(())
}
