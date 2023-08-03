# ClientSideMod

This is basically a [Downloadable](./downloadable) of any type with some extra fields:

| Name | Type | Description |
| --- | --- | --- |
| `optional` | bool | Marks if optional or not |
| `desc` | string | Provide a description |

These fields are used for exporting to [mrpack]() or [packwiz]()

!!! example
    ```toml title="server.toml"
    [[clientsidemods]]
    type = "modrinth"
    id = "3dskinlayers"
    version = "JHapWF9O"
    optional = true
    desc = "It adds 3D skin layers :moyai:"
    ```
