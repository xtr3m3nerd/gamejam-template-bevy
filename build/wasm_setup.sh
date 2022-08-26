#!/bin/bash
# https://github.com/bevyengine/bevy/tree/latest/examples#platform-specific-examples

rustup target add wasm32-unknown-unknown
cargo install wasm-bindgen-cli
