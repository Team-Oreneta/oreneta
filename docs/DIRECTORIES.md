# Directory tree
This is the directory tree of Oreneta's source code. Note that for conciseness's sake, only general directories are included; not every Rust module.
+ `isoroot/`: All files and subdirectories in this folder will be added to the initial ramdisk. Currently, Makefile targets are a little annoying, and you might have to `make clean` or delete `build/` in order for changes to `isoroot/` to show up. This directory is `/` to Oreneta.
+ `config/` contains some configuration files necessary for building or file generation.
+ `src/`: `src/` contains the primary source code for Oreneta. All Rust files in `src/` will be compiled and linked.
+ `src/arch/`: This is where all of the architecture dependent code _should_ be, though it is not currently due to Oreneta currently only targeting i686 (32 bit Intel and AMD processors).
+ `src/arch/[arch]/asm/` contains NASM Assembly files specific for the architecture `[arch]`. All `.asm` files in this directory are compiled and linked with the Rust when targeting `[arch]`.