#!/bin/bash
set -e
cd "`dirname $0`"

cargo build --all --target wasm32-unknown-unknown --release
# cp $TARGET/wasm32-unknown-unknown/release/defi.wasm ./res/
cp target/wasm32-unknown-unknown/release/nftpawn_token.wasm ./res/
