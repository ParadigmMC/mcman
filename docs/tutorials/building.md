# Building, Running and Developing

After editing `server.toml` or writing configuration files to `config/`, you're probably going to want to run the server.

mcman is not designed to handle running the server itself, but provides a `run` and a `dev` command which we'll explain later.

## Building

To start the server, you need the files. The `.jar` files of the server, addons, and other stuff. We call the process of downloading, processing, copying etc. **building**.

Building requires a valid [server.toml](../reference/server.toml.md) file - thats it.

To not conflict with everything else, the build output folder is by default the `server/` directory, next to `server.toml` and `config/`. This folder should already be `.gitignore`d by default.

You can override the output directory with the `--output <path>` option, but this is not recommended.

You can also skip some steps if you want using the `--skip`/`-s` option. Possible values are `plugins`, `mods`, `worlds` and `bootstrap`. If you need to skip multiple stages, stack them up like so: `-s mods -s worlds`

### First steps

`mcman` will try to load a [lockfile](../reference/lockfile.md) if present before beginning the build. This is done to speed up build times and skip unnecesary things. (you can ignore this)

First, mcman will download or install the server jar as defined in `server.toml`'s [`jar`](../reference/servertype/index.md) field.

### Java

Some servers (quilt, forge, neoforge, spigot/bukkit) require **java** to be present while building. This is because those server types use an installer mechanism instead of providing a pre-built `server.jar` file that mcman can download.

If the server type is one that requires installation, you can find the installer's output logs under the output directory. The file name is `.S.mcman.log` where `S` is a short identifier for the installer. (`qsi` for quilt, `bt` for buildtools, `fi` and `nfi` for forge/neoforge respectively)

The installer must exit with a non-zero code or mcman considers it a fail and stops building the server.

### Addons (plugins and mods)

Next, `mcman` will download all the addons according to `server.toml`

By default, most downloaded addons get [cached](./caching.md) in your local system. If a Downloadable is cached, mcman will copy it from cache instead of downloading it again.

If possible, mcman will also do size/hash checks on both downloaded and copied/cached files.

Every addon gets 3 attempts, but this can be [overridden](./options.md#addon-download-attempts)

### Worlds

If there are any worlds specified in `server.toml`, they are processed.

See [Using Worlds](./using-worlds.md) to learn more about worlds in mcman.

Datapacks are also downloaded similar to addons in this stage.

### Bootstrapping

See [Variables aka Bootstrapping](./variables.md)

### Finishing up

Some last touches include:

- Generating `start.sh`/`start.bat` scripts (can be disabled with `launcher.disable`)
- Creating an `eula.txt` if `launcher.eula_args` is set to true and the server doesn't support the argument

## Running

## Developing


## 4. Configurations (Bootstrapping)

In this stage, mcman will 'bootstrap' your configuration files - which is a fancy synonim for "copy, paste, find and replace"

You can check the [variables](./variables.md) section for more info

## 5. Scripts

Finally, mcman generates `start.bat` and `start.sh` scripts. These can be disabled and configured further under `server.toml` [(docs here)](../reference/server-launcher.md)
