# `mcman cache`

Provides some utilities for managing your local cache directory.

## `mcman cache path`

Prints the cache path. This is generally:

- Windows: `%LocalAppData%/mcman`
- Linux: `~/.cache/mcman`

## `mcman cache open`

Opens the cache folder using a file explorer

## `mcman cache list`

Lists the cache entries.

```
$ mcman cache list
Listing cache...
Folder: C:\Users\dennis\AppData\Local\mcman
=> github - 24 entries
=> modrinth - 278 entries
=> papermc - 1 entries
303 entries in 3 namespaces in total
```

??? note "Detailed listing"
    You can use the `-d` flag to get a more detailed list

    ```
    $ mcman cache list -d
    Listing cache...
    Folder: C:\Users\dennis\AppData\Local\mcman
    => github - 8 entries
        └ emilyploszaj
        └ EngineHub
        └ LemmaEOF
        └ MerchantPug
        └ ModFest
        └ NucleoidMC
        └ Patbox
        └ TheEpicBlock
    => modrinth - 10 entries
        └ 10DZYVis
        └ 13RpG7dA
        └ 14Z3YVAP
        └ 1cjUVbYD
        └ 1eAoo2KR
        └ 1IjD5062
        └ 1itdse3V
        └ 1LrBk5C6
        └ 1qeWG44Y
        └ 2qcCxsBR
    => papermc - 1 entries
        └ paper
    19 entries in 3 namespaces in total
    ```

## `mcman cache clear`


