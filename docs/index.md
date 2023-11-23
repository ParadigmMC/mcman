# mcman

The Powerful Minecraft Manager CLI.

`mcman` allows you to:
- Have a single `server.toml` for auto-updating plugins/mods, the server jar, worlds, etc...
- get rid of the hassle of downloading and dragging over jar files,
- efficiently write config files using config and environment variables,
- use `git` for your servers to be able to version control and collaborate,
- render markdown about the server
- test if the server works using CI
- and more!

`mcman` is not great for:
- Creating clientside modpacks or modpacks to only publish them (use [packwiz](https://packwiz.infra.link/) for that)

## Quick Start

- [Installation](./installation.md)
- [Getting Started](./tutorials/getting-started.md)
- [Building, Running and Developing](./tutorials/building.md)
- [Variables and Bootstrapping](./tutorials/variables.md)
- [Commands](./commands)
- [Reference](./reference/server.toml.md)

## Features

- Downloads or sets up server software (server.jar)
- Downloads plugins, mods and datapacks
- Provides a better way to manage configuration files
- Gives you the ability to use variables from both the config file and environment variables inside your server's mod/plugin configuration files
- Supports many sources to download from (16)
- Your source isn't supported? Just use a custom url
- Import from URL
- Import from mrpacks or packwiz packs
- Export to mrpacks or packwiz packs
- Supports Jenkins.
- Rendering server info to markdown feature
- Docker support

## TODO

- Curseforge Modpack support
- Better commands to search, add or remove mods/plugins/datapacks
