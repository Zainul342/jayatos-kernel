#!/bin/bash
set -e

# Pastikan rustup default ke nightly (fitur eksperimental diperlukan)
rustup override set nightly
rustup component add rust-src llvm-tools-preview

# Install bootimage tool (hanya sekali)
if ! command -v bootimage &> /dev/null
then
    echo "Installing bootimage..."
    cargo install bootimage
fi

# Build Kernel
echo "Building ProgrammerOS Jayatos..."
cargo build --target x86_64-programmeros_jayatos.json

# Create Bootable Image
echo "Creating Bootable Image..."
bootimage build --target x86_64-programmeros_jayatos.json

# Run in QEMU
echo "Launching QEMU..."
qemu-system-x86_64 -drive format=raw,file=target/x86_64-programmeros_jayatos/debug/bootimage-programmeros_jayatos.bin
