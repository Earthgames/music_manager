use std::process::Stdio;
use std::{
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};

use log::{error, info};

use crate::{music_tag::file_has_replaygain_tags, Result};

pub fn normalize(dir: &Path, file: &Path, quiet: &bool, force: &bool) -> Result<()> {
    if !force && file_has_replaygain_tags(file)? {
        info!(
            "\"{}\" already has replaygain tags, skipping normalizing",
            file.display()
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
        .args(["-s", "i"]) // output mode i =  write replaygain2.0 tags plus extra tags
        .arg(file)
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
    info!("Normalized \"{}\"", file.display());

    Ok(())
}
