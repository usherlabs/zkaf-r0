#! /bin/bash

cargo build --target wasm32-wasi --release -p verity_zk_verifier --locked
wasm-opt -Os -o ./target/wasm32-wasi/release/verity_zk_verifier.wasm \
        ./target/wasm32-wasi/release/verity_zk_verifier.wasm