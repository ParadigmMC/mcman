# mcman

mcman is a command line tool that makes it easier to create and manage minecraft servers. Instead of manually downloading jars from the internet and doing intricate replace operations and whatnot on your plugin configuration files, mcman downloads everything you need from one configuration file (`server.toml`) and handles your config files with one very useful feature - variables.

## Quick Start

- [Installation](./installation.md)
- [Getting Started](./tutorials/getting-started.md)
- [Explaining how building works](./tutorials/building.md)
- [Commands](./commands)
- [Reference](./reference/server.toml.md)

## Features

- Downloads or sets up server software (server.jar)
- Downloads plugins, mods and datapacks
- Provides a better way to manage configuration files
- Gives you the ability to use variables from both the config file and environment variables inside your server's mod/plugin configuration files
- Supports many sources to download from (13)
- Your source isn't supported? Just use a custom url
- Import from URL
- Import from mrpacks or packwiz packs
- Export to mrpacks or packwiz packs
- Supports Jenkins.
- Rendering server info to markdown feature
- Docker support

## TODO

- Maven support, also allowing Forge and NeoForge
- Curseforge Modpack support
- Better commands to search, add or remove mods/plugins/datapacks
