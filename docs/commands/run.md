# `mcman run`

See [here](../concepts/building.md) for more info

Supports the same arguments as [mcman build](./build.md)

Builds the server and runs it. This is kind of the same as running `mcman build && cd server && start`

## `mcman run --test`

You can use the `--test` option to test if your server works. mcman will build and run the server and see if it fully starts up. If it crashes, stops, or doesnt succeed, mcman will report the issue and exit with code `1`.

If `options.upload_to_mclogs` is `true` in `server.toml`, mcman will upload `latest.log` and the crash log (if it crashed) to [mclo.gs](https://mclo.gs/) and print the URL to the console.

You can use CI/CD to test if your server works. For example, [this](https://github.com/ParadigmMC/mcman-bc23/blob/1938a567a2324607d816f17481e49c922af1ed87/.github/workflows/bc23test.yml) is a github workflow that tests if the BlanketCon 23 server boots up successfully.

mcman's criteria for a "successful boot" is this line:

```
[12:57:24] [Server thread/INFO]: Done (5.290s)! For help, type "help"
```
