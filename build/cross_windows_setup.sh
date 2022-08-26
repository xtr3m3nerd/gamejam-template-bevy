#!/bin/bash
# https://bevy-cheatbook.github.io/setup/cross/linux-windows.html

# MSVC
# rustup target add x86_64-pc-windows-msvc
# cargo install xwin
# xwin --accept-license 1 splat --output /opt/xwin

# cargo build --target=x86_64-pc-windows-msvc --release

# [target.x86_64-pc-windows-msvc]
# linker = "lld"
# rustflags = [
#   "-Lnative=/opt/xwin/crt/lib/x86_64",
#   "-Lnative=/opt/xwin/sdk/lib/um/x86_64",
#   "-Lnative=/opt/xwin/sdk/lib/ucrt/x86_64"
# ]

# GNU
rustup target add x86_64-pc-windows-gnu
rustup toolchain install stable-x86_64-pc-windows-gnu
# install MINGW environment
# pacman -S cross-x86_64-w64-mingw32

# cargo build --target=x86_64-pc-windows-gnu --release
