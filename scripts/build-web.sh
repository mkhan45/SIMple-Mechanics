#!/bin/sh

./scripts/wasm-bindgen-macroquad.sh simple_mechanics $1

# https://github.com/WebAssembly/wabt
# wasm-strip docs/wbindgen/simple_gravity.wasm
mv docs/wbindgen/simple_mechanics_bg.wasm docs/
mv docs/wbindgen/simple_mechanics.js docs/

if [ "$1" = "serve" ]
then
    # cargo install basic-http-server
    basic-http-server docs
fi
