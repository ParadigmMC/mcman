# mcman

Robust Minecraft Server Framework

<!-- todo: a screenshot here -->

## Getting Started

Check out the [tutorial](./TUTORIAL.md)

## Features

- Automatically copy plugin configurations
  - Supports variables inside config files
- Automatic plugin installation
  - **Supports Plugins From:**
  - [Modrinth](https://modrinth.com/plugins/)
  - [Spigot](https://spigotmc.org/)
  - Custom plugins from URL
  - (todo) github releases
  - (todo) Jenkins
- Automatic server jar downloading and launch script generation
  - **Supports:**
    - Vanilla
    - [PaperMC](https://papermc.io/): paper, folia, velocity, waterfall
    - [PurpurMC](https://purpurmc.org/)
    - Custom jars from URL

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
server-port=${PORT}
gamemode=creative
motd=${MOTD}
online-mode=false
```

You only need to write the values you need.

This also works with YAML configurations. (currently just copies the file, sorry!)

When you run `mcman build`, mcman will bootstrap your server into the `server/` folder. You can use **mcman** to efficiently configure and develop your minecraft servers using git.

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

## Special Thanks

- [flags.sh](https://flags.sh/) for the flags and stuff
- PaperMC and Modrinth for having an amazing API
- You for using our project
