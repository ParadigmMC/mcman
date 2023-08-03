# `mcman run`

> Added in 0.3.0

See [here](../tutorials/building.md) for more info

Supports the same arguments as [mcman build](./build.md)

Builds the server and runs it. This is kind of the same as running `mcman build && cd server && start`

!!! abstract Testing
    This command can be used to test if everything works.

    To test, use `mcman run --test`

    When mcman sees something like this in the console
    ```
    [12:57:24] [Server thread/INFO]: Done (5.290s)! For help, type "help"
    ```
    It will mark the test as succeeded unless the server process exits with a non-zero code.

    If a test passes, mcman will exit with 0.
