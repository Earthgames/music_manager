use std::path::PathBuf;
use std::process::{Command, Stdio};

use log::warn;

use crate::commands::add::add;
use crate::music_tag::get_music_tag;

use anyhow::{Context, Result};

pub fn tag(
    dir: PathBuf,
    files: &[String],
    category: &str,
    quiet: &bool,
    force: &bool,
) -> Result<()> {
    let mut tagged = vec![];
    let mut files = files.to_owned();
    // Retain files without tags
    files.retain(|file| {
        if !force && get_music_tag(file.as_ref()).is_ok() {
            warn!("\"{}\" already has music tags, skipping tagging", file);
            tagged.push(file.clone());
            false
        } else {
            true
        }
    });
    if !files.is_empty() {
        Command::new("picard")
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
            .arg("-s") // standalone instance of Picard
            .args(&files)
            .status()
            .context("Could not execute picard")?;
        let mut tagged_files: Vec<String> = files.clone();
        tagged_files.retain(|file| get_music_tag(file.as_ref()).is_ok());
        add(&tagged_files, category, quiet, force, &true)?;
    } else {
        // In main the empty files is checked
        warn!("All files where tagged")
    }
    Ok(())
}
