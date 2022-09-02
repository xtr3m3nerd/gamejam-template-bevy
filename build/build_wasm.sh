#!/bin/bash

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_game \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/release/game.wasm

sed -i 's/async function init(input/async function init(input, onload_callback/g' wasm/target/wasm_game.js
sed -i 's/return finalizeInit(instance, module)/if(typeof onload_callback === "function") { onload_callback(); }\nreturn finalizeInit(instance, module)/g' wasm/target/wasm_game.js
