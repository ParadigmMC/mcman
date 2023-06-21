# `mcman` Documentation

Index:

- [CLI](#cli)
- [Folder Structure](#folder-structure)
- [Variables](#variables)
- [server.toml](#servertoml)
  - [Server Launcher](#server-launcher)
  - [Downloadables](#downloadable)

## CLI

todo!()

## Folder Structure

In a normal server environment, everything is in one folder and a big giant mess to navigate.
And database files are next to config files!

When using `mcman`, your folder structure will look something like this:

- 📂 **cool_server/**
  - 📋 `server.toml`
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

Or, if your variables are sensitive (such as discord bot tokens) you can use environment variables:

```bash
# Linux/Mac:
export TOKEN=asdf
```

```bat
rem Windows:
set TOKEN=asdf
```

And then use the variables inside any config file inside `config/`:

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
bossbar: "${WEBSITE}"
```

📜 `config/plugins/nice_plugin/config.yml`:

```yaml
messages:
  no_permissions: ${Prefix} You do not have the permissions.

token: ${TOKEN}
```

## server.toml

This toml file defines your server.

```toml
# string, the name of your server
name = "My Server"
# string, define the minecraft version
mc_version = "1.20.1"

[jar]
# Downloadable - the server jar
type = "vanilla" # example

[launcher] # table, see more below
# ...

[variables] # table, see the Variables section

[[plugins]] # list of Downloadable
# ...

[[mods]] # list of Downloadable
# ...
```

### Server Launcher

The `[launcher]` table lets mcman create launch scripts for you.

Default values aren't written back to config - except for `aikars_flags`, `proxy_flags` and `eula_args` which are always written.

```toml
[launcher]
# disables generating launch scripts completely
disable = false # false by default

# adds your own args
jvm_args = "-Dhello=true"
game_args = "--world abc"

# use aikar's flags
# these do optimizations, see flags.sh for more info
aikars_flags = true

# like aikar's, but for proxies (bungeecord, waterfall, velocity)
proxy_flags = false

# adds -Dcom.mojang.eula.agree=true
# therefore you agree to mojang's eula
eula_args = true

# adds --nogui to game args, should set to false on proxies...
nogui = true

# specify -Xmx/-Xms (memory)
memory = "2048M"
```

## Types

Below are some types used in `server.toml`

### Downloadable

A downloadable is some source of a plugin, mod or a server jar.

Index of types:

- [Vanilla](#vanilla)
- [PaperMC](#papermc)
- [PurpurMC](#purpurmc)
- [Modrinth](#modrinth)
- [Spigot](#spigot)
- [Github Releases](#github-releases)
- [Jenkins](#jenkins)
- [Custom URL](#custom-url)

#### Vanilla

Used for a vanilla server jar. Has no properties

```toml
type = "vanilla"
```

#### PaperMC

Allows downloading a [PaperMC](https://papermc.io/) project.

**Options:**

- `type` = `papermc`
- `project`: string - The project name
- `build`: string | `"latest"` - Optional

```toml
# Its recommended to use the shortcuts:
type = "paper"
type = "folia"
type = "waterfall"
type = "velocity"

# Or you can use the base instead:
type = "papermc"
project = "paper"

# Optionally define the build if you dont want to use the latest:
type = "papermc"
project = "folia"
build = "17"
# Note: the shortcuts do not support the 'build' property
```

#### PurpurMC

Downloads server jar from [PurpurMC](https://purpurmc.org/).

**Options:**

- `type` = `purpur`
- `build`: string | `"latest"` - Optional

```toml
type = "purpur"

# like paper, can also specify build
build = "10"
# if omitted, uses latest
```

#### Modrinth

Downloads from [Modrinth](https://modrinth.com/)'s API

**Options:**

- `type` = `modrinth` | `mr`
- `id`: string - id of the project or the slug
- `version`: string | `"latest"` - Version ID, `"latest"` not recommended

```toml
[[plugins]]
# you can also use 'mr' as an alias
type = "modrinth"
id = "coreprotect"
version = "mvLpRWww"
```

#### Spigot

This uses [Spiget](https://spiget.org/)'s API.

**Options:**

- `type` = `spigot`
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

- `type` = `ghrel`
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

- `type` = `jenkins`
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

```toml
[[mods]]
type = "url"
url = "https://example.com/download/Example.jar"
# Optionally define the filename, useful if it cannot be inferred from the url
filename = "example-mod.jar"
```
