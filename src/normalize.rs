use crate::Result;
use log::error;
use std::{
    io::{Error, ErrorKind},
    path::Path,
    process::Command,
};

pub fn normalize(dir: &Path, files: &Vec<String>, quiet: bool) -> Result<()> {
    let normalizer = match Command::new("loudgain")
        .current_dir(dir)
        .args([
            "-r",
            match quiet {
                true => "-q",
                false => "",
            },
        ])
        .args(files)
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
    Ok(())
}
