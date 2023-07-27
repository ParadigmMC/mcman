<!-- markdownlint-disable MD033 -->
[latest-win]: https://github.com/ParadigmMC/mcman/releases/latest/download/mcman.exe
[latest-linux]: https://github.com/ParadigmMC/mcman/releases/latest/download/mcman

# Getting Started

**Index:**

- [Installation](#installation)
- [Recommended Usage](#recommended-usage)
- [Using configuration files](#using-configuration-files)

## Installation

**Stable Releases:**

| [Windows][latest-win] | [OSX/Linux][latest-linux] |
| :-------------------: | :-----------------------: |

- [Github Releases](https://github.com/ParadigmMC/mcman/releases)
- [build action](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml) (requires github account)

### Windows: Scoop

[Scoop](https://scoop.sh/) is a command-line installer for Windows. You can use 2 commands in powershell to install it.

Add the [minecraft](https://github.com/The-Simples/scoop-minecraft) bucket and install mcman:

```powershell
scoop bucket add minecraft https://github.com/The-Simples/scoop-minecraft
scoop install mcman
```

## Recommended Usage

First, initialize a github repository for your server and clone it. It can be private too if you want.

After cloning the repository, go into the folder and run `mcman init`. This command will interactively set your server's basic configuration (name, version, server software etc).

After initializing your server or when you change things in your server files (like `server.toml`) you can commit your changes into the git repository. mcman will provide a `.gitignore` which will prevent you from accidentally commiting the output folder (`server/`)

<details>
<summary>
📦 Importing from a mrpack (modrinth modpack)
</summary>

You can use the `--mrpack` flag on `mcman init` to import from an mrpack while initializing a server.

- If its from modrinth, like [adrenaserver](https://modrinth.com/modpack/adrenaserver): `mcman init --mrpack mr:adrenaserver`

Use `mr:` and then the project id/slug of the modpack (should be visible on the url)

- You can also just paste in the modpack page's url: `mcman init --mrpack https://modrinth.com/modpack/adrenaserver`

- If its from another source, you can provide a download link to it: `mcman init --mrpack https://example.com/pack.mrpack`

- If its a file: `mcman init --mrpack ../modpacks/pack.mrpack`

If your server is already initialized, use the `mcman import mrpack <source>` command. The source argument also accepts the sources defined above.
</details>

<details>
<summary>
🧵 Proxies (velocity, waterfall, bungeecord)
</summary>

Yes, you can use proxies with mcman. Just select "proxy server" while running `mcman init`
</details>

After initializing the server, you'll see a `server.toml` file - thats the configuration for your server. It contains:

- The minecraft version of your server
- Which server software you are using (quilt, paper, vanilla, etc...)
- Plugins, mods, etc
- and some more configurations

**How do i run my server?**

Its pretty simple. To build your server, run `mcman build`. It will configure everything, download your plugins, mods and sets up your configuration files.

The build command will build the server into the `server/` directory.

If you havent disabled [launch script generation](./DOCS.md#server-launcher) mcman will also generate `server/start.bat` and `server/start.sh` for you. So you can use:

```sh
mcman build && cd server && start
```

## Usage with Docker

After initialization mcman also provides a default dockerfile for you, this dockerfile basically runs your server after running `mcman build`.

## Using configuration files

While running a minecraft server, 99% of the time you have to edit the configuration files of the server. **mcman** can actually help you with that.

Next to your `server.toml` file, you'll see a `config/` folder. It's actually pretty simple. **mcman** will copy files from `config/` to `server/` while building your server (after installing the server, plugins/mods/datapacks etc.)

Actually, mcman doesn't *just* copy the files. It has **variables** too - you can use variables inside your configuration files. This is very useful if you want to have something (like the server name) in multiple places - if you want to change it, you can just change the variable in `server.toml` (instead of manually going through every file that contains it)

> Note
> `config/server.properties` should already be present after mcman init, so pretend it doesn't exist for now

For an example, let's create a `server.properties` file in `config/` and fill it like so:

```properties
motd=${MESSAGE}
```

And add this to `server.toml`:

```toml
[variables]
MESSAGE = "Hello from server.toml!"
```

After you run `mcman build`, you can see that `server/server.properties` is like this:

```properties
motd=Hello from server.toml!
```

You can read more about [variables](./DOCS.md#variables) here.

**Tip:** You can 'pull' a config file from `server/` to `config/` with the [`mcman pull`](./DOCS.md#mcman-pull-file) command.
