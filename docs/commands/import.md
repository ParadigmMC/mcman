# `mcman import ...`

> Alias: `mcman i ...`

Commands related to importing

## `mcman import url <URL>`

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

## `mcman import datapack <URL>`

Like [import url](#mcman-import-url-url), but imports as a datapack rather than a plugin or a mod.

Example usage:

```sh
# datapack alias is dp
mcman import dp https://modrinth.com/plugin/tectonic
```

## `mcman import mrpack <src>`

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

## `mcman import packwiz <src>`

> Alias: `mcman i pw <src>`

Imports a [packwiz](https://packwiz.infra.link/) pack

!!! note
    [`mcman init`](./init.md) supports initializing with packwiz

The source can be:

- A packwiz pack URL
- A local file path to `pack.toml`

Example usages:

```sh
mcman import packwiz https://raw.githack.com/ParadigmMC/mcman-example-quilt/main/pack/pack.toml
mcman import packwiz ../pack.toml
```
