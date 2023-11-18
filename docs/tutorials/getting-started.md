# Getting Started

Let's create a simple server as an example.

## Creating a new server

Create a new folder for your server (or create and clone a repository) and run `mcman init`, it will interactively ask you what to name the server, the version, server software and so on.

!!! note "Using git"
    `mcman` is fully git-compatible, meaning you can store, share and version control your server using something like Github.

    This is optional, but recommended since you get many benefits such as rollbacks/backups, collaboration, branches, etc.

    When you run `init`, mcman will try to add some extra entries to `.gitignore` if they are missing.

![mcman init](https://cdn.discordapp.com/attachments/1109215116060266567/1134187743300296815/render1690481729604.gif)

After running `mcman init`, you'll get a few things:

- `server.toml`: This configuration file is the core of your server. It includes things like **server software**, **plugins/mods** and a lot more.
- `config/` folder: This folder will hold all your server configuration files, for example, `server.properties`

## Initializing from a modpack

If you want to create a server from a modpack, you can use the `--mrpack` or `--packwiz` options while using `mcman init`.

- Import from a local file:
    - `mcman init --mrpack modpack.mrpack`
    - `mcman init --pw ../pack.toml`
- Import from an URL:
    - `mcman init --mrpack https://example.com/pack.mrpack`
    - `mcman init --pw https://example.com/pack.toml`
- Import from Modrinth: `mcman init --mrpack mr:modpack-name`

## Building

We call 'downloading and configuring everything' **building**. Before you can run the server, it first needs to be built.

`mcman` is not designed to run your server, but build it.

- You can build the server using the [`mcman build`](../commands/build.md) command.
- If you want `mcman` to run it, use the [`mcman run`](../commands/run.md) command. This is not recommended for production/live servers though.
- There's also the [`mcman dev`](../commands/dev.md) command that starts up a development session with [hot reloading](./dev.md). This command is designed for server administrators to be able to iterate and develop their servers faster/easier.

To learn more about how building works, see [Building, Running and Developing](./building.md)

TLDR: everything gets downloaded and copied to the output folder which is `server/` by default.

## Configuring further

If you open the newly generated `config/server.properties` file, you'll see something like this:

```properties title="config/server.properties"
server-port=${PORT:25565}
motd=${SERVER_NAME:A Minecraft Server}
```

If you build the server, a `server.properties` file inside the output directory (`server/`) will appear. If you open the contents of it, it should look like this:

```properties title="server/server.properties"
server-port=25565
motd=my-server-name
```

As you can guess, when building the server, mcman will process configuration files inside `config/` and copy them over to `server/`. We call this process **bootstrapping**.

Learn more about how bootstrapping works and how to use it [here](./variables.md)

## Adding Plugins or Mods

Your server's plugins or mods will be saved in the `server.toml` file.

For example:

```toml
[[mods]]
type = "modrinth"
id = "create-fabric"
version = "latest"
```

You can either write them manually or use the [`mcman import url <URL>`](../commands/import.md#mcman-import-url-url) command. Every mod or plugin is a [Downloadable](../reference/downloadable/index.md).
