# PaperMC

Downloads a [PaperMC](https://papermc.io/) project.

!!! example
    ```toml title="PaperMC Downloadable"
    type = "papermc"
    project = "waterfall"
    build = "17"
    ```

??? tip "Shortcuts"
    There are also 3 shortcut Downloadable types:

    - `paper`
    - `velocity`
    - `waterfall`

    ```toml title="Example shortcut"
    type = "paper"
    ```

    !!! note
        The shortcuts dont support the `build` property. They are implicitly the latest build.



**Fields:**

| Name      | Type              | Description                     |
| --------- | ----------------- | ------------------------------- |
| `type`    | `"papermc"`       |                                 |
| `project` | string            | The project name                |
| `build`   | string/`"latest"` | Optionally provide the build id |
