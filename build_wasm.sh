#!/bin/bash
set -euxo pipefail

WASM_FILE=hello_wasm.wasm
WASM_DIR=wasm_out

if [ "$#" -ne 1 ]; then
    echo "Usage $0 (release|debug)"
    exit 1
fi
MODE=$1
WASM_DIR_RAW=target/wasm32-unknown-unknown/$MODE
case $MODE in
    release)
        CARGO_ARGS="--release"
        ;;
    debug)
        CARGO_ARGS=""
        ;;
    *)
esac
mkdir -p $WASM_DIR
cargo build --target=wasm32-unknown-unknown $CARGO_ARGS
wasm-bindgen $WASM_DIR_RAW/$WASM_FILE --out-dir $WASM_DIR
