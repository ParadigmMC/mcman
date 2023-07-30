# Quilt

Downloads [Quilt](https://quiltmc.org/) installer and installs the quilt server.

!!! note
    `mcman` will need to run `java` to install the quilt server jar, ensure it exists in the environment before building

!!! example
    ```toml
    type = "quilt"
    installer = "latest"
    loader = "latest"
    ```

**Fields:**

| Name        | Type                              | Description              |
| ----------- | --------------------------------- | ------------------------ |
| `type`      | `"quilt"`                         |                          |
| `installer` | string/`"latest"`                 | Installer version to use |
| `loader`    | string/`"latest-beta"`/`"latest"` | Loader version to use    |
