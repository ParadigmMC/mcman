# `mcman export ...`

Exporting commands

## `mcman export mrpack [filename]`

Export the server as an `mrpack` (modrinth modpack) file

If `[filename]` argument isn't given, it will be exported as `${SERVER_NAME}.mrpack`

See also: [special variables](#special-variables) that contain export-related variables

## `mcman export packwiz`

> **Alias & Full Command:** `mcman export pw [-o --output <FOLDER>] [--cfcdn]`

Export the server as a packwiz pack, by default to `pack/` folder.

If you are in a git repo, mcman will give you the githack url to the generated `pack.toml` at the end of the export.

??? "Extra options (output & cfcdn)"
    You can use the `--output <folder>` option to set a custom destination to the pack.

    Using `mcman export pw --output packwiz/pack` will create `pack.toml` to `./packwiz/pack/pack.toml`

    If the `--cfcdn` flag is used, every `curserinth` downloadable will use `download.mode = "url"` with `download.url` being the url from curseforge's cdn.

    If its not used, `download.mode = "metadata:curseforge"` is used with `update.curseforge = { .. }` (default packwiz behavior)

See also: [special variables](#special-variables) that contain export-related variables
