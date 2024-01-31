# Caching

mcman will try to cache downloaded files and other metadata such as Github API requests.

The cache folder will generally be:

- Windows: `%LocalAppData%/mcman`
- Linux: `~/.cache/mcman`

You can also manage the cache using the `mcman cache` command.

- `mcman cache path`: Prints the cache path
- `mcman cache list`: Lists caches
    - `-d` for detailed
- `mcman cache open`: Opens the cache folder using a file explorer
- `mcman cache clear`: Clears the cache without confirmation

## Folders

Most sources have their own folders:

- Modrinth: `modrinth/{project}/{version}/{file}`
- Curserinth: `curserinth/{project}/{version}/{file}`
- Github:
    - Metadata: `github/{owner}/{repo}/releases.json`
    - Releases: `github/{owner}/{repo}/releases/{tag}/{file}`
- Hangar: `hangar/{owner}/{proj}/{version}/{file}`
- Jenkins: `jenkins/{url}/{...job}/{build}/{file}`
- Maven: `maven/{url}/{...group}/{artifact}/{version}/{file}`
- PaperMC: `papermc/{proj}/{proj}-{mcver}-{build}.jar`
