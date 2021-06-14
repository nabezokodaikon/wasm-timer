#!/bin/bash
cargo build --target wasm32-unknown-unknown --release
wasm-bindgen target/wasm32-unknown-unknown/release/wasm_timer.wasm --target web --out-dir ./public/pkg
deno bundle public/ts/timer.ts public/js/timer.js
