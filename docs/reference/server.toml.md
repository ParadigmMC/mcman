# server.toml

This section is for the `server.toml` configuration file.

Each `server.toml` defines a differient server.

!!! note
    You shouldn't nest `server.toml` files in sub-folders.
    
    If you are in your server's sub-directories, mcman will be able to find the `server.toml` file recursively.

??? example
    ``` toml title="server.toml"
    name = "My SMP"
    mc_version = "1.20.1"

    [jar]
    type = "purpur"
    ```

**Fields:**

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
