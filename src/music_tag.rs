use std::{io::Error, path::Path};

use lofty::{Accessor, ItemKey, Probe, Tag, TagExt, TaggedFileExt};
use log::error;

use crate::Result;

/// Type to store music albums and songs
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct MusicTag {
    pub song_title: String,
    pub artist_name: String,
    pub album_title: String,
    pub album_artist: String,
    pub replaygain: bool,
}

/// Get a music tag form a file
pub fn get_music_tag(music_file: &Path) -> Result<MusicTag> {
    let music_tag;
    let tag = get_tag(music_file)?;

    // Song title
    let Some(title) = tag.title() else {
        error!(
            "\"{}\" is skipped because it has no title tag",
            music_file.display()
        );
        return Err(error("could not find title tag"));
    };

    // artist name
    let Some(artist) = tag.artist() else {
        error!(
            "{} is skipped because it has no artist tag",
            music_file.display()
        );
        return Err(error("could not find artist tag"));
    };

    // album title
    let Some(album) = tag.album() else {
        error!(
            "{} is skipped because it has no album tag",
            music_file.display()
        );
        return Err(error("could not find album tag"));
    };

    // album artist
    let Some(album_artist) = tag.get(&ItemKey::AlbumArtist) else {
        error!(
            "{} is skipped because it has no album artist tag",
            music_file.display()
        );
        return Err(error("could not find album artist tag"));
    };
    
    

    {
        music_tag = MusicTag {
            song_title: title.to_string(),
            album_artist: album_artist
                .clone()
                .into_value()
                .into_string()
                .unwrap_or_default(),
            album_title: album.to_string(),
            artist_name: artist.to_string(),
            replaygain: tag_has_replaygain_tags(tag)?,
        };
    }

    Ok(music_tag)
}

/// Check if a music file has replaygain tags
pub fn file_has_replaygain_tags(music_file: &Path) -> Result<bool> {
    let tag = get_tag(music_file)?;
    tag_has_replaygain_tags(tag)
}

pub fn tag_has_replaygain_tags(tag: Tag) -> Result<bool> {
    let result = tag.contains(&ItemKey::ReplayGainTrackGain)
        || tag.contains(&ItemKey::from_key(
        lofty::TagType::VorbisComments,
        "R128_TRACK_GAIN", // for opus and ogg types
    ));
    Ok(result)
}

fn get_tag(music_file: &Path) -> Result<Tag> {
    let tagged_file = Probe::open(music_file)?.read()?;
    let tag = match tagged_file.primary_tag() {
        Some(tag) => tag,
        None => match tagged_file.first_tag() {
            Some(tag) => tag,
            None => return Err(error("No tag found")),
        },
    };
    Ok(tag.clone())
}

fn error(error: &str) -> Box<Error> {
    Box::new(Error::new(std::io::ErrorKind::NotFound, error))
}
