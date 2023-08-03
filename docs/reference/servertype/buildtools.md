# BuildTools

Setup Spigot or CraftBukkit using [BuildTools](https://www.spigotmc.org/wiki/buildtools/).

!!! example
    ```toml
    type = "buildtools"
    software = "craftbukkit"
    ```

**Fields:**

| Name       | Type                          | Description                                              |
| ---------- | ----------------------------- | -------------------------------------------------------- |
| `type`     | `"buildtools"`                |                                                          |
| `software` | `"spigot"` or `"craftbukkit"` | The software to compile                                  |
| `args`     | string[]                      | Additional args. mcman already adds `--rev {mc_version}` |
