use crate::Result;
use lofty::{Accessor, ItemKey, Probe, Tag, TagExt, TaggedFileExt};
use log::warn;

/// Type to store music albums and songs
#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct MusicTag {
    pub artist_name: String,
    pub song_title: String,
    pub album_title: String,
}

pub fn get_music_tags(music_files: &Vec<String>) -> Result<Vec<MusicTag>> {
    let mut music_songs: Vec<MusicTag> = Vec::new();

    for music_file in music_files {
        let tag = get_tag(music_file)?;

        if let Some(title) = tag.title() {
            if let Some(artist) = tag.artist() {
                if let Some(album) = tag.album() {
                    music_songs.push(MusicTag {
                        song_title: title.to_string(),
                        artist_name: artist.to_string(),
                        album_title: album.to_string(),
                    });
                } else {
                    warn!("{} is skipped because it has no album tag", &music_file)
                }
            } else {
                warn!("{} is skipped because it has no artist tag", &music_file)
            }
        } else {
            warn!("{} is skipped because it has no title tag", &music_file)
        }
    }
    Ok(music_songs)
}

pub fn has_replaygain_tags(music_file: &String) -> Result<bool> {
    let tag = get_tag(music_file)?;
    Ok(tag.contains(&ItemKey::ReplayGainTrackPeak) && tag.contains(&ItemKey::ReplayGainTrackGain))
}

fn get_tag(music_file: &String) -> Result<Tag> {
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
