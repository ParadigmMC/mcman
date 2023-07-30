# BuildTools

Setup Spigot, Bukkit or CraftBukkit using [BuildTools](https://www.spigotmc.org/wiki/buildtools/).

!!! example
    ```toml
    type = "buildtools"
    args = []
    ```

**Fields:**

| Name   | Type           | Description                                                                               |
| ------ | -------------- | ----------------------------------------------------------------------------------------- |
| `type` | `"buildtools"` |                                                                                           |
| `args` | string[]       | Additional args, such as `["--compile", "bukkit"]` - mcman only adds `--rev {mc_version}` |
