#!/bin/bash
cd ..
wasm-bindgen poultry_wars.wasm --out-dir ./pkg --target web
node helpers/patch_wasm.js pkg/poultry_wars.js
cp helpers/db_bridge.js pkg/db_bridge.js

#To gen the wasm
#cargo build --target wasm32-unknown-unknown --release