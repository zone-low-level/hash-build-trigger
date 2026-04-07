# hbd

`hbd` is a build-trigger command used to code overtime as you make changes across your code.
It is basically a watch command that excuses you from repeated recompiling of your zig code.

It internally runs zig build.


## Current status

The build process is yet to be automated. (Just a loop!)
This thing can be differentiated further to even work with config files that automatically detect which files to watch provided you
specify the language you are using.

One can see how ineective it can be in multi language codebases basically.
