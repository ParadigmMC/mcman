# Github Releases

Download something from GitHub Releases

!!! example
    ```toml
    type = "ghrel"
    repo = "ViaVersion/ViaVersion"
    tag = "4.7.0"
    asset = "ViaVersion" #(1)!
    ```

    1. The real asset name is `ViaVersion-4.7.0.jar`.

!!! note
    mcman checks for inclusion for the `asset` field, so the first asset with its filename containing `asset` (`"ViaVersion"` in the above example) will get downloaded. Use `"first"` to use the first asset which should be enough for most releases.

**Fields:**

| Name    | Type              | Description                                             |
| ------- | ----------------- | ------------------------------------------------------- |
| `type`  | `"ghrel"`         |                                                         |
| `repo`  | string            | Repository with its owner, like `"ParadigmMC/mcman"`    |
| `tag`   | string/`"latest"` | The 'tag' (version number in most cases) of the release |
| `asset` | string/`"first"`  | The name of the asset (checks for inclusion)            |
