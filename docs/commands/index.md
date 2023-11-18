# CLI Commands

This section shows the commands of mcman. You can type `mcman`, `mcman help` or `mcman --help` for a basic list of it.

## Cheatsheet

- Create
    - `mcman init`: create new server
    - `mcman init --mrpack <source>`: create server from mrpack
    - `mcman init --packwiz <source>`: create server from packwiz
- Build
    - `mcman build`: build the server
    - `mcman run`: build then run the server
    - `mcman run --test`: build then run to test if it works
    - `mcman dev`: start a dev session
- Addons
    - `mcman import url <url>`: import an addon from url
    - `mcman import datapack <url>`: import datapacks
- Export/Import
    - `mcman import packwiz <source>`: import packwiz packs
    - `mcman import mrpack <source>`: import mrpacks
    - `mcman export packwiz [pack folder]`: export as packwiz pack
    - `mcman export mrpack [filename.mrpack]`: export as mrpack
- Info
    - `mcman info`: show info about the server
    - `mcman version`: show version
- Cache
    - `mcman cache path`: print cache path
    - `mcman cache open`: open the cache folder
    - `mcman cache list [-d]`: list caches, `-d` for detailed
    - `mcman cache clear`: delete caches without confirm
- Misc
    - `mcman markdown`: render markdown templates
    - `mcman download <dl>`: download a downloadable
    - `mcman world unpack [world]`: unzip a world
    - `mcman pull <file>`: pull files from `server/` to `config/`
    - `mcman env gitignore`: edit git dotfiles
    - `mcman env docker`: create default docker files
