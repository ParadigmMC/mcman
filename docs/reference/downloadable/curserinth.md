# CurseRinth

Downloads a mod from [CurseRinth](https://curserinth.kuylar.dev/)'s API, which is basically [curseforge](https://www.curseforge.com/)

!!! example
    ```toml title="Downloads JustEnoughItems from Curseforge"
    type = "curserinth"
    id = "jei"
    version = "4593548"
    ```

**Fields:**

| Name      | Type                  | Description                   |
| --------- | --------------------- | ----------------------------- |
| `type`    | `"curserinth"`/`"cr"` |                               |
| `id`      | string                | The slug or the id of the mod |
| `version` | string/`"latest"`     | The file id of the mod        |
