# Oreneta

Oreneta is a free and open source operating system kernel (and maybe someday, a full OS). It is designed and built by hack-clubbers around the world, and programmed entirely in Rust (and some Assembly). It is designed to prioritize speed, while keeping modularity in mind.

## Building

First, you need to install a couple of dependencies. On Debian or Debian based distros, you can run `apt install base-devel nasm xorriso llvm-tools clang` as root to install. On Arch Linux, you can run `sudo pacman -S base-devel xorriso mtools nasm qemu`. You also need Rust, of course, which you can get by following [these](https://www.rust-lang.org/tools/install) instructions.

To build, simply run `make`. This will compile the Rust into a static library, compile the assembly, and then link with `clang`. It then uses `grub-mkrescue` to generate an ISO file. You can also run `make run` to run the code in QEMU.
