# Changelog

## 0.4.3

- Fix modrinth ratelimit ( @ClusterConsultant, #36 )
- Fix a bug with old vanilla versions ( @MrPixelized, #39 )
- Fix misspelt `.gitattibutes` -> `.gitattributes` ( jayson729 on discord )
- Preset flags are now `none` by default
- Fix packwiz export showing incorrect URL

## 0.4.2

- Fixes packwiz export
- Adds `world pack` and fixes `world unpack` commands

## 0.4.1

- Fixes markdown not being able to render Github and Jenkins downloadables
- Fix a bug with old forge versions ( thanks for reporting @EldosHD #34 )
- docs: extract changelogs to a file ( thanks for the pr @Madscientiste #35 )

## 0.4.0

- ⚡ New `server/.mcman.lock` file containing metadata about the last build. This file will be used ( if it exists ) to validate and skip files that do not need to be downloaded or bootstrapped again (making builds get a lot faster)

- 📦 New global **caching** most downloaded files and useful metadata will be cached under these directories:
  - Windows: `%localappdata%/mcman`
  - Linux: `~/.cache/mcman` (most likely)

- :octocat: Fixed **Github rate limiting** <sup>(unless you have like a billion `ghrel`'s).</sup> 
    
    - Because of the caching system, metadata from GitHub get cached. `mcman` can use this metadata to send *conditional requests* which do not count towards the rate limit. You can also provide a token by setting `sources.github.api_token` in the config or using the `GITHUB_TOKEN` environment variable.

- 🔒 File **hash checksums** are also implemented, meaning more secure and stable downloads. (Yes, cached files also get checked)

- 📜 **Visual overhaul**: `mcman`'s got a new look. There are now progress bars and spinners everywhere, and its more consistent overall.

- ☔ **Hot reloading** with development sessions, you can develop your server without having to build it every time you change something!

- 🌐 **Worlds** You can now download (in `server.toml`) and store (`worlds/`) worlds!

- ☕ **Java environment variables** You can now set the java version to use (`server.toml`) and mcman will use `JAVA_*_BIN` variables to find and use the java with the version

- ✨ **Networks** Have multiple servers? `network.toml` is now here! Have common addons, common configs and variables! It can also help you manage port configurations with special variables like `${NETWORK_VELOCITY_SERVERS}`

- 📎 **mclo.gs** `mcman run` and `mcman dev` can auto-upload logs to [mclo.gs](mclo.gs)
    
    - enable it by setting env var `upload_to_mclogs` to `true` or setting `services.mclogs.enabled=true` in config

- 🧰 An experimental `.mcman.toml` config file was added (will be looked in current directory and your home config directory)

## 0.3.0

- Added [CurseRinth](https://curserinth.kuylar.dev/) support
- Added **packwiz importing**
- Added **packwiz exporting** (client)
- Added initializing with `--packwiz <source>`
- Added **mrpack exporting**
- Added client-side mods field
- Added `client-config/` folder
- Fixed github ratelimiting issue
- Some init improvements
- A lot of improvements overall

## 0.2.2

- Added support for **Datapacks**
  - Added command `mcman import datapack`
- Added **BuildTools** support.
  - This includes *spigot, bukkit and craftbukkit*
- Even better docs and tutorial.md

## 0.2.1

- Added **Fabric** support.
- Added **Quilt** support.
- Added `mcman import mrpack` command.
- `mcman init` now supports mrpacks

## 0.2.0

- Wrote more [documentation](./DOCS.md)
- New branding lol
- Added markdown templates
  - `markdown` in server.toml
  - `mcman markdown` command
- Added `launcher.properties` in server.toml
- Added `mcman import url <URL>` command
  - Supports modrinth, modrinth's cdn, github, spigot, jenkins and custom urls.
  - Also wayy too interactive. For example, it'll ask for which release to use and suggest which asset to use. Similar thing in modrinth importing.
- Added **BungeeCord** support.
- Added **Jenkins** as a source.
- Impoved `mcman init` command. It now has a little wizard!
- Made mcman build look prettier
- Removed `Folia` shortcut because PaperMC api does not provide it.

## 0.1.0: The Prototype

Initial project
