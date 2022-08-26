#!/bin/bash

cd "$( dirname -- "$0"; )/.."

cargo build --release --target wasm32-unknown-unknown
wasm-bindgen --out-name wasm_game \
  --out-dir wasm/target \
  --target web target/wasm32-unknown-unknown/release/game.wasm

sed -i 's/async function init(input/async function init(input, onload_callback/g' wasm/target/wasm_game.js
sed -i 's/return finalizeInit(instance, module)/if(typeof onload_callback === "function") { onload_callback(); }\nreturn finalizeInit(instance, module)/g' wasm/target/wasm_game.js

cd -

# async function init(input, onload_callback) {
#     if (typeof input === 'undefined') {
#         input = new URL('wasm_example_bg.wasm', import.meta.url);
#     }
#     const imports = getImports();
#
#     if (typeof input === 'string' || (typeof Request === 'function' && input instanceof Request) || (typeof URL === 'function' && input instanceof URL)) {
#         input = fetch(input);
#     }
#
#     initMemory(imports);
#
#     const { instance, module } = await load(await input, imports);
#
# 		if(typeof onload_callback === "function") {
# 			onload_callback();
# 		}
#
#     return finalizeInit(instance, module);
# }
