# mcman

![mcman](https://media.discordapp.net/attachments/1109215116060266567/1121117662785851522/mcman_large.png)

[![builds](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/build.yml?logo=github)](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml)
[![docker publish](https://img.shields.io/github/actions/workflow/status/ParadigmMC/mcman/publish.yml?logo=github&label=docker%20publish)](https://github.com/ParadigmMC/mcman/actions/workflows/publish.yml)
![GitHub Repo stars](https://img.shields.io/github/stars/ParadigmMC/mcman?logo=github)

The Minecraft Server Manager CLI

<!-- todo: a (terminal) screenshot here -->

## Features

- Downloads the server jar, plugins and mods according to the `server.toml` config file
  - Always keep up to date with new serverjar builds!
  - No more manually downloading jars - mcman auto updates them according to your `server.toml`
  - Supports a variety of [sources](./DOCS.md#downloadable):
    - Server jars:
      - Vanilla
      - PaperMC (Paper, Folia, Waterfall and Velocity)
      - PurpurMC
    - Plugins/Mods:
      - Modrinth
      - Spigot
    - And even **Github Releases**, **Custom URL**s and **Jenkins!**
- Bootstraps your server configuration files
  - Allows you to use variables inside your config files
  - Environment variables for secrets
  - You can now use git to version-control your server without making a complex `.gitignore`!
- Docker support out of the box
- Easy to use

## Getting Started

View the [Documentation](./DOCS.md) here.

## Special Thanks

- [flags.sh](https://flags.sh/) for the flags and stuff
  - thank you aikar
- PaperMC and Modrinth for having an amazing API
- You for using our project
