# mcman

![mcman](https://media.discordapp.net/attachments/1109215116060266567/1134554971937972374/mcman-2.png)

[![GitHub release](https://img.shields.io/github/release/ParadigmMC/mcman.svg)](https://github.com/ppy/osu/releases/latest)
[![builds](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/build.yml?logo=github)](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml)
[![docker publish](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/publish.yml?logo=github&label=docker%20publish)](https://github.com/ParadigmMC/mcman/actions/workflows/publish.yml)
![GitHub Repo stars](https://img.shields.io/github/stars/ParadigmMC/mcman?logo=github)

Powerful Minecraft Server Manager CLI. Easily install jars (server, plugins & mods) and write config files. Docker and git support included.

<!-- todo: a (terminal) screenshot here -->

## Features

- 📜 Everything in one simple `server.toml` file!
- 📥 Downloads the *server jar*, *plugins*, *mods* and *datapacks*!
- 🔁 Always keep up to date with new builds and releases
- ✔️ No more manually downloading jars or editing config files on remote
- 📚 Supports a variety of [sources](./DOCS.md#downloadable):
  - **Servers**:
    - 🌳 `Vanilla`, `Paper`, `Purpur`, `BuildTools` (Spigot/CraftBukkit)
    - 📜 **Modded:** Fabric & Quilt
    - ⛓️ **Proxies:** Velocity, Waterfall, BungeeCord
  - **Plugins/Mods/Datapacks**: 🍀 `Modrinth`, 🔥 `CurseRinth` (Curseforge), 🚰  `Spigot` (resources)
  - **And even** :octocat: `Github Releases`, 🔗 `Custom URL`s and 💁 `Jenkins`!
- ⚙️ Better configuration files with `config/`!
  - Allows you to use **variables** inside your config files
  - Use *environment variables* for secrets
  - You can now use `git` to version-control your server without making a complex `.gitignore`!
- 🐳 Supports Docker
- 📦 Import from or export to [mrpack](./DOCS.md#mcman-import-mrpack-src)s!
- 📦 Import from or export to [packwiz](./DOCS.md#mcman-import-mrpack-src) packs!

## Getting Started

![mcman init](https://cdn.discordapp.com/attachments/1109215116060266567/1134187743300296815/render1690481729604.gif)

- 🚀 [Installation instructions](./TUTORIAL.md#installation)

- ✨ [Recommended Usage](./TUTORIAL.md#recommended-usage)

- 📜 View the [**Documentation**](./DOCS.md) here.

- 📋 Want an example? Here's [iptfreedom](https://github.com/IPTFreedom/iptfreedom)

Submit a PR or open an issue if you have a mcman-server repository that we can add here!

## Changelog

### `0.3.0` (unreleased)

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

### `0.2.2`

- Added support for **Datapacks**
  - Added command `mcman import datapack`
- Added **BuildTools** support.
  - This includes *spigot, bukkit and craftbukkit*
- Even better docs and tutorial.md

### `0.2.1`

- Added **Fabric** support.
- Added **Quilt** support.
- Added `mcman import mrpack` command.
- `mcman init` now supports mrpacks

### `0.2.0`

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

### `0.1.0`: The Prototype

Initial project

## Special Thanks

- [flags.sh](https://flags.sh/) for the flags and stuff
  - thank you aikar
- PaperMC and Modrinth for having an amazing API
- You for using our project
