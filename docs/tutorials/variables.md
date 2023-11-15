# Variables and Bootstrapping

In `server.toml`, you can define variables like so:

```toml
[variables]
hello = "world"
# define variables here
```

When you build your server, any config file with common extensions (`yml`, `json`, `toml`, `txt` etc.) will be **bootstrapped** - their contents will be replaced with variables using the variable syntax.

Bootstrapping is essentially copying the file and doing a complex find-and-replace.

The syntax for variables are `${name}` where `name` is the name of the variable. A colon can be used to set a default value: `${MOTD:Hi, im a Minecraft Server!}`

??? "Using environment variables"
    If your variables are sensitive (such as discord bot tokens) you can use environment variables:

    === "Linux"
        ```bash
        export TOKEN=asdf
        ```

    === "Windows"
        ```bat
        set TOKEN=asdf
        ```

    Environment variables can be accessed just like other variables.

## Examples

Environment variable: `TOKEN=asdf`

`server.toml`

```toml
name = "funnies"

[variables]
PORT = "25500"
MOTD = "welcome to funnies"
WEBSITE = "https://example.com/"
Prefix = "[funnies]"
```

`server.properties`

=== "📜 config/server.properties"
    ```properties
    server-port=${PORT:25565}
    gamemode=creative
    motd=${MOTD}
    online-mode=false
    ```

=== "➡️ server/server.properties"
    ```properties
    server-port=25500
    gamemode=creative
    motd=welcome to funnies
    online-mode=false
    ```

`plugins/someplugin/config.yml`

=== "📜 config/..."
    ```yaml
    bossbar: "${SERVER_NAME} - ${WEBSITE}"
    messages:
        no_permissions: ${Prefix} You do not have the permissions.

    token: ${TOKEN}
    ```

=== "➡️ server/..."
    ```yaml
    bossbar: "funnies - https://example.com/"
    messages:
        no_permissions: [funnies] You do not have the permissions.

    token: asdf
    ```

## Network Variables

See [Networks/Variables](./network.md#variables) for more info.

## Special Variables

There are some special variables:

- `SERVER_NAME`: `name` property from `server.toml`
- `SERVER_VERSION`: `mc_version` property from `server.toml`
- `SERVER_PORT` and `SERVER_IP`: See [Networks/Variables](./network.md)
- `PLUGIN_COUNT`/`MOD_COUNT`: the number of plugins or mods
- `WORLD_COUNT`: the number of defined worlds
- `CLIENTSIDE_MOD_COUNT`: the number of client-side mods

There are also some special variables for [network](./network.md)s which can be found [here](./network.md#special-variables)

When exporting to mrpack or packwiz, these variables from `server.toml` are used:

| Variable Name     | mrpack - `modrinth.index.json` | packwiz - `pack.toml` |
| :---------------- | :----------------------------- | :-------------------- |
| `MODPACK_NAME`    | `name`                         | `name`                |
| `MODPACK_SUMMARY` | `summary`                      | `description`         |
| `MODPACK_AUTHORS` | *nothing*                      | `author`              |
| `MODPACK_VERSION` | *nothing*                      | `version`             |

:3
