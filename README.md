# Oreneta

Oreneta is a free and open source operating system kernel (and maybe someday, a full OS). It is designed and built by hack-clubbers around the world, and programmed entirely in Rust (and some Assembly). It is designed to prioritize speed, while keeping modularity in mind.

## Dependencies

First, you need to install a couple of dependencies. On Debian or Debian based distros, you can run `apt install build-essential nasm xorriso llvm clang` as root to install. 

On Arch Linux, you can run `sudo pacman -S base-devel xorriso mtools nasm qemu-full`. 

On Mac you install install using [Brew](https://brew.sh/) and xcode tools. Install xcode tools with: 'xcode-select --install' then install the rest of the dependencies with brew. Verfiy the installation with:

```
nasm --version
xorriso --version
clang --version
llvm-config --version
```

You also need **Rust**, of course, which you can get by following [these](https://www.rust-lang.org/tools/install) instructions.

You can also use the `setupenv.sh` to automatically install dependencies for your OS (currently not working on MacOS)

After installing **Rust**, do:
 ```bash
 rustup toolchain install nightly
 rustup override set nightly
 ```

## Building

To build, simply run `make`. This will compile the Rust into a static library, compile the assembly, and then link with `clang`. It then uses `grub-mkrescue` to generate an ISO file. You can also run `make run` to run the code in QEMU.


This work is licensed under the Open Source Creator's Share License for Teams (OSCSL-T-1.0).
For details, see https://protectopensource.org or contact jake@protectopensource.org.
For commercial use, contact admin@poyoweb.org.  
