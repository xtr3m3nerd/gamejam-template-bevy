#!/bin/bash

curdir=$(pwd)

game_name="game"

cd "$( dirname -- "$0"; )/.."

rm -rf target/itch_io
mkdir -p target/itch_io

# Build Wasm
cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_game \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/release/game.wasm

sed -i 's/async function init(input/async function init(input, onload_callback/g' wasm/target/wasm_game.js
sed -i 's/return finalizeInit(instance, module)/if(typeof onload_callback === "function") { onload_callback(); }\nreturn finalizeInit(instance, module)/g' wasm/target/wasm_game.js

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
