# Change log

## 0.0.4

- Added a tag command, to tag with picard and move automatically to the library
  - The download command will run the tag command on the downloaded music files
- Added check function, to check if all files are in the right directory,
  have files specified in the config in the album folder (like a cover image)
- Added the ability to add albums, to add albums with there cover in one go
- Added artist-only categories, these put all the albums in the main category folder,
- cat works with library structure
- Replaced loudgain with rsgain
- Doesn't use tmp directory anymore, uses the Untagged directory instead
- Use album artist for the artist folder
  instead of having an artist folder and then the album folder
- Category config default name is now the folder name

## 0.0.3

- yt-dlp and loudgain respect loglevel quiet
- added the add subcommand, to add music on disk to the library
- renamed genre to category
- music in categories is sorted on artist then album
- normalizing now works
- cat(perviously genr) does not work with the current library structure

## 0.0.2

- reorganized the code
- added logging
- added man pages
- added bash/zsh completion
- improved error handling

## 0.0.1

First version

- download music with yt-dlp to `.opus` files and normalize it / add replaygain tags with loudgain and move to a folder
- search a genre(folder) to see what genres there are, and what music is in them
- create a genre(folder)
- a config file with options which folders to use
