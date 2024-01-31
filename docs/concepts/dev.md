# Development Mode

`mcman dev` is a command that allows you to configure your server more efficiently.

## What does it do?

Development Mode does a few things - mostly watching files for changes.

It watches:

- `server.toml`, and rebuilds the server when you change it
- the `config/**` directory
- and `hotreload.toml`

## Actions for when files change

The `hotreload.toml` file allows you to create **actions** to execute when some files under `config/` change after they have been automatically [bootstrapped](./variables.md).

``` toml title="hotreload.toml"
[[files]]
path = "server.properties"
action = "reload"

[[files]]
path = "plugins/Something/**"
action = "/something reload"
```

For every file entry, define `path` as a glob pattern to match files.

And for `action`, you can use

- `"reload"` to reload the server
- `"restart"` to rebuild the server
- and any value starting with `/` to send commands, for example: `"/say hello"`

