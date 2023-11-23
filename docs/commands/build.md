# `mcman build`

See the page for [Building](../tutorials/building.md) for more info

## `--force`

The `--force` flag can be used to make mcman not skip already downloaded files, basically acting like the output directory is empty.

## `--output <path>`

You can alternatively set the output folder manually using `--output <path>` option. The default is `server`.

## `--skip`/`-s`

You can use the `--skip`/`-s` flag to skip stages.

- Use the flag multiple times to skip many: `-s bootstrap -s worlds`
- The stages are: `plugins`, `mods`, `worlds` and `bootstrap`

## After building

After building, you can start the server with the launch scripts if theyre not [disabled](../reference/server-launcher.md):

=== "Windows"
    ```bat
    cd server
    call start.bat
    ```


=== "Linux"
    ```sh
    cd server
    ./start.sh
    ```
