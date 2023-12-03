# Music Manager

A cli to manage music

## How to use

```text

Usage: music_manager [OPTIONS] `<COMMAND>`

Commands:

  down    download youtube music and move in a genre directory
  genr    print genres with a description
  mkgenr  makes a new genre directory
  help    Print this message or the help of the given subcommand(s)

Options:

  -c, --clean    Clean tmp directory on exit
  -h, --help     Print help
  -V, --version  Print version
```

## How to install

Install [loudgain](https://github.com/Moonbase59/loudgain "https://github.com/Moonbase59/loudgain") and [yt-dlp](https://github.com/yt-dlp/yt-dlp "https://github.com/yt-dlp/yt-dlp"). Making sure that they can be run form a terminal

Clone this repository and use `cargo install --path .`

## Config

The main config file should be at

| Platform          | Path                                                              |
| ----------------- | ----------------------------------------------------------------- |
| Linux             | /home/user/.config/music_manager/config.toml                      |
| Windows(untested) | /Users/user/Library/Application Support/music_manager/config.toml |
| macOs(untested)   | C:\Users\user\AppData\Roaming/music_manager/config.toml           |

The following options are available

music_dir, the root directory for the music

default_dir, the directory where the music without a directory name are put in

tmp_dir, the directory where the music is downloaded in

### Folder structure

An example  of a folder structure. Note that the tmp folder does not have to be in the root directory

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
├── soul
│   └── description.toml
└── tmp
```

A description.toml should be in every folder that can be used by the Music Manager
the description has the following fields
name, the name of the music that is in the folder
description, a description of the music that is in the folder

Note that the name field is not used when searching for a directory in the down subcommand. But shorts can be used to type less thus  `music_manager down youtube/link j` will result in it being moved to the j-pop folder
