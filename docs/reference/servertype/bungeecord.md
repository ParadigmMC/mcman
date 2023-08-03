# BungeeCord

BungeeCord is just a shortcut to a [jenkins](../downloadable/jenkins.md) downloadable:

```toml
type = "bungeecord"
```

!!! note
    If you'd like to get a specific build, use this:

    ```toml
    type = "jenkins"
    url = "https://ci.md-5.net"
    job = "BungeeCord"
    build = "latest" #(1)
    artifact = "BungeeCord"
    ```

    1. Change this to the build id of your choosing
