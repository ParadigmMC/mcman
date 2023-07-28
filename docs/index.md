<!-- markdownlint-disable MD033 -->
# mcman

mcman is a command line tool that makes it easier to create and manage minecraft servers. Instead of manually downloading jars from the internet and doing intricate replace operations and whatnot on your plugin configuration files, mcman downloads everything you need from one configuration file (`server.toml`) and handles your config files with one very useful feature - variables.



Index:

- [CLI](#cli)
- [Folder Structure](#folder-structure)
- [Variables](#variables)
- [server.toml](#servertoml)
  - [Server Launcher](#server-launcher)
  - [Markdown Options](#markdown-options)
  - [World](#world) (for datapacks)
  - [Downloadables](#downloadable)

## CLI



---

#

#
---

## Types

Below are some types used in `server.toml`

### Downloadable

A downloadable is some source of a plugin, mod, datapack or a server jar.

Index of sources:

- [Vanilla](#vanilla)
- [Fabric](#fabric)
- [Quilt](#quilt)
- [PaperMC](#papermc) (Paper, Waterfall and Velocity)
- [PurpurMC](#purpurmc)
- [BungeeCord](#bungeecord)
- [Modrinth](#modrinth)
- [CurseRinth](#curserinth)
- [Spigot](#spigot)
- [Github Releases](#github-releases)
- [Jenkins](#jenkins)
- [Custom URL](#custom-url)
- [BuildTools](#buildtools)

#### Vanilla

Used for a vanilla server jar. Has no properties

```toml
type = "vanilla"
```

#### Fabric

The [Fabric](https://fabricmc.net/) mod loader

**Options:**

- `type` = `"fabric"`
- `installer`: string | `"latest"` - Installer version to use
- `loader`: string | `"latest"` - Loader version to use

```toml
type = "fabric"
installer = "latest"
loader = "latest"
```

#### Quilt

The [Quilt](https://quiltmc.org/) project - mod loader compatible with fabric

Due to some complexities with quilt, `mcman` will need to run `java` to install the quilt server jar - keep this in mind.

**Options:**

- `type` = `"quilt"`
- `installer`: string | `"latest"` - Installer version to use
- `loader`: string | `"latest-beta"` | `"latest"` - Loader version to use

```toml
type = "quilt"
installer = "latest"
loader = "latest"
```

#### PaperMC

Allows downloading a [PaperMC](https://papermc.io/) project.

**Options:**

- `type` = `"papermc"`
- `project`: string - The project name
- `build`: string | `"latest"` - Optional

```toml
# Its recommended to use the shortcuts:
type = "paper"
type = "waterfall"
type = "velocity"

# Or you can use the base instead:
type = "papermc"
project = "paper"

# Optionally define the build if you dont want to use the latest:
type = "papermc"
project = "waterfall"
build = "17"
# Note: the shortcuts do not support the 'build' property
```

#### PurpurMC

Downloads server jar from [PurpurMC](https://purpurmc.org/).

**Options:**

- `type` = `"purpur"`
- `build`: string | `"latest"` - Optional

```toml
type = "purpur"

# like paper, can also specify build
build = "10"
# if omitted, uses latest
```

#### BungeeCord

BungeeCord is just a shortcut to a [jenkins](#jenkins) downloadable:

```toml
type = "bungeecord"
```

If you'd like to get a specific build, use this:

```toml
type = "jenkins"
url = "https://ci.md-5.net"
job = "BungeeCord"
build = "latest"
artifact = "BungeeCord"
```

#### Modrinth

Downloads from [Modrinth](https://modrinth.com/)'s API

**Options:**

- `type` = `"modrinth"` | `"mr"`
- `id`: string - id of the project or the slug
- `version`: string | `"latest"` - Version ID, `"latest"` not recommended

```toml
[[plugins]]
# you can also use 'mr' as an alias
type = "modrinth"
id = "coreprotect"
version = "mvLpRWww"
```

#### CurseRinth

Downloads from [CurseRinth](https://curserinth.kuylar.dev/)'s API, which is basically [curseforge](https://www.curseforge.com/)

**Options:**

- `type` = `"curserinth"`
- `id`: string - id of the project or the slug
- `version`: string | `"latest"` - File id

```toml
[[plugins]]
type = "curserinth"
id = "jei"
version = "4593548"
```

#### Spigot

This uses [Spiget](https://spiget.org/)'s API.

**Options:**

- `type` = `"spigot"`
- `id`: string - id of the project

You can find the ID of the resource in the URL:

> `https://www.spigotmc.org/resources/luckperms.28140/`

In this case, luckperms has the id of `28140` - but you can paste it with the name too:

```toml
[[plugins]]
type = "spigot"
id = "luckperms.28140"
```

mcman will ignore everything before the dot

#### Github Releases

Allows downloading from github releases

**Options:**

- `type` = `"ghrel"`
- `repo`: string - repository identifier, like `"ParadigmMC/mcman"`
- `tag`: string | `"latest"` - The tag of the release
- `asset`: string | `"first"` - The name of the asset (checks for inclusion)

```toml
[[plugins]]
type = "ghrel"
repo = "ViaVersion/ViaVersion"
tag = "4.7.0"
# real asset name is ViaVersion-4.7.0.jar
asset = "ViaVersion"
# searches for inclusion
```

#### Jenkins

Use a jenkins server

**Options:**

- `type` = `"jenkins"`
- `url`: string - url of the jenkins server
- `job`: string - The job
- `build`: string | `"latest"` - The build number to use
- `artifact`: string | `"first"` - The name of the artifact (checks for inclusion, like ghrel)

Example using [Scissors](https://github.com/AtlasMediaGroup/Scissors) 1.20.1:

```toml
[jar]
type = "jenkins"
url = "https://ci.plex.us.org"
# nested jobs are supported like this:
job = "Scissors/1.20.1"

# these are the default values and since
# they are optional, they can be removed
build = "latest"
artifact = "first"
```

#### Custom URL

Allows you to download from a defined URL.

**Options:**

- `type` = `"url"`
- `url`: string - URL to the file
- `filename`: string? - Optional filename if you dont like the name from the url
- `desc`: string? - Optional description (shown in markdown tables)

```toml
[[mods]]
type = "url"
url = "https://example.com/download/Example.jar"
# Optionally define the filename, useful if it cannot be inferred from the url
filename = "example-mod.jar"
```

#### BuildTools

Setup Spigot, Bukkit or CraftBukkit using [BuildTools](https://www.spigotmc.org/wiki/buildtools/).

**Options:**

- `type` = `"buildtools"`
- `args`: string[] - Additional args, such as `["--compile", "bukkit"]` - mcman only adds `--rev {mc_version}`

```toml
[server.jar]
type = "buildtools"
args = []
```
