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

    === "Linux"
        ```bash
        export TOKEN=asdf
        ```

    === "Windows"
        ```bat
        set TOKEN=asdf
        ```

    Environment variables are combined with the `#!toml [variables]` field while building.

And then use the variables inside any config file inside `config/`:

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

Variables are then mapped into every configuration file in `config/` to `server/` while [building](./building.md) the server.

## Results in server/

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
