# Building

Normally, in your ordinary Minecraft server, **everything** (including your server jar, plugins, config files yada yada) is in the **"main"** directory.

`mcman` does things differiently - that folder with bunch of jar files, configs and worlds are all under the **`server/`** folder.

Why? This makes sure your files are fully local and not uploaded to git, and makes the file structure much cleaner. You want to fully wipe the server data? Just delete `server/`!

## What is 'Building'?

If you just created a `server.toml` file or are looking in a git repository, you wont see any `server/` folder.

**Building** refers to `mcman` doing all the work for you, i.e. downloading jars, to give you your own local `server/` folder with a whole Minecraft server ready to run!

## How do I build the server?

You use the **`mcman build`** command.

## I'm tired of calling 'start.sh'

There is a convenient command for you then! You can use `mcman run` to *build and run* your server.

## Is there a way I can develop my server easier?

Yes! We have a command for that, [read here](./dev.md)!

## Well okay, how does it work?

1. Server jar is downloaded (or installed)
   - Some server types (such as Spigot or Forge) dont have jar files mcman can just download, so it needs to **install** them by running their installers.
2. Addons (plugins and mods) are downloaded
3. Worlds are [unpacked or downloaded](./using-worlds.md) if they dont exist
4. Datapacks are downloaded
5. Files get [bootstrapped](./variables.md) with variables (`config/` -> `server/`)
6. [Launch scripts](../reference/server-launcher.md) are created

