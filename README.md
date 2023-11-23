# mcman

![mcman](https://media.discordapp.net/attachments/1109215116060266567/1134554971937972374/mcman-2.png)

[![GitHub release](https://img.shields.io/github/release/ParadigmMC/mcman.svg)](https://github.com/ppy/osu/releases/latest)
[![builds](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/build.yml?logo=github)](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml)
[![docker publish](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/publish.yml?logo=github&label=docker%20publish)](https://github.com/ParadigmMC/mcman/actions/workflows/publish.yml)
![GitHub Repo stars](https://img.shields.io/github/stars/ParadigmMC/mcman?logo=github)
[![quiltmc featured](https://img.shields.io/badge/quiltmc-featured-8A2BE2)](https://tech.lgbt/@quiltmc/110690787441497920)
![downloads](https://img.shields.io/github/downloads/ParadigmMC/mcman/total?logo=github)
[![discord server](https://img.shields.io/discord/1108817072814817410?logo=discord)](https://discord.gg/YFuxNmKvSr)

Powerful Minecraft Server Manager CLI. Easily install jars (server, plugins & mods), write config files, manage worlds and more. Docker and git support included.

## Getting Started

![mcman init](https://cdn.discordapp.com/attachments/1109215116060266567/1134187743300296815/render1690481729604.gif)

| 🚀 [Installation](https://paradigmmc.github.io/mcman/installation/) | ✨ [Getting Started](https://paradigmmc.github.io/mcman/tutorials/getting-started/) | 📜 [Documentation](https://paradigmmc.github.io/mcman/) |
| ------------------------------------------------------------------ | ---------------------------------------------------------------------------------- | ------------------------------------------------------ |

- Join the [discord](https://discord.gg/YFuxNmKvSr) for support!
- 📋 Some examples can be found under [examples/](./examples/)

Submit a PR or open an issue if you have a mcman-server repository that we can add here!

## Features

- 📜 Everything in one simple `server.toml` file!
- 📥 Downloads the *server jar*, *plugins*, *mods*, *datapacks* and *worlds* (if any)!
- 🔁 Always keep up to date with new builds and releases
- ✔️ No third-party hosts (metadata/mirrors)
- :octocat: Fully `git`-compatible!
- 🐳 Supports Docker, out of the box
- 📦 **Import** from or **export** to `mrpack`s or `packwiz` packs!
- 📚 Supports way too many sources, some are:
  - Modrinth, CurseRinth, Spigot, Hangar, Github Releases, Jenkins, Maven
  - If you need something else, it even supports custom urls!
- ⚙️ Better configuration files with `config/`!
  - Allows you to use **variables** inside your config files
  - Use *environment variables* for secrets
- 🌐 Keep worlds as `worlds/*.zip` for git, or set it to be downloaded from somewhere!
- ✨ Managing a network? Use `network.toml` to manage servers' ports, have shared variables, config files, plugins or mods.

## Reviews

"faster than gradle builds"

\- kuylar

"makes even oracle linux usable"

\- PureComedi

"I'm technically a contributor"

\- Trash Panda

## Changelog

whats a semver? /s

### `0.4.0`

- See [#31](https://github.com/ParadigmMC/mcman/issues/31)

### `0.3.0`

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
