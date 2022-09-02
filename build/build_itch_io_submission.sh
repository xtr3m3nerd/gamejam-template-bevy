#!/bin/bash

curdir=$(pwd)

game_name="game"

cd "$( dirname -- "$0"; )/.."

rm -rf target/itch_io
mkdir -p target/itch_io

# Build Wasm
./build/build_wasm.sh

cp -r wasm target/itch_io/wasm
rm target/itch_io/wasm/assets
cp -r $game_name/assets target/itch_io/wasm/assets

cd target/itch_io/wasm
zip -r ../wasm.zip ./*
cd -

# Build Windows
cargo build --target=x86_64-pc-windows-gnu --release

mkdir -p target/itch_io/windows
cp -r $game_name/assets target/itch_io/windows/assets
cp target/x86_64-pc-windows-gnu/release/$game_name.exe target/itch_io/windows/

cd target/itch_io/windows
zip -r ../windows.zip ./*
cd -


# Build Linux
cargo build --release

mkdir -p target/itch_io/linux
cp -r $game_name/assets target/itch_io/linux/assets
cp target/release/$game_name target/itch_io/linux/

cd target/itch_io/linux
zip -r ../linux.zip ./*
cd -

cd $curdir
