# Networks

If you want to manage multiple servers at once like a network, you can use `network.toml`:

```toml
name = "CoolNetwork"
port = 25565
```

## Servers

In `network.toml`, define servers in the `servers` table:

```toml
name = "CoolNetwork"
port = 25565

[servers.lobby]
port = 25566

[servers.game1]
port = 25567
```

In the folder structure, keep the servers under the `servers/` folder:

```yaml
cool_network
├─ network.toml
└─ servers/
   ├─ lobby
   │  └─ server.toml
   └─ game1
      └─ server.toml
```

If needed, you can optionally define `ip_address` in servers. This is `"127.0.0.1"` by default.

At the moment, most of these rules aren't used or enforced, but kept in here so other tools could be created around this.

## Variables

Just like the normal `server.toml` variables, you can define custom variables in `network.toml`:

```toml
[variables]
SOME = "thing"
```

Network variables need to be prefixed with `NW_` while accessing them.

So instead of using `${SOME}` to access it, `${NW_SOME}` can be used.

## Special Variables

Here are some more special variables.

!!! note
    You can use the `PORT_name` and `IP_name` environment variables to override server ip addresses and ports. The `name` must be the name of the server as defined in `server.toml`.

- `SERVER_IP`: the IP address of the server
- `SERVER_PORT`: the port of the server
- `NETWORK_NAME`: name of the network
- `NETWORK_PORT`: the defined port
- `NETWORK_SERVERS_COUNT`: amount of servers defined

You can also get the port or IP of another server via

- `NW_SERVER_name_IP`
- `NW_SERVER_name_PORT`
- `NW_SERVER_name_ADDRESS`: Basically "ip:port"

These generate a table of servers that can be used in proxy server configurations:

- `NETWORK_VELOCITY_SERVERS` for Velocity
- `NETWORK_BUNGEECORD_SERVERS` for BungeeCord/Waterfall

=== "Usage in Velocity"
    ```toml
    #${NETWORK_VELOCITY_SERVERS}
    ```

=== "Usage in BungeeCord"
    ```toml
    #${NETWORK_BUNGEECORD_SERVERS}
    ```

You can comment the line because it will start with a comment/disclaimer.
