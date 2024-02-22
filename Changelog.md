# Change log

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
