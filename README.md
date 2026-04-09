# hbd

`hbd` is a build-trigger command used to code overtime as you make changes across your code.
It is basically a watch command that excuses you from repeated recompiling your code.

By default it is built to support zig codebases.


## Usage

```
$ hbd --help
Watch for changes in your source files and recompile automatically.

Usage: hbd [OPTIONS] --dirs <DIRS>...

Options:
  -d, --dirs <DIRS>...           Directories to scan (can be repeated)
  -e, --extensions <EXTENSIONS>  File extensions to include (e.g. rs toml json). If omitted, all files are included
  -b, --build-cmd <BUILD_CMD>    Build command to run if hash changed (default: "zig build") [default: "zig build"]
      --cache-file <CACHE_FILE>  Cache file that stores the last hash [default: zig-out/.last-source-hash]
      --disable <DISABLE>        disable the clearing of stdout during builds [possible values: true, false]
  -h, --help                     Print help
  -V, --version                  Print version
  ```
  
  Simply specify directory to watch.
  
 ```sh 
    hbd -d src/
 ```
