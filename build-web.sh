#!/bin/bash

cargo build --release --target=wasm32-unknown-emscripten
emcc test.c -o test.html -Os -Wall target/wasm32-unknown-emscripten/release/libpixie.a