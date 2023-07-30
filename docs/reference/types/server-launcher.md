# Server Launcher

The `[launcher]` table lets mcman create launch scripts for you while running the [build](../../commands/build.md) command.

Default values aren't written back to config - except for `aikars_flags`, `proxy_flags` and `eula_args` which are always written.

??? example "Example ServerLauncher"

    ``` toml
    [launcher]
    disable = false # (1)

    # (2)
    jvm_args = "-exampleidk"
    game_args = "--world abc"

    aikars_flags = true # (3)
    proxy_flags = false # (4)

    eula_args = true # (5)

    nogui = true # (6)

    memory = "2048M" # (7)

    # (8)
    [launcher.properties]
    hello="thing"
    ```

    1. Disables generating launch scripts completely. `false` by default
    2. If needed, you can add custom arguments here. The format is `java [jvm_args] -jar server.jar [game_args]`
    3. Use aikar's flags - these do optimizations, see [flags.sh](https://flags.sh) for more info
    4. Like aikar's, but for proxies (bungeecord, waterfall, velocity)
    5. Adds `-Dcom.mojang.eula.agree=true` - this flag exists in spigot/paper to ignore `eula.txt`. Writes to `eula.txt` when on fabric or quilt
    6. Adds `--nogui` to game args, disable if its a proxy server as they dont support it
    7. Specify `-Xmx`/`-Xms` (memory) for the server.
    8. A table of properties. This is the same as using
       ``` toml
       jvm_args = "-Dhello=thing"
       ```

**Fields:**

| Name | Type | Description |
| --- | --- | --- |
| `disable` | bool | Disables script generation altogether |
| `nogui` | bool | Adds `--nogui` at the end |
| `aikars_flags` | bool | Use aikars flags <sup>[flags.sh](https://flags.sh)</sup> |
| `proxy_flags` | bool | Use proxy flags <sup>[flags.sh](https://flags.sh)</sup> |
| `jvm_args` | string | Custom jvm args (before `-jar serv.jar`) |
| `game_args` | string | Custom game args (after `-jar serv.jar`) |
| `memory` | string | How much memory to give (`-Xmx`/`-Xms`), example: `"2048M"` |
| `properties` | table { string: string } | sets `-D`-prefixed system property jvm args |
