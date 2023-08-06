# Github Releases

Download something from GitHub Releases

!!! example
    ```toml
    type = "ghrel"
    repo = "ViaVersion/ViaVersion"
    tag = "4.7.0"
    asset = "ViaVersion-${tag}.jar" #(1)!
    ```

    1. The real asset name is `ViaVersion-4.7.0.jar`.

!!! note
    The strings can contain variable syntax:

    - `${mcver}` or `${mcversion}` for the `mc_version` in [server.toml](../server.toml.md) (usable in `tag` and `asset`)
    - `${tag}`, `${release}` or ${version} for the resolved github release version (usable in `asset`)

!!! note
    For the `asset` field, its first checked if the given asset exists on the release. If it doesn't, it will pick the first asset whose filename contains the `asset` value

**Fields:**

| Name    | Type              | Description                                             |
| ------- | ----------------- | ------------------------------------------------------- |
| `type`  | `"ghrel"`         |                                                         |
| `repo`  | string            | Repository with its owner, like `"ParadigmMC/mcman"`    |
| `tag`   | string/`"latest"` | The 'tag' (version number in most cases) of the release |
| `asset` | string/`"first"`  | The name of the asset                                   |
