# Understanding Building

Building is the process of... basically building the entire server.

Before everything else, building requires a [server.toml](../reference/server.toml.md). Check out the [getting started](./getting-started.md) tutorial if you dont have one.

Before everything, mcman will create a `server/` directory to download everything into if it doesnt exist.

## 1. Server Jar

First, mcman will download the server jar. And if neccesary (quilt and buildtools) will run the installer.

## 2. Plugins/Mods

In this stage, mcman downloads every mod and plugin defined in the `server.toml`.

## 3. Datapacks

Like plugins and mods, if there are any, mcman will download every datapack for every world that exists

## 4. Configurations (Bootstrapping)

In this stage, mcman will 'bootstrap' your configuration files - which is a fancy synonim for "copy, paste, find and replace"

You can check the [variables](./variables.md) section for more info

## 5. Scripts

Finally, mcman generates `start.bat` and `start.sh` scripts. These can be disabled and configured further under `server.toml` [(docs here)](../reference/server-launcher.md)
