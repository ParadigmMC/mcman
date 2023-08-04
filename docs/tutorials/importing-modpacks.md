# Importing Modpacks

`mcman` can import from the [mrpack](https://modrinth.com/modpacks) format (modrinth modpacks) or [packwiz](https://packwiz.infra.link/) packs.

??? question "Can I import after initializing?"
    Yes you can!

    - For mrpacks: `mcman import mrpack <source>`
    - For packwiz packs: `mcman import packwiz <source>`

    The source arguments are the same

## mrpack

You can import mrpacks with the `--mrpack` flag while initializing:

```
mcman init --mrpack <source>
```

The `source` argument can be

* An URL to a direct download of the `.mrpack` file
* A local path to the `.mrpack` file
* A modpack from [Modrinth](https://modrinth.com/modpacks) prefixed with "`mr:`"
    * For example, the modpack [Packed](https://modrinth.com/modpack/packed) would be written as "`mr:packed`"

## packwiz

Like mrpacks, you can import while initializing with:

```
mcman init --packwiz <source>
```

The `source` argument can be

* An URL with `http`/`https` scheme
* Path to a local `pack.toml` file

## Whats next?

Tutorial -> Getting Started -> [Building](./getting-started.md#building)
