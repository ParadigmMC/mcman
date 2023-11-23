# Development Sessions (Hot reloading)

If you need to iterate on your server or its configuration files, this feature is for you. Hot reloading allows you to develop your server more efficiently.

## How to use

To start a development session, use the [`mcman dev`](../commands/dev.md) command. This will first build your server then run it while also watching for file changes.

While the development session is active, when you modify:

- `server.toml` -> Server gets rebuilt
- `config/*` -> File gets bootstrapped
- `hotreload.toml` -> Hot reload settings get reloaded

## `hotreload.toml`

When a config file gets changed, after bootstrapping it, mcman will execute any action for the file defined in `hotreload.toml` if present.

For example:

```toml
[[files]]
path = "server.properties"
action = "reload"
```

The path is a glob pattern.

### Hot reload actions

There are currently 3 actions:

- `reload` - Sends a reload command to the server
- `restart` - Restarts the server process
- `/...` - If the action starts with a `/`, it will get interpreted as a command to be sent.

Example of the command action:

```toml
[[files]]
path = "plugins/examplePlugin/*"
action = "/examplePlugin reload"
```
