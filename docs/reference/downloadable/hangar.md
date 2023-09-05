# Hangar

Downloads a plugin from [Hangar](https://hangar.papermc.io/)

!!! example
    ```toml
    type = "hangar"
    id = "kennytv/Maintenance" #(1)!
    version = "latest"
    ```

    1. You can just use the project's slug here too

**Fields:**

| Name      | Type              | Description                                               |
| --------- | ----------------- | --------------------------------------------------------- |
| `type`    | `"hangar"`        |                                                           |
| `id`      | string            | The slug/name of the project                              |
| `version` | string/`"latest"` | Version name, `"latest"` to always use the latest version |
