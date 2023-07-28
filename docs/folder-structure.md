# Folder Structure

In a normal server environment, everything is in one folder and a big giant mess to navigate.
And database files are next to config files!

When using `mcman`, your folder structure will look something like this:

- 📂 **cool_server/**
  - 📋 **`server.toml`**
  - 🚢 `Dockerfile`
  - 📜 `.dockerignore` and `.gitignore`
  - 📁 **config/**
    - 📜 `server.properties`
  - 📁 **server/** (git-ignored - only on your host)
    - ... server env files ...
    - ☕ `server.jar`
    - 📜 `server.properties`
    - 📜 `bukkit`/`spigot`/`paper`/`commands`/`help`/`permissions`/`pufferfish`/`purpur`/`wepif.yml`

Inside the folder for your server, you'll see a few files and folders:

- **`server.toml`**: This is the configuration file for your server, more info [in its own section](#servertoml)
- **config/** folder: This is the folder your server config files should go. `mcman` will process everything into the output.
  - The path is converted as follows:
    `config/server.properties` => `server/server.properties`
    And every config file (.properties, .toml, .yaml, .yml etc) will be processed with [variables](#variables)
- **server/** folder: This folder is where `mcman` will build the server files into, aka the output. This folder is gitignored by default (because why would you do that?)
  - According to the `server.toml`, mcman can generate launcher scripts at `server/start.sh` and `server/start.bat`
- **`.gitignore` and `.dockerignore`**: Ignore files for git and docker
- **Dockerfile**: If you enabled docker, this is the dockerfile