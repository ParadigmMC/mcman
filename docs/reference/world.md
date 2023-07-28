# World

> Added in v0.2.2

Represents a world in your server. Currently only exists for datapack support.

This is a simple type - it only contains a list of [Downloadable](./downloadable)s.

Worlds are indexed by their name in `server.toml`'s `worlds` table.

```toml title="server.toml"
[worlds.skyblock]
datapacks = []
```

**Fields:**

| Name | Type | Description |
| --- | --- | --- |
| `datapacks` | [Downloadable[]](./downloadable) | A list of datapacks to download for this world |
