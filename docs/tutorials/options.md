# Options/Misc

Here are some misc. options of mcman

## Setting the Java binary

If you want `mcman` to use a custom java binary, you can set the environment variable `JAVA_BIN` to its path.

For servers that have the `launcher.java_version` field set, mcman will try to use the `JAVA_*_BIN` environment first before `JAVA_BIN`.

For example, for

```toml
[launcher]
java_version = "16"
```

`mcman` will first check for `JAVA_16_BIN`, then `JAVA_BIN` and if both aren't set, `"java"` will be used as default.

## Disabling lockfiles

To disable [Lockfile](../reference/lockfile.md)s, you can set the `MCMAN_DISABLE_LOCKFILE` environment variable to `true`.

## Addon download attempts

By default, addons get 3 tries to be downloaded. To change this, set the `MAX_TRIES` environment variable to the max amount of tries. For example, set it to `1` if you want `mcman` to try only once.

## Overriding server ports in networks

See the note on [this section](./network.md#special-variables)
