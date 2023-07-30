# Understanding the Folder Structure

In a normal server environment, everything is in one folder and a big giant mess to navigate.
And database files are next to config files!

When using `mcman`, your folder structure will look something like this:

```yaml
cool_server
├─ server.toml
├─ config/
│  └─ ...
└─ server/ #(1)!
   └─ ...
```

1. This folder should be inside `.gitignore`, so you shouldn't see it in most Github repositories.

Inside the folder for your server ('cool_server' in this case), you'll see a few files and folders:

## `server.toml`

This is the configuration file for your server. It contains useful metadata such as:

- What software the server runs on
- What mods or plugins it has
- Additional worlds with datapacks or client-side mods
- Launcher and markdown configurations

These are all can be found under [this section](../reference/server.toml.md) in the reference.

## config/ Directory

Your server's configuration files which you have overridden (edited) should all be here.

When building, mcman uses this folder to 

- **config/** folder: This is the folder your server config files should go. `mcman` will process everything into the output.
  - The path is converted as follows:
    `config/server.properties` => `server/server.properties`
    And every config file (.properties, .toml, .yaml, .yml etc) will be processed with [variables](#variables)
- **server/** folder: This folder is where `mcman` will build the server files into, aka the output. This folder is gitignored by default (because why would you do that?)
  - According to the `server.toml`, mcman can generate launcher scripts at `server/start.sh` and `server/start.bat`
- **`.gitignore` and `.dockerignore`**: Ignore files for git and docker
- **Dockerfile**: If you enabled docker, this is the dockerfile