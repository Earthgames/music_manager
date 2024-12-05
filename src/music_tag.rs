use std::path::Path;

use lofty::file::TaggedFileExt;
use lofty::read_from_path;
use lofty::tag::{Accessor, ItemKey, Tag, TagExt, TagType};

use anyhow::{anyhow, Result};

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
    let tag = get_tag(music_file)?;

    // Song title
    let Some(title) = tag.title() else {
        return Err(anyhow!("could not find title tag"));
    };
    // artist name
    let Some(artist) = tag.artist() else {
        return Err(anyhow!("could not find artist tag"));
    };
    // album title
    let Some(album) = tag.album() else {
        return Err(anyhow!("could not find album tag"));
    };
    // album artist
    let Some(album_artist) = tag.get(&ItemKey::AlbumArtist) else {
        return Err(anyhow!("could not find album artist tag"));
    };
    Ok(MusicTag {
        song_title: title.to_string(),
        album_artist: album_artist
            .clone()
            .into_value()
            .into_string()
            .unwrap_or_default(),
        album_title: album.to_string(),
        artist_name: artist.to_string(),
        replaygain: tag_has_replaygain_tags(tag),
    })
}

/// Check if a music file has replaygain tags
pub fn file_has_replaygain_tags(music_file: &Path) -> Result<bool> {
    let tag = get_tag(music_file)?;
    Ok(tag_has_replaygain_tags(tag))
}

pub fn tag_has_replaygain_tags(tag: Tag) -> bool {
    let result = tag.contains(&ItemKey::ReplayGainTrackGain)
        || tag.contains(&ItemKey::from_key(
            TagType::VorbisComments,
            "R128_TRACK_GAIN", // for opus and ogg types
        ));
    result
}

fn get_tag(music_file: &Path) -> Result<Tag> {
    let tagged_file = read_from_path(music_file)?;
    let tag = match tagged_file.primary_tag() {
        Some(tag) => tag,
        None => match tagged_file.first_tag() {
            Some(tag) => tag,
            None => {
                return Err(anyhow!(
                    "No tag found in file : \"{}\"",
                    music_file.display()
                ))
            }
        },
    };
    Ok(tag.clone())
}
