use crate::{music_tag::has_replaygain_tags, Result};
use log::{error, info};
use std::{
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};

pub fn normalize(dir: &Path, file: &Path, quiet: bool, force: bool) -> Result<()> {
    if !force && has_replaygain_tags(file)? {
        info!("{} already has replaygain tags, skipping normalizing", file.display());
        return Ok(());
    }

    let normalizer = match Command::new("loudgain")
        .current_dir(dir)
        .arg(match quiet {
            true => "-rq",
            false => "-r",
        })
        .arg(file)
        .status()
    {
        Ok(e) => e,
        Err(err) => {
            error!("Could not execute loudgain");
            return Err(err.into());
        }
    };

    if !normalizer.success() {
        error!(
            "loudgain {}\nFailed to normalize audio with loudgain",
            normalizer
        );
        return Err(Box::new(Error::new(
            ErrorKind::Other,
            format!(
                "Loudgain exited with unsuccessfully with code {}",
                normalizer
            ),
        )));
    };
    info!("Normalized {}", file.display());

    Ok(())
}
