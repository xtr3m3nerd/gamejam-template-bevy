#!/bin/bash

cd "$( dirname -- "$0"; )/.."

cargo build --target=x86_64-pc-windows-gnu --release

cd -
