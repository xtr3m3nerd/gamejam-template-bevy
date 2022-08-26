#!/bin/bash
# https://github.com/bevyengine/bevy/tree/latest/examples#platform-specific-examples

rustup target add aarch64-linux-android armv7-linux-androideabi
cargo install cargo-apk

# cargo apk run --example android_example

# [package.metadata.android]
# build_targets = ["aarch64-linux-android", "armv7-linux-androideabi"]
#
# [package.metadata.android.sdk]
# target_sdk_version = 31

