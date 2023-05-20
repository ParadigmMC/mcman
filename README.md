# mcman

Robust Minecraft Server Framework

<!-- todo: a screenshot here -->

## Getting Started

Check out the [tutorial](./TUTORIAL.md)

## Features

- Automatic setting of some values such as port
- Automatic configuration handler
  - Supports variables for even better configs for your servers
- Automatic plugin/mod installation
  - **Supports these sources:**
  - [`modrinth`](https://modrinth.com/)
  - [`spigot`](https://spigotmc.org/)
  - github releases
  - Jenkins
  - Custom URL
- Automatic server jar downloading and launch script generation
  - **Supports:** `vanilla`, `spigot`, `paper`, `folia`, `purpur` and custom forks too
  - **Proxies?:** It has `bungeecord`, `waterfall` and `velocity` too.

## Folder Structure

Dont you hate how everything is a fork of everything so there's a billion config files in your server environment? *mcman fixes that*

- 📂 cool_server/
  - 📋 server.toml
  - 📁 config/
    - 📜 server.properties
  - 📁 server/
    - ... normal bloated server env files ...
    - ☕ server.jar
    - 📜 server.properties
    - 📜 bukkit/spigot/paper/commands/help/permissions/pufferfish/purpur/wepif.yml

And the great part is, here's the contents of 📜 `cool_server/config/server.properties`:

```properties
server-port=25599
gamemode=creative
motd=My cool server!
level-name=custom_map
max-players=50
online-mode=false
```

The rest of the values are kept as-is! So you only have to write the values you need to be changed.

This also works with YAML configurations.

When you run `mcman build`, mcman will bootstrap your server into the `server/` folder. You can use mcman to efficiently configure and develop your minecraft servers using git now.

## Variables

Does your config have secrets? Or do you want to repeat something over multiple places? You can use the variables feature to replace values in config files like so:

📋 **`server.toml`:**

```toml
# ...
[variables]
Prefix = "[MyEpicServer]"
```

For secrets, you can use the `.env` file:

```properties
TOKEN=asdf
```

...Or set your environment variables prefixed with "`CONFIG_`":

- Windows: `set CONFIG_TOKEN=asdf`
- MacOS and Linux: `export CONFIG_TOKEN=asdf`

And then in your config files:

📜 **`config/plugins/nice_plugin/config.yml`:**

```yaml
messages:
  no_permissions: ${Prefix} You do not have the permissions.

token: ${TOKEN}
```
