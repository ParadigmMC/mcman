[latest-win]: https://github.com/ParadigmMC/mcman/releases/latest/download/mcman.exe
[latest-linux]: https://github.com/ParadigmMC/mcman/releases/latest/download/mcman

# Installation

=== "Github Releases"
    You can use the links below to get the mcman executable.

    [:fontawesome-brands-windows: Windows][latest-win]{ .md-button } [:fontawesome-brands-linux: OSX/Linux][latest-linux]{ .md-button }

    [:simple-github: Github Releases Page](https://github.com/ParadigmMC/mcman/releases){ .md-button } [:simple-github: Build Action (nightly)](https://github.com/ParadigmMC/mcman/actions/workflows/build.yml){ .md-button }
    
    We suggest you to add it to your `PATH` so you can run it from anywhere! (windows)

=== "Windows: scoop"
    Add the [minecraft](https://github.com/The-Simples/scoop-minecraft) bucket and install mcman:

    ```powershell
    scoop bucket add minecraft https://github.com/The-Simples/scoop-minecraft
    scoop install mcman
    ```

    [Scoop](https://scoop.sh/) is a command-line installer for Windows. You can use 2 commands in powershell to install it. (4 commands in total to install mcman!)

=== "Linux: wget"
    wget

    ```sh title="Install mcman"
    wget https://github.com/ParadigmMC/mcman/releases/latest/download/mcman
    sudo mv ./mcman /usr/bin/
    sudo chmod +x /usr/bin/mcman
    ```

    bottom text

