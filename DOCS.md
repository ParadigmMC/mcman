<!-- markdownlint-disable MD033 -->
# `mcman` Documentation

> Warning:
> Outdated, see [this website](https://paradigmmc.github.io/mcman/) for more up-to-date docs

You might be looking for [tutorial.md](./TUTORIAL.md)

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

Here are a list of commands. You can type `mcman` or `mcman --help` for a basic list of it.

### `mcman init`

Initializes a new server in the current directory.

> **Full Command:** `mcman init [--name <name>] [--mrpack <source> | --packwiz <source>]`

This command is interactive. Just run `mcman init`!

<details>
<summary>
📦 Importing from a mrpack (modrinth modpack)
</summary>

You can use the `--mrpack` flag on `mcman init` to import from an mrpack while initializing a server.

- If its from modrinth, like [adrenaserver](https://modrinth.com/modpack/adrenaserver): `mcman init --mrpack mr:adrenaserver`

Use `mr:` and then the project id/slug of the modpack (should be visible on the url)

- You can also just paste in the modpack page's url: `mcman init --mrpack https://modrinth.com/modpack/adrenaserver`

- If its from another source, you can provide a download link to it: `mcman init --mrpack https://example.com/pack.mrpack`

- If its a file: `mcman init --mrpack ../modpacks/pack.mrpack`

If your server is already initialized, use the `mcman import mrpack <source>` command. The source argument also accepts the sources defined above.

Example using [Adrenaserver](https://modrinth.com/modpack/adrenaserver):

```sh
# these are all identical
mcman init --mrpack mr:adrenaserver
mcman init --mrpack https://modrinth.com/modpack/adrenaserver
mcman init --mrpack https://cdn.modrinth.com/data/H9OFWiay/versions/2WXUgVhc/Adrenaserver-1.4.0%2B1.20.1.quilt.mrpack
```

</details>

<details>
<summary>
📦 Importing from a packwiz pack
</summary>

You can use the `--packwiz` (alias `--pw`) flag on `mcman init` to import a packwiz pack while initializing.

**If the pack is in your filesystem**:

```sh
mcman init --pw path/to/pack.toml
```

**If the pack is online**:

```sh
mcman init --pw https://raw.githack.com/EXAMPLE/EXAMPLE/main/pack.toml
```

If your server is already initialized, use the `mcman import packwiz <source>` command. The source argument also accepts the sources defined above.
</details>

### `mcman version`

Shows the version of **mcman** and checks if its up to date.

### `mcman build`

Builds the server into the [output folder](#folder-structure) using the [`server.toml`](#servertoml) and the `config/` directory.

<details>
<summary>Extra flags (output, skip, force)</summary>

You can alternatively set the output folder manually using `--output <path>` option.

The `--force` flag can be used to disable checking if files exist, effectively forcefully downloading everything.

You can use the `--skip <stages>` flag to skip stages.

- Stages should be comma-seperated, like `--skip bootstrap,scripts,dp`
- The stages are: `serverjar`, `plugins`, `mods`, `dp` (datapacks), `bootstrap` (config/) and `scripts`

</details>

After building, you can start the server with the launch scripts if theyre not [disabled](#server-launcher):

```sh
cd server

# windows
call start.bat
# linux
./start.sh
```

### `mcman pull <file>`

'Pulls' a file from `server/` to `config/`

Example usage:

```sh
~/smp $ ls
 ...
 server.toml
 ...

~/smp $ cd server/config/SomeMod

~/smp/server/config/SomeMod $ mcman pull config.txt
  server/config/SomeMod/config.txt => config/config/SomeMod/config.txt
```

### `mcman info`

Shows info about the server in the terminal.

### `mcman markdown`

> Alias: `mcman md`

This command refreshes the markdown files defined in the [server.toml](#markdown-options) files with the templates.

See [markdown options](#markdown-options) for more information.

### `mcman import ...`

> Alias: `mcman i ...`

Commands related to importing

#### `mcman import url <URL>`

Imports a plugin or a mod from a url.

Supports:

- `[cdn.]modrinth.com`
- `curserinth.kuylar.dev`
- `www.curseforge.com`
- `www.spigotmc.org`
- `github.com`
- If not any of those, will prompt with **direct url** or **jenkins**

Example usages:

```sh
mcman import url https://modrinth.com/plugin/imageframe
mcman import url https://www.spigotmc.org/resources/armorstandeditor-reborn.94503/
mcman import url https://ci.athion.net/job/FastAsyncWorldEdit/
```

#### `mcman import datapack <URL>`

Like [import url](#mcman-import-url-url), but imports as a datapack rather than a plugin or a mod.

Example usage:

```sh
# datapack alias is dp
mcman import dp https://modrinth.com/plugin/tectonic
```

#### `mcman import mrpack <src>`

Imports a [mrpack](https://docs.modrinth.com/docs/modpacks/format_definition/) file (modrinth modpacks)

**Note:** [`mcman init`](#mcman-init) supports mrpacks

The source can be:

- A direct URL to a `.mrpack` file
- A local file path
- Modpack URL (`https://modrinth.com/modpack/{id}`)
- Modrinth project id prefixed with `mr:`

Example usages:

```sh
# direct link
mcman import mrpack https://cdn.modrinth.com/data/xldzprsQ/versions/xWFqQBjM/Create-Extra-full-1.1.0.mrpack
# only /modpack urls
mcman import mrpack https://modrinth.com/modpack/create-extra
# prefixed
mcman import mrpack mr:simply-skyblock
# local file
mcman import mrpack My-Pack.mrpack
```

#### `mcman import packwiz <src>`

> Alias: `mcman i pw <src>`

Imports a [packwiz](https://packwiz.infra.link/) pack

**Note:** [`mcman init`](#mcman-init) supports packwiz

The source can be:

- A packwiz pack URL
- A local file path to `pack.toml`

Example usages:

```sh
mcman import packwiz https://raw.githack.com/ParadigmMC/mcman-example-quilt/main/pack/pack.toml
mcman import packwiz ../pack.toml
```

---

### `mcman export ...`

Exporting commands

#### `mcman export mrpack [filename]`

Export the server as an `mrpack` (modrinth modpack) file

If `[filename]` argument isn't given, it will be exported as `${SERVER_NAME}.mrpack`

See also: [special variables](#special-variables) that contain export-related variables

#### `mcman export packwiz`

> **Alias & Full Command:** `mcman export pw [-o --output <FOLDER>] [--cfcdn]`

Export the server as a packwiz pack, by default to `pack/` folder.

If you are in a git repo, mcman will give you the githack url to the generated `pack.toml` at the end of the export.

<details>
<summary>
Extra options (output & cfcdn)
</summary>

You can use the `--output <folder>` option to set a custom destination to the pack.

Using `mcman export pw --output packwiz/pack` will create `pack.toml` to `./packwiz/pack/pack.toml`

If the `--cfcdn` flag is used, every `curserinth` downloadable will use `download.mode = "url"` with `download.url` being the url from curseforge's cdn.

If its not used, `download.mode = "metadata:curseforge"` is used with `update.curseforge = { .. }` (default packwiz behavior)

</details>

See also: [special variables](#special-variables) that contain export-related variables

---

## Folder Structure

In a normal server environment, everything is in one folder and a big giant mess to navigate.
And database files are next to config files!

When using `mcman`, your folder structure will look something like this:

- 📂 **cool_server/**
  - 📋 **`server.toml`**
  - 🚢 `Dockerfile`
  - 📜 `.dockerignore` and `.gitignore`
  - 📁 **config/**
    - 📜 `server.properties`
  - 📁 **server/** (git-ignored - only on your host)
    - ... server env files ...
    - ☕ `server.jar`
    - 📜 `server.properties`
    - 📜 `bukkit`/`spigot`/`paper`/`commands`/`help`/`permissions`/`pufferfish`/`purpur`/`wepif.yml`

Inside the folder for your server, you'll see a few files and folders:

- **`server.toml`**: This is the configuration file for your server, more info [in its own section](#servertoml)
- **config/** folder: This is the folder your server config files should go. `mcman` will process everything into the output.
  - The path is converted as follows:
    `config/server.properties` => `server/server.properties`
    And every config file (.properties, .toml, .yaml, .yml etc) will be processed with [variables](#variables)
- **server/** folder: This folder is where `mcman` will build the server files into, aka the output. This folder is gitignored by default (because why would you do that?)
  - According to the `server.toml`, mcman can generate launcher scripts at `server/start.sh` and `server/start.bat`
- **`.gitignore` and `.dockerignore`**: Ignore files for git and docker
- **Dockerfile**: If you enabled docker, this is the dockerfile

## Variables

In your configuration files inside `config/`, you can use variables defined in `server.toml`:

📋 `server.toml`:

```toml
name = "a"
mc_version = "1.20.1"

# <...>

[variables]
PORT = "25500"
MOTD = "welcome to a"
WEBSITE = "https://example.com/"
Prefix = "[a]"
# key-value table
```

<details>
<summary>
Using environment variables
</summary>

If your variables are sensitive (such as discord bot tokens) you can use environment variables:

```bash
# Linux/Mac:
export TOKEN=asdf
```

```bat
:: Windows:
set TOKEN=asdf
```

Environment variables are also put onto config files.

</details>

And then use the variables inside any config file inside `config/`:

<details>
<summary>
Example configuration files
</summary>

📜 `config/server.properties`:

```properties
# use a colon (:) to provide defaults inside configs
server-port=${PORT:25565}
gamemode=creative
motd=${MOTD}
online-mode=false
```

📜 `config/plugins/someplugin/bossbar.yaml`:

```yaml
bossbar: "${SERVER_NAME} - ${WEBSITE}"
```

📜 `config/plugins/nice_plugin/config.yml`:

```yaml
messages:
  no_permissions: ${Prefix} You do not have the permissions.

token: ${TOKEN}
```

</details>

### Special Variables

These variables are also present when building:

- `SERVER_NAME`: name property from server.toml
- `SERVER_VERSION`: mc_version property from server.toml

When exporting to [mrpack](#mcman-export-mrpack-filename) or [packwiz](#mcman-export-packwiz), these variables from `server.toml` are used:

| Variable Name     | mrpack - `modrinth.index.json` | packwiz - `pack.toml` |
| :---------------- | :----------------------------- | :-------------------- |
| `MODPACK_NAME`    | `name`                         | `name`                |
| `MODPACK_SUMMARY` | `summary`                      | `description`         |
| `MODPACK_AUTHORS` | *nothing*                      | `author`              |
| `MODPACK_VERSION` | *nothing*                      | `version`             |

---

## server.toml

This toml file defines your server.

```toml
name = "My Server"
mc_version = "1.20.1"

[jar]
type = "vanilla"

# --- snip ---
```

**Fields:**

| Name | Type | Description |
| --- | --- | --- |
| `name` | string | Name of the server |
| `mc_version` | string/`"latest"` | The minecraft version of the server |
| `jar` | [Downloadable](#downloadable) | Which server software to use |
| `launcher` | [ServerLauncher](#server-launcher) | Options for generating launch scripts |
| `plugins` | [Downloadable[]](#downloadable) | A list of plugins to download |
| `mods` | [Downloadable[]](#downloadable) | A list of mods to download |
| `clientsidemods` | [ClientSideMod[]](#clientsidemod) | A list of client-side only mods, for packwiz/mrpack support |
| `variables` | table { string: string } | See [variables](#variables) section |
| `worlds` | table { string: [World](#world) } | A table of worlds which may contain datapacks. The keys are the world names |
| `markdown` | [MarkdownOptions](#markdown-options) | Options for markdown files, see [here](#markdown-options) for more info |

### World

> Added in v0.2.2

Represents a world in your server. Currently only exists for datapack support.

```toml
[worlds.skyblock]
datapacks = []
```

**Fields:**

- `datapacks`: [Downloadable[]](#downloadable) - The list of datapacks to download for this world

### Server Launcher

The `[launcher]` table lets mcman create launch scripts for you.

Default values aren't written back to config - except for `aikars_flags`, `proxy_flags` and `eula_args` which are always written.

<details>
<summary>
Example ServerLauncher with comments
</summary>

```toml
[launcher]
# disables generating launch scripts completely
disable = false # false by default

# adds your own args
jvm_args = "-exampleidk"
game_args = "--world abc"

# use aikar's flags
# these do optimizations, see flags.sh for more info
aikars_flags = true

# like aikar's, but for proxies (bungeecord, waterfall, velocity)
proxy_flags = false

# adds -Dcom.mojang.eula.agree=true
# therefore you agree to mojang's eula
# writes eula.txt when on fabric/quilt
eula_args = true

# adds --nogui to game args
nogui = true

# specify -Xmx/-Xms (memory)
memory = "2048M"

# a table of properties
[launcher.properties]
hello="thing"
# ^ same as this:
# jvm_args = "-Dhello=thing"
```

</details>

**Fields:**

| Name | Type | Description |
| --- | --- | --- |
| `disable` | bool | Disables script generation altogether |
| `nogui` | bool | Adds `--nogui` at the end |
| `aikars_flags` | bool | Use aikars flags <sup>[flags.sh](https://flags.sh)</sup> |
| `proxy_flags` | bool | Use proxy flags <sup>[flags.sh](https://flags.sh)</sup> |
| `jvm_args` | string | Custom jvm args (before `-jar serv.jar`) |
| `game_args` | string | Custom game args (after `-jar serv.jar`) |
| `memory` | string | How much memory to give (`-Xmx`/`-Xms`), example: `"2048M"` |
| `properties` | table { string: string } | sets `-D`-prefixed system property jvm args |

### Markdown Options

This category contains the options for markdown rendering via [`mcman md`](#mcman-markdown)

**Fields:**

- `files`: string[] - list of files to render
- `auto_update`: bool - weather to auto-update the files on some commands

```toml
# server.toml
[markdown]
files = [
  "README.md",
  "PLUGINS.md",
]
auto_update = false
```

**Markdown Templates:**

These are the templates mcman will render inside your markdown files.

<details>
<summary>
Server Info Table
</summary>

This template renders a table with server jar info.

```md
<!--start:mcman-server-->
... content ...
<!--end:mcman-server-->
```

Example render:

> | Version | Type                                       | Build    |
> | ------- | ------------------------------------------ | -------- |
> | 1.20.1  | [Paper](https://papermc.io/software/paper) | *Latest* |

</details>

<details>
<summary>
Addons List
</summary>

This template renders a list of addons (plugins or mods)

```md
<!--start:mcman-addons-->
... content ...
<!--end:mcman-addons-->
```

Example render:

> | Name | Description |
> | --- | --- |
> | [BlueMap](https://modrinth.com/plugin/bluemap) |  A Minecraft mapping tool that creates 3D models of your Minecraft worlds and displays them in a web viewer. |
> | [FastAsyncWorldEdit](https://modrinth.com/plugin/fastasyncworldedit) | Blazingly fast world manipulation for artists, builders and everyone else |

</details>

### ClientSideMod

This is basically a [Downloadable](#downloadable) with some extra fields:

| Name | Type | Description |
| --- | --- | --- |
| `optional` | bool | Marks if optional or not |
| `desc` | string | Provide a description |

These fields are used for exporting to [mrpack](#mcman-export-mrpack-filename)/[packwiz](#mcman-export-packwiz)

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
