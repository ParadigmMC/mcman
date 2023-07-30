# Spigot

Downloads a plugin from [Spiget](https://spiget.org/)'s API.

You can find the ID of the resource in the URL:

> `https://www.spigotmc.org/resources/luckperms.28140/`

In this case, luckperms has the id of `28140` - but you can paste it with the name too:

!!! example
    ```toml title="Download LuckPerms from spigot"
    type = "spigot"
    id = "luckperms.28140"
    ```

!!! tip
    mcman will ignore everything before the dot in the `id` field. This helps with identifying the plugins and should be easier to just copy-paste the id from the URL.

**Fields:**

| Name   | Type       | Description       |
| ------ | ---------- | ----------------- |
| `type` | `"spigot"` |                   |
| `id`   | string     | ID of the project |
