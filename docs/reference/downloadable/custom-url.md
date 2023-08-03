# Custom URL

Download from a direct download link.

!!! example
    ```toml
    type = "url"
    url = "https://example.com/download/Example.jar"
    filename = "example-mod.jar" #(1)
    ```

    1. Optionally define the filename, useful if it cannot be inferred from the url

**Fields:**

| Name       | Type    | Description                                                              |
| ---------- | ------- | ------------------------------------------------------------------------ |
| `type`     | `"url"` |                                                                          |
| `url`      | string  | URL to the file                                                          |
| `filename` | string? | Optional filename if you dont like the name from the url                 |
| `desc`     | string? | Optional description (shown in [markdown](../markdown-options.md)) |
