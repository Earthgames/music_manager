use lofty::{Accessor, Probe, TaggedFileExt};

// type to store music albums and songs

#[derive(Eq, Ord, PartialEq, PartialOrd)]
pub struct MusicTag {
    pub artist_name: String,
    pub song_title: String,
    pub album_title: String,
}

pub fn get_music_tags(music_files: Vec<String>) -> Result<Vec<MusicTag>, &'static str> {
    let mut music_songs: Vec<MusicTag> = Vec::new();

    for music_file in music_files {
        let tagged_file = Probe::open(&music_file)
        		.expect("ERROR: Bad path provided!")
		.read()
		.expect("ERROR: Failed to read file!");
        let tag = match tagged_file.primary_tag() {
            Some(tag) => tag,
            None => tagged_file.first_tag().expect("No tag found"),
        };

        if let Some(title) = tag.title() {
            if let Some(artist) = tag.artist() {
                if let Some(album) = tag.album() {
                    music_songs.push(MusicTag {
                        song_title: title.to_string(),
                        artist_name: artist.to_string(),
                        album_title: album.to_string(),
                    });
                } else {
                    println!("{} is skipped because it has no album tag", &music_file)
                }
            } else {
                println!("{} is skipped because it has no artis tag", &music_file)
            }
        } else {
            println!("{} is skipped because it has no titel tag", &music_file)
        }
    }
    Ok(music_songs)
}

//TODO use lofty instead of id3
// pub fn _change_album(album_titel: &str, file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
//     let mut tag = match Tag::read_from_path(file_path) {
//         Ok(tag) => tag,
//         Err(Error {
//             kind: id3ErrorKind::NoTag,
//             ..
//         }) => Tag::new(),
//         Err(err) => return Err((Box::new(err)) as Box<dyn std::error::Error>),
//     };
//     tag.set_album(album_titel);

//     tag.write_to_path(file_path, Version::Id3v24)?;
//     Ok(())
// }
