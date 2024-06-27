# Music Manager

A cli to manage music

## How to use

```text
Usage: music_manager [OPTIONS] <COMMAND>

Commands:
  down   <URL> [CATEGORY]   Download music form YouTube, and move in a category directory 
  cat   [CATEGORY]          Print categories with a description
  mkcat <CATEGORY> [DESCRIPTION] 
                            Makes a new category directory
  check [CATEGORY] [-t]     Check music, -t will make it check all tags(slow)
  tag   [-f] -c <CATEGORY> [FILES]...
                            Tag music and move to the library, -f will force to tag all files
  help                      Print this message or the help of the given subcommand(s)

Options:
  -l, --log_level <LOG_LEVEL>  
                        log level: 0 silent, 1 errors, 2 warnings, 3 info, [default: 3]
  -h, --help            Print help
  -V, --version         Print version
```

## How to install

Install [rsgain](https://github.com/complexlogic/rsgain "https://github.com/complexlogic/rsgain"), [yt-dlp](https://github.com/yt-dlp/yt-dlp "https://github.com/yt-dlp/yt-dlp")(to use down) and [picard](https://github.com/metabrainz/picard "https://github.com/metabrainz/picard")(to use tag).
Make sure that they can be run form a terminal

Clone this repository and use `cargo install --path .`

### Completions

For linux only, the same files could possibly be used on windows.
There are also files for other shells, but I would not know where to put them

#### Bash

Copy the ./target/assets/music_manager.bash to /usr/share/bash_completion/completions/music_manager:
`sudo cp ./target/assets/music_manager.bash /usr/share/bash-completion/completions/music_manager`

#### Zsh

*Untested*
Copy the ./target/assets/_music_manager to /usr/share/zsh/functions/Completion/Base/_music_manager
`sudo cp ./target/assets/_music_manager /usr/share/zsh/functions/Completion/Base/`

### Man pages

Move the man pages form ./target/assets/ to /usr/share/man/man1
`sudo mv ./target/assets/*.1 /usr/share/man/man1/`

## Config

The main config file should be at

| Platform          | Path                                                              |
|-------------------|-------------------------------------------------------------------|
| Linux             | /home/user/.config/music_manager/config.toml                      |
| Windows(untested) | /Users/user/Library/Application Support/music_manager/config.toml |
| macOs(untested)   | C:\Users\user\AppData\Roaming/music_manager/config.toml           |

The following options are available

- music_dir, the root directory for the music
- default_dir, the directory where the music without a directory name are put in
- file_extensions, which file-extension are allowed in the library
- album_files, files to check in the album folders, global, like "cover*"

### Folder structure

An example of a folder structure.

```text
.
├── bgm
│   └── description.toml
├── j-pop
│   └── description.toml
├── other
│   └── description.toml
├── rock
│   └── description.toml
└── soul
    └── description.toml

```

A description.toml should be in every folder that can be used by the Music Manager

the description has the following fields

- name, the name of the music that is in the folder
- description, a description of the music that is in the folder
- artist_category, if the albums should be put directly be put in the folder
- albums_files, files to check in the folder like "cover*"

Note that the name field is not used when searching for a category. But shorts can be used, so: `music_manager down youtube/link j` will result in it being moved to the `j-pop` folder
