use crate::Result;
use lofty::{Accessor, ItemKey, Probe, Tag, TagExt, TaggedFileExt};
use log::error;
use std::{io::Error, path::Path};

/// Type to store music albums and songs
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct MusicTag {
    pub artist_name: String,
    pub song_title: String,
    pub album_title: String,
}

/// Get a music tag form a file
pub fn get_music_tag(music_file: &Path) -> Result<MusicTag> {
    let music_tag;
    let tag = get_tag(music_file)?;

    if let Some(title) = tag.title() {
        if let Some(artist) = tag.artist() {
            if let Some(album) = tag.album() {
                music_tag = MusicTag {
                    song_title: title.to_string(),
                    artist_name: artist.to_string(),
                    album_title: album.to_string(),
                };
            } else {
                error!(
                    "{} is skipped because it has no album tag",
                    music_file.display()
                );
                return Err(Box::new(Error::new(
                    std::io::ErrorKind::NotFound,
                    "could not find album tag",
                )));
            }
        } else {
            error!(
                "{} is skipped because it has no artist tag",
                music_file.display()
            );
            return Err(Box::new(Error::new(
                std::io::ErrorKind::NotFound,
                "could not find artist tag",
            )));
        }
    } else {
        error!(
            "{} is skipped because it has no title tag",
            music_file.display()
        );
        return Err(Box::new(Error::new(
            std::io::ErrorKind::NotFound,
            "could not find title tag",
        )));
    }
    Ok(music_tag)
}

/// Check if a music has replaygain tags
pub fn has_replaygain_tags(music_file: &Path) -> Result<bool> {
    let tag = get_tag(music_file)?;
    Ok(tag.contains(&ItemKey::ReplayGainTrackPeak) && tag.contains(&ItemKey::ReplayGainTrackGain))
}

fn get_tag(music_file: &Path) -> Result<Tag> {
    let tagged_file = Probe::open(music_file)?.read()?;
    let tag = match tagged_file.primary_tag() {
        Some(tag) => tag,
        None => match tagged_file.first_tag() {
            Some(tag) => tag,
            None => {
                return Err(Box::new(std::io::Error::new(
                    std::io::ErrorKind::NotFound,
                    "No Tag found",
                )))
            }
        },
    };
    Ok(tag.clone())
}
