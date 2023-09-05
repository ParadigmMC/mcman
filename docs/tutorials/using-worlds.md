# Using Worlds

Worlds are an experimental feature added in `0.4.0`

This feature allows you to save, load or download worlds while building.

## Downloading Worlds

In [`server.toml`](../reference/server.toml.md) under `worlds` you can use the [`World`](../reference/world.md) type to specify a download.

!!! example
    ```toml
    [worlds.lobby.download]
    type = "url"
    url = "..."
    ```

As you can see, the `download` property is just a [`Downloadable`](../reference/downloadable/index.md), so you can go crazy and use... github [releases](https://github.com/ModFest/bc23-pack/releases/tag/world)? Im not the one to judge.

## Packing a world

'Packing' is used to refer to zipping the world contents into a zip file and putting it under the `worlds/` directory.

Since this is an experimental feature, you unfortunately need to do this manually.

!!! note
    The world files shouldnt be inside a folder inside the zip file, rather, be directly inside the zip archive.

## Unpacking a world

When building, if the [world](../reference/world.md) is in `worlds` inside [`server.toml`](../reference/server.toml.md) **and** the world does not exist in `server/`, mcman will automatically unpack it.

!!! example
    ```toml
    [worlds.'my-world']
    # just the key existing is enough
    ```

To manually unpack the world to `server/`, use:

```
mcman world unpack <name>
```
