#!/bin/bash

os=""
if [[ "$OSTYPE" == "darwin"* ]]; then
    os=$"macOS"
elif [ -f "/etc/os-release" ]; then
    if grep -q "Ubuntu\|Debian" /etc/os-release; then
        os=$"Debian/Ubuntu"
	apt install base-devel nasm xorriso llvm-tools clang
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
	rustup toolchain install nightlyB
	rustup override set nightly
    elif grep -q "Arch" /etc/os-release; then
        os=$"Arch"
	sudo pacman -S base-devel xorriso mtools nasm qemu
	curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
        rustup toolchain install nightly
        rustup override set nightly
    fi
else
    echo "Unknown"
    exit 1
fi


echo $os