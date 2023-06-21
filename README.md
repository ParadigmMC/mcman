# mcman

The Minecraft Server Manager

<!-- todo: a screenshot here -->

## Getting Started

Check out the [tutorial](./TUTORIAL.md)!

View the [Documentation](./DOCS.md) here.

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

## Special Thanks

- [flags.sh](https://flags.sh/) for the flags and stuff
  - thank you aikar
- PaperMC and Modrinth for having an amazing API
- You for using our project
