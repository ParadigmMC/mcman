# Server (`server.toml`)

Each `server.toml` file defines a differient server.

To generate one, you can use the [`mcman init`](../commands/init.md) command (see [Getting Started](../tutorials/getting-started.md))

!!! note
    If you are in your server's sub-directories, mcman will be able to find the `server.toml` file recursively.

```toml
name = "My SMP"
mc_version = "1.20.1"

[jar]
type = "purpur"
```

## Fields

### `name`: String

The name field defines the name of the server. It's recommended to be alphanumeric because of the other features using this field.

For example, to [overwrite](../tutorials/options.md#overriding-server-ports-in-networks) the `SERVER_PORT` variable, you can use the `PORT_name` environment variable where `name` is the server's name.

The [`SERVER_NAME` variable](../tutorials/variables.md#special-variables) can be used to access this field.

### `mc_version`: String

The mc_version field is used in many ways, most notably:

- Selecting the server jar
- Filtering addon versions

This field can also be accessed using the `SERVER_VERSION`, `mcversion` or `mcver` variables.

### `jar`: Downloadable



### `launcher`: Server Launcher

This field is used to configure or disable the generated `start.bat` and `start.sh` scripts.

The order of these arguments are:

`java [jvm_args] [memory] [preset] [eula] [properties] <startup> [nogui] [game_args]`

Where `startup` is either `-jar *.jar` or some library shenanigans (NeoForge/Forge require this).

`launcher.disable`

:   Set to `true` if you do not want `start.bat` or `start.sh` generated.

`launcher.eula_args`

:   Bukkit/Spigot forks such as Paper and Purpur all support the `-Dcom.mojang.eula.agree=true` flag which allows the agreement of eula without `eula.txt`. If this is set to true, mcman will add this flag to the arguments. If the server software does not support this argument, `eula=true` will be written to `eula.txt`. 

`launcher.nogui`

:   Adds `--nogui` to the end

`launcher.preset_flags`

:   The preset flags to use. Available preset flags:

    - `aikars`: The famous Aikar's flags, mostly used by Paper-like servers but probably also effective in modded servers.
    - `proxy`: These flags were optimized for proxy servers such as bungeecord, waterfall or velocity.
    - `none`: (default) Dont use any preset flags

    These flags were generously borrowed from [flags.sh](https://flags.sh/)

`launcher.java_version`

:   This field does not add any arguments to the startup command but rather helps mcman decide which java binary to use.

    ```toml
    [launcher]
    java_version = "17"
    ```

    See [this section](../tutorials/options.md#setting-the-java-binary) for more information.

`launcher.jvm_args` and `launcher.game_args`

:   These optional fields allow you to add your own custom arguments.

`launcher.memory`

:   If set, this controls the `-Xmx`/`-Xms` arguments. The value is the same as those arguments, for example:

    ```toml
    [launcher]
    memory = "4G"
    ```

    However you can override this using the `MC_MEMORY` environment variable.

`launcher.properties`

:   This table allows you to write property arguments such as `-Dcom.mojang.eula.agree=true`:

    ```toml
    [launcher.properties]
    "com.mojang.eula.agree" = "true"
    ```

### `plugins`/`mods`: Downloadable[]
### `clientsidemods`: ClientSideMod[]
### `variables`: Table of Strings
### `worlds`: Table of String to World
### `markdown`: Markdown Options
### `options`: Server Options

| Name             | Type                                        | Description                                                                    |
| ---------------- | ------------------------------------------- | ------------------------------------------------------------------------------ |
| `name`           | string                                      | Name of the server                                                             |
| `mc_version`     | string/`"latest"`                           | The minecraft version of the server                                            |
| `jar`            | [Downloadable](./downloadable/index.md)     | Which server software to use                                                   |
| `launcher`       | [ServerLauncher](./server-launcher)   | Options for generating launch scripts                                          |
| `plugins`        | [Downloadable[]](./downloadable/index.md)   | A list of plugins to download                                                  |
| `mods`           | [Downloadable[]](./downloadable/index.md)   | A list of mods to download                                                     |
| `clientsidemods` | [ClientSideMod[]](./clientsidemod)    | A list of client-side only mods, for packwiz/mrpack support                    |
| `variables`      | table of string                             | See [variables](../tutorials/variables) section                                |
| `worlds`         | table of [World](./world)             | A table of worlds which may contain datapacks. The keys are the world names    |
| `markdown`       | [MarkdownOptions](./markdown-options) | Options for markdown files, see [here](./markdown-options) for more info |
