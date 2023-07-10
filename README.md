# mcman

![mcman](https://media.discordapp.net/attachments/1109215116060266567/1121117662785851522/mcman_large.png)

[![builds](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/build.yml?logo=github)](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml)
[![docker publish](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/publish.yml?logo=github&label=docker%20publish)](https://github.com/ParadigmMC/mcman/actions/workflows/publish.yml)
![GitHub Repo stars](https://img.shields.io/github/stars/ParadigmMC/mcman?logo=github)

Powerful Minecraft Server Manager CLI. Easily install jars (server, plugins & mods) and write config files. Docker and git support included.

<!-- todo: a (terminal) screenshot here -->

## Features

- Downloads the server jar, plugins and mods according to the `server.toml` config file
  - Always keep up to date with new serverjar builds!
  - No more manually downloading jars - mcman auto updates them according to your `server.toml`
  - Supports a variety of [sources](./DOCS.md#downloadable):
    - Servers:
      - Vanilla
      - Fabric
      - Quilt
      - Paper
      - PurpurMC
      - Velocity
      - Waterfall
      - BungeeCord
    - Plugins/Mods:
      - Modrinth
      - Spigot
    - And even **Github Releases**, **Custom URL**s and **Jenkins!**
  - Supports importing from [mrpack](./DOCS.md#mcman-import-mrpack-src)s!
- Bootstraps your server configuration files
  - Allows you to use variables inside your config files
  - Environment variables for secrets
  - You can now use git to version-control your server without making a complex `.gitignore`!
- Docker support out of the box
- Easy to use

## Getting Started

View the [Documentation](./DOCS.md) here.

## Changelog

### `0.3.0` (unreleased)

- Added **Fabric** support.
- Added **Quilt** support.
- Added `mcman import mrpack` command.

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
