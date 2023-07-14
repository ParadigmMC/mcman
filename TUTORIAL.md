<!-- markdownlint-disable MD033 -->
[latest-win]: https://github.com/ParadigmMC/mcman/releases/download/latest/mcman.exe
[latest-linux]: https://github.com/ParadigmMC/mcman/releases/download/latest/mcman

# Getting Started

## Installation

**Stable Releases:**

| Windows              | OSX/Linux              |
| :------------------: | :--------------------: |
| [latest][latest-win] | [latest][latest-linux] |

For past releases, go to the [releases](https://github.com/ParadigmMC/mcman/releases) tab.

**Dev Releases:**

We have github [actions](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml) that build mcman.

These require you to be logged in to github.

Please note that these builds might not work completely.

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
