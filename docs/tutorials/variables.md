# Variables

In your configuration files inside `config/`, you can use variables defined in `server.toml`:

```toml title="📋 server.toml"
name = "funnies"
mc_version = "1.20"

[variables]
PORT = "25500"
MOTD = "welcome to funnies"
WEBSITE = "https://example.com/"
Prefix = "[funnies]"
# key-value table
```

??? "Using environment variables"
    If your variables are sensitive (such as discord bot tokens) you can use environment variables:

    ```bash title="Linux/Mac"
    export TOKEN=asdf
    ```

    ```bat title="Windows"
    set TOKEN=asdf
    ```

    Environment variables are also put onto config files.

And then use the variables inside any config file inside `config/`:

??? "Example configuration files"
    ```properties title="📜 config/server.properties"
    # use a colon (:) to provide defaults inside configs
    server-port=${PORT:25565}
    gamemode=creative
    motd=${MOTD}
    online-mode=false
    ```

    ```yaml title="📜 config/plugins/someplugin/config.yml"
    bossbar: "${SERVER_NAME} - ${WEBSITE}"
    messages:
        no_permissions: ${Prefix} You do not have the permissions.

    token: ${TOKEN}
    ```

Variables are then mapped into every configuration file in `config/` to `server/` while [building](../commands/build.md) the server.

??? "Results after `mcman build` with the config files above"
    ```properties title="📜 server/server.properties"
    # use a colon (:) to provide defaults inside configs
    server-port=25500
    gamemode=creative
    motd=welcome to funnies
    online-mode=false
    ```

    ```yaml title="📜 server/plugins/someplugin/config.yml"
    bossbar: "funnies - https://example.com/"
    messages:
        no_permissions: [funnies] You do not have the permissions.

    token: asdf
    ```

## Special Variables

These variables are also present when building:

- `SERVER_NAME`: name property from server.toml
- `SERVER_VERSION`: mc_version property from server.toml

When exporting to mrpack or packwiz, these variables from `server.toml` are used:

| Variable Name     | mrpack - `modrinth.index.json` | packwiz - `pack.toml` |
| :---------------- | :----------------------------- | :-------------------- |
| `MODPACK_NAME`    | `name`                         | `name`                |
| `MODPACK_SUMMARY` | `summary`                      | `description`         |
| `MODPACK_AUTHORS` | *nothing*                      | `author`              |
| `MODPACK_VERSION` | *nothing*                      | `version`             |
