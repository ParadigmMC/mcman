# Jenkins

Download an artifact from a [Jenkins](https://www.jenkins.io/) server.

!!! example
    Example using [Scissors](https://github.com/AtlasMediaGroup/Scissors) 1.20.1:

    ```toml
    type = "jenkins"
    url = "https://ci.plex.us.org"
    job = "Scissors/1.20.1"

    # (1)
    build = "latest"
    artifact = "first"
    ```

    1. These are the default values and since they are optional, they can be removed.

!!! info
    Nested jobs can be written using slashes. For example, if the URL was something like `/job/A/job/B`, the job field would be `A/B`.

**Fields:**

| Name | Type | Description |
| --- | --- | --- |
| `type` | `"jenkins"`||
| `url` | string | URL to the Jenkins instance |
| `job` | string | The job name |
| `build` | string/`"latest"` | The build number to use |
| `artifact` | string/`"first"` | The name of the artifact (checks for inclusion, like [github releases](./github-releases.md)) |
