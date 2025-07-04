# Building
Building Oreneta is fairly simple. First, you need to install a couple of dependencies. On Debian or Debian based distros, you can run `apt install build-essential nasm xorriso llvm clang` as root to install. 

On Arch Linux, you can run `sudo pacman -S base-devel xorriso mtools nasm qemu-full`. 

On Mac you can install the dependencies using [Homebrew](https://brew.sh/) and Xcode tools. You can install Xcode tools with: `xcode-select --install` then install the rest of the dependencies with Brew (Commands coming soon, feel free to make a PR). Verify the installation with:

```
nasm --version
xorriso --version
clang --version
llvm-config --version
```

You will also need the Rust language and utilities, of course, which you can get by following [these](https://www.rust-lang.org/tools/install) instructions.

While you can also use the `setupenv.sh` to automatically install dependencies for your operating system, it is not recommended (currently not working on MacOS)

After installing Rust, do:
 ```bash
 rustup toolchain install nightly
 rustup override set nightly
 ```

## Building

To build, simply run `make`. This will compile the Rust into a static library, compile the assembly, and then link with `clang`. It then uses `grub-mkrescue` to generate an ISO file. You can also run `make run` to run the code in QEMU.


This work is licensed under the Open Source Creator's Share License for Teams (OSCSL-T-1.0).
For details, see https://protectopensource.org or contact jake@protectopensource.org.
For commercial use, contact admin@poyoweb.org.  
