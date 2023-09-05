# BuildTools

Setup Spigot or CraftBukkit using [BuildTools](https://www.spigotmc.org/wiki/buildtools/).

!!! note
    `mcman` will need to run `java` to install the server, ensure it exists in the environment before building

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
