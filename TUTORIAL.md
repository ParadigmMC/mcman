# Getting Started

## Installation

todo

## Create your first server

For the purposes of this tutorial, we will create a simple survival server.

1. Create a folder and name it as your server's name

2. **Recommended:** initialize a git repository
   - this allows you to share and manage your configs between hosts more efficiently 

3. Run **`mcman init`** in the new folder. This will create your `server.toml` config file and some other files as a starting point.

4. Let's use Purpur for our server. Open `server.toml` and edit it like so:

    ```toml
    [jar]
    type = "purpur"
    version = "1.19.4"
    ```

5. Now let's test if it works. Run **`mcman build`** - this will build your server in the *server/* folder. If you see `purpur-blablabla.jar` then it means it works properly.

6. Let's add some plugins. Add the following to your config file:

    ```toml
    [[plugins]]
    type = "modrinth"
    id = "authmevelocity"
    version = "mhfhAMOb"
    ```

7. When you run **`mcman build`** again, the plugin will automatically be installed.

8. Let's now add some configuration for the plugin: Create a new file in `./config/plugins/authmevelocity/config.conf` and configure the plugin in the file.

9. When you run **`mcman build`** again, the plugin configuration will automatically be copied to the output folder.

10. Use the generated launcher scripts to launch your server (`start.bat` or `start.sh`).
