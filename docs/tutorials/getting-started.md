# Getting Started

Let's create a simple server as an example.

## Initial Setup

Since mcman is git-compatible, you can create a new Github Repository to put your server configuration files in. This is optional but recommended.

Create a new folder (or clone your repository) for your server and `cd` into it.

Then inside your server folder, run [`mcman init`](../commands/init.md)

![mcman init](https://cdn.discordapp.com/attachments/1109215116060266567/1134187743300296815/render1690481729604.gif)

It will interactively allow you to set up a basic **`server.toml`** file.

!!! question "Whats a `server.toml` file????"
    When using mcman, a `server.toml` file is some kind of metadata file containing neccesary information about it. For example, it contains the server name, version, plugins/mods and more.

!!! tip "Want to import from a modpack?"
    mcman supports importing from some modpack formats (`mrpack` and `packwiz`)

    See [this section](./importing-modpacks.md) to see how

## Building

Now, lets 'build' and run the server!

- If you want to run it yourself, use [`mcman build`](../commands/build.md) `&& cd server` and run the `start.{bat,sh}` script.
- Orrr you can just do [`mcman run`](../commands/run.md) which does both for you.

## Bootstrapping

If you open the newly generated `config/server.properties` file, you'll see something like this:

```properties title="config/server.properties"
server-port=${PORT:25565}
motd=${SERVER_NAME:A Minecraft Server}
```

If you run `mcman build`, you should see a `server.properties` file inside the `server/` folder too.

If you open *that* file, inside `server/`, you'll see that it contains these two lines:

```properties title="server/server.properties"
server-port=25565
motd=mcman-example-quilt
```

As you can guess, when running `mcman build`, mcman will process configuration files inside `config/` and copy them over to `server/` alongside downloading the server jar/plugins/mods and such.

For more information, check out the [Variables](./variables.md) section :3

## Adding Plugins or Mods

For now, you can use the [`mcman import url <URL>`](../commands/import.md#mcman-import-url-url) command to import mods or plugins from URLs.

Or alternatively write and edit the [`server.toml`](../reference/server.toml.md) yourself to add it. You can check out the [reference](../reference/downloadable/index.md) for the Downloadable type which is basically a mod/plugin source.
