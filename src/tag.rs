use std::path::PathBuf;
use std::process::{Command, Stdio};

use log::{error, info, warn};

use crate::commands::add::add;
use crate::music_tag::get_music_tag;
use crate::Result;

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
        match Command::new("picard")
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
            .arg("-s") // standalone instance of picard
            .args(&files)
            .status()
        {
            Ok(_) => (),
            Err(err) => {
                error!("Could not execute picard because of {}", err);
            }
        };
        let mut tagged_files: Vec<String> = files.clone();
        tagged_files.retain(|file| get_music_tag(file.as_ref()).is_ok());
        add(&tagged_files, category, quiet, force, &true)?;
    } else {
        // In main the empty files is checked
        warn!("All files where tagged")
    }
    Ok(())
}
