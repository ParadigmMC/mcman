# Markdown Options

Ever wanted to display all of the mods or plugins your server has? Using `mcman` you can do that!

If you havent added it, add your markdown file's name (`README.md` for example) into the `markdown.files` list. When the `mcman markdown` command is run, mcman will render every template in the listed files.

``` toml title="server.toml"
[markdown]
files = [
  "README.md",
  "PLUGINS.md",
]
auto_update = false
```

## Fields

`files`: string[]

:   List of filenames to 'render'

`auto_update`: bool

:   If set to `true`, markdown files will be rendered on commands that modify `server.toml`

!!! warning
    If `#!toml auto_update = true`, commands might take longer. We recommend you dont turn it on until you're done adding most of the mods/plugins.

## Markdown Templates

These are the templates mcman will render inside your markdown files. When `mcman markdown` runs, the files specified in `server.toml` will be read and the templates below will be updated with the rendered markdown code. You can have as many markdown files or templates as you want.

### Server Info Table

This template renders a table with server jar info.

``` md title="README.md"
<!--start:mcman-server-->
... content ...
<!--end:mcman-server-->
```

!!! note "Example render:"
    | Version | Type                                       | Build    |
    | ------- | ------------------------------------------ | -------- |
    | 1.20.1  | [Paper](https://papermc.io/software/paper) | *Latest* |

### Addons List

This template renders a list of addons (plugins or mods)

```md title="README.md"
<!--start:mcman-addons-->
... content ...
<!--end:mcman-addons-->
```

!!! note "Example render:"
    | Name | Description |
    | --- | --- |
    | [BlueMap](https://modrinth.com/plugin/bluemap) |  A Minecraft mapping tool that creates 3D models of your Minecraft worlds and displays them in a web viewer. |
    | [FastAsyncWorldEdit](https://modrinth.com/plugin/fastasyncworldedit) | Blazingly fast world manipulation for artists, builders and everyone else |
