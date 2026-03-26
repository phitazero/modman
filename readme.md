modman - a CLI mod manager for Minecraft used with Prism Launcher[^1] with pacman-styled commands. Linux only.

[^1]: Or any other launcher supporting multiple isolated instances, each having the contents of the default .minecraft/ (resourcepacks, mods, saves) in /path-to-instance/minecraft/. Not sure if such launchers exist, just in case. Tested only with Prism Launcher.

Made this thing because:  
1. Tired of manually downloading, dragging around and updating the .jar's
1. I like CLI and specifically the way pacman commands look
1. Sunk cost fallacy (in case questions like "why don't just use X" appear)

# Installation
Git clone, cd, `cargo build --release`, move `target/release/modman` somewhere, where it's in PATH.

# Usage
Used with Prism Launcher[^1].

`cd` into the instance's directory.  
`modman -M -l <loader> -v <version>`  
will create a modpack and symlink few things I want to have shared (worlds, resourcepacks, shaders, etc.) into a shared directory.

After that you'll be able to search for mods with  
`modman -Ss <query>`,  
install mods by their slugs with  
`modman -S <slug>`,  
remove with  
`modman -Rs <slug>`  
and many other things described in the next section.

# Commands
## -h
Show help. Can be used as a flag with any of the operations listed below, to get help for that exact operation.

## -S
### `-S <slugs>`  
Install mods by slugs.  

### `-Ss <query>`  
Search for mods by a query. 
The slug for each mod is specified in parentheses.   
By default filters the mods to be compatible with the current modpack (if present). This can be disabled with the `-d` flag.  
Shows a limited amount of search results, specified in the config. This can be disabled with the `-a` flag.

### `-Si <slugs>`
Fetch detailed info about mods by their slugs.  
Slugs of the dependencies can be additinally fetched with the `-f` flag.

### `-Su [slugs]`
Upgrade mods.  
If slugs not specified - performs the operation on all mods.   
It it not recommended to Ctrl-C during this operation, since it works stupidly - removes all mods, then reinstalls them. During reinstallation the slugs of all of the mods live solely in RAM, the manifest file is cleaned, so Ctrl-C'ing will destroy some of the info of mods to be installed.

### Also:
In case some dependencies can't be installed (there once was a Forge-only dependency for a Fabric mod, stupid Modrinth) use `-j` to omit installing them. The mod will be marked correspondingly, and dependencies will not be auto-installed when upgrading it, even without `-j`.

## `-R`
### `-R <slugs>`
Removes mods.  
With `-s` also removes the dependencies unused by any other mods.

## `-Q`
### `-Q [slugs]`
Prints the slugs and version numbers of the corresponding mods (if present and not filtered out).  
If slugs not specified - performs the operation on all mods.    
To print without version numbers use `-q`.

### `-Qi [slugs]`
Prints detailed info about the mods.  
If slugs not specified - performs the operation on all mods.   

### Filters:
`-d` - mods installed as dependencies  
`-e` - explicitly installed mods  
`-t` - mods unrequired by other mods

## `-M`
### `-M -l <loader> -v <version>`
Create a modpack and symlink `resourcepacks/`, `shaderpacks/`, `saves/`, `screenshots/`, `hotbar.nbt`, `options.txt` and `servers.dat` of the instance into those of a shared `.minecraft/` directory you'll have to specify in config.

### `-Mi`
Print info about the current modpack.
