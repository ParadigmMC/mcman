# Using Worlds

mcman can help you manage your server's worlds. You can make mcman [download](#downloading-worlds) a world or keep the world under the [`worlds/`](#the-worlds-folder) directory.

Worlds are defined in [`server.toml`](../reference/server.toml.md) under the `worlds` table. The key of the table is the world name. For example, the world with name 'city' would be `worlds.city`.

## Downloading Worlds

If you set the `download` field to a [Downloadable](../reference/downloadable/index.md) mcman will download the world zip file and unzip it if the world does not exist in the output directory.

```toml
[worlds.earth.download]
type = "url"
url = "https://example.com/cdn/worlds/earth.zip"
```

## The `worlds/` Folder

Optionally, you can store your worlds under the `worlds/` folder.

```yaml
worlds
├─ lobby.zip
└─ arena.zip
```

When building, if the world does not exist in the output directory, mcman will unzip the world file located in the `worlds/` folder.

!!! note
    For your world to be unpacked, there needs to be a world entry in `server.toml` for it:

    ```toml
    [worlds.city]
    # just the entry is enough
    ```

You can also manually unpack a world using the [`mcman unpack <world>`](../commands/world.md) command.
