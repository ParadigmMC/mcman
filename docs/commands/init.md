# `mcman init`

Initializes a new server in the current directory.

This command is interactive. Just run `mcman init`!

See the [getting started](../tutorials/getting-started.md) tutorial for what to do next

> **Full Command:** `mcman init [--name <name>] [--mrpack <source> | --packwiz <source>]`

??? "📦 Importing from a mrpack (modrinth modpack)"
    You can use the `--mrpack` flag on `mcman init` to import from an mrpack while initializing a server.

    - If its from modrinth, like [adrenaserver](https://modrinth.com/modpack/adrenaserver): `mcman init --mrpack mr:adrenaserver`

    Use `mr:` and then the project id/slug of the modpack (should be visible on the url)

    - You can also just paste in the modpack page's url: `mcman init --mrpack https://modrinth.com/modpack/adrenaserver`

    - If its from another source, you can provide a download link to it: `mcman init --mrpack https://example.com/pack.mrpack`

    - If its a file: `mcman init --mrpack ../modpacks/pack.mrpack`

    If your server is already initialized, use the `mcman import mrpack <source>` command. The source argument also accepts the sources defined above.

    Example using [Adrenaserver](https://modrinth.com/modpack/adrenaserver):

    ```sh
    # these are all identical
    mcman init --mrpack mr:adrenaserver
    mcman init --mrpack https://modrinth.com/modpack/adrenaserver
    mcman init --mrpack https://cdn.modrinth.com/data/H9OFWiay/versions/2WXUgVhc/Adrenaserver-1.4.0%2B1.20.1.quilt.mrpack
    ```

??? "📦 Importing from a packwiz pack"
    You can use the `--packwiz` (alias `--pw`) flag on `mcman init` to import a packwiz pack while initializing.

    **If the pack is in your filesystem**:

    ```sh
    mcman init --pw path/to/pack.toml
    ```

    **If the pack is online**:

    ```sh
    mcman init --pw https://raw.githack.com/EXAMPLE/EXAMPLE/main/pack.toml
    ```

    If your server is already initialized, use the `mcman import packwiz <source>` command. The source argument also accepts the sources defined above.

??? question "I dont see a Dockerfile/.gitignore"
    If mcman can't detect a git repository, it wont write a `.gitignore`

    The same applies for `Dockerfile` when `docker --version` fails.

    You can use [`mcman env`](./env.md) to get those files.
