# `network.toml`

The `network.toml` file defines a Network of multiple servers, such as BungeeCord or Velocity based.

```toml
name = "SuperCraft"
proxy = "proxy"
port = 25565

[servers.lobby]
port = 25566

[servers.game1]
port = 25567

[variables]
MOTD = "Welcome to SuperCraft!"
```

## Folder Structure

The folder structure should look something like this:

```yaml
.
├─ network.toml
└─ servers/
   ├─ server1
   │  └─ server.toml
   └─ server2
      └─ server.toml
```

Have a folder `servers` next to `network.toml` and create a folder for each `server.toml`

## Fields

`name`: string

:   Defines the name of this network. You can access this in config files using `${NETWORK_NAME}`

`proxy`: string

:   Folder name of the proxy server, currently unused.

`port`: number

:   The port this network (or rather its proxy)

`variables`: table

:   This field defines variables global to the network. They can be accessed by prefixing the variable name with `NW_`:

    ``` toml title="network.toml"
    [variables]
    MOTD = "hello world"
    ```

    ``` properties title="servers/game1/server.properties"
    motd=${NW_MOTD}
    ```

`servers`: table of [ServerEntry](#serverentry)

:   In this table, you define your servers the network has

## ServerEntry

```toml
[servers.dennis_smp]
port = 25566
ip_address = "127.0.0.1"
```

**Fields:**

`port`: number

:   Defines the port of the server. In the server configuration files you can use `${SERVER_PORT}` to access this.

    You can also override the value by defining `PORT_name` system environment variable where `name` is name of the server entry.

`ip_address`: string

:   Optionally define the IP address of the server. This is `"127.0.0.1"` by default.

    Similarly to `port`, you can override this using the `IP_name` system environment variable.


