# Modrinth

Downloads a mod, plugin or a datapack from [Modrinth](https://modrinth.com/)'s API

!!! example
    ```toml
    type = "modrinth" #(1)!
    id = "coreprotect"
    version = "mvLpRWww" #(2)!
    ```

    1. You can also use `mr` as an alias
    2. You can find the version in the url of the download link or the version page.

        The 'version number' is also accepted (since mcman 0.3.0)

**Fields:**

| Name      | Type                | Description                                                |
| --------- | ------------------- | ---------------------------------------------------------- |
| `type`    | `"modrinth"`/`"mr"` |                                                            |
| `id`      | string              | The slug or the ID of the project                          |
| `version` | string/`"latest"`   | Version ID or number, `"latest"` not recommended as of now |
