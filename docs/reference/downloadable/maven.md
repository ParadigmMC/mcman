# Maven

> Added in 0.4.0

Download from a [Maven](https://maven.apache.org/) instance.

**Fields:**

| Name       | Type      | Description                   |
| ---------- | --------- | ----------------------------- |
| `type`     | `"maven"` |                               |
| `url`      | string    | URL to the Maven instance     |
| `group`    | string    | The group, seperated with `.` |
| `artifact` | string    | The name of the artifact      |
| `version`  | string    | The version of the artifact   |
| `filename` | string    | Filename to download          |

!!! note
    The strings can contain variable syntax:

    - `${mcver}` or `${mcversion}` for the `mc_version` in [server.toml](../server.toml.md)
    - `${artifact}` for the resolved artifact (in `version` or `filename`)
    - `${version}` for the resolved version (in `filename`)

!!! note
    For the `version` field, its first checked if the given version exists on the artifact. If it doesn't, it will pick the first version that contains the contents of `version`

    This is also true for the `filename` field

    **For example:**

    ```md
    Lets assume these are the versions and their files:

    - 1.19.4-1.0.0
      - amongus-1.19.4.jar
      - amongus-1.19.4-extra-sus.jar
    - 1.19.4-3.6.7
      - 3.6.7.jar

    And that:
    - artifact: "amongus"
    ```

    | `mc_version` | maven `version`  | resolved version | maven `filename`            | resolved filename              |
    | :----------- | :--------------- | :--------------- | :-------------------------- | :----------------------------- |
    | `1.19.4`     | `${mcver}-1.0.0` | `1.19.4-1.0.0`   | `${artifact}-${mcver}.jar`  | `amongus-1.19.4.jar`           |
    | `1.19.4`     | `${mcver}`       | `1.19.4-1.0.0`   | `${artifact}-${mcver}-extr` | `amongus-1.19.4-extra-sus.jar` |
    | `1.19.4`     | `${mcver}-3`     | `1.19.4-3.6.7`   | `${version}.jar`            | `3.6.7.jar`                    |
    