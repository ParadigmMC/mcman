# `mcman build`

See [here](../tutorials/building.md) for more info

??? "Extra flags (output, skip, force)"
    You can alternatively set the output folder manually using `--output <path>` option.

    The `--force` flag can be used to disable checking if files exist, effectively forcefully downloading everything.

    You can use the `--skip <stages>` flag to skip stages.

    - Stages should be comma-seperated, like `--skip bootstrap,scripts,dp`
    - The stages are: `serverjar`, `plugins`, `mods`, `dp` (datapacks), `bootstrap` (config/) and `scripts`

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
