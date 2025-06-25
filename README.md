# \[Navtar\] Workspace Directory Manager

A simple CLI to jump between registered directories using short names. 

## Usage

- `wm` -> Will display the available command.
- `wm blog` -> Navigate to the directory registered under the name 'blog'.
- `wm add blog .` -> Registered the current working directory under the name 'blog'.
- `wm list` -> Show all workspaces
- `wm remove blog` -> Delete workspace entry

## Manual Installation

1. Edit `wrapper.ps1` to point toward `navtar.exe`. Using absolute path is recommended. 

    ```
    $wspExe = "path\to\navtar.exe"
    ```

    > [!NOTE]
    > In case you wonder where the heck is the navtar.exe, well... you have to build it first using `cargo build --release`
2. Place `wrapper.ps1` anywhere you like but take not the path. Recommended to put it inside your PowerShell profile directory.
3. Then edit your PowerShell profile by adding the following line:

    ```Microsoft.PowerShell_profile.ps1
    . <Path-To-Where-You-Put-The-Wrapper>\wrapper.ps1
    ```

4. Then restart your terminal/powershell or source the profile by using this command: `. $PROFILE`.

> [!NOTE]
> You can find your PowerShell profile by using command `echo $PROFILE`

## Future Plan

- [ ] Create an installer or helper script to quickly setup this thing without the need of manual installation.

Well, this is my attempt to build something using Rust. There might be things that I will add later on. I do have similar tools
but written in Python. However, I find it a bit slow especially to list the registered workspace. I call it workspace instead of directory
because that what I used this thing for. To quickly `cd` into my playground that scattered around.
