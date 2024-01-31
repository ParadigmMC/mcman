# Lockfile (`.mcman.lock`)

The lockfile, found under the output directory, is generated after every build. It's in the JSON format and contains metadata about the installed mods, plugins and last dates of config files.

While it's primary purpose is to be a cache and speed up building, it also makes sure that the removed mods/plugins from the `server.toml` file also get their jar files deleted.

## Disabling

See [Options/Disabling lockfiles](../concepts/options.md#disabling-lockfiles)

## Format

```ts
type Lockfile = {
    plugins: [Downloadable, ResolvedFile][],
    mods: [Downloadable, ResolvedFile][],
    files: BootstrappedFile[],
}

type BootstrappedFile = {
    path: string,
    date: Timestamp,
}
```
