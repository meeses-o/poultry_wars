#!/bin/bash
cd gwb/game
cargo build --target wasm32-unknown-unknown --release
cd ../..
rm web/poultry_wars.wasm
rm -r web/pkg
cp gwb/game/target/wasm32-unknown-unknown/release/poultry_wars.wasm web
cd web
wasm-bindgen poultry_wars.wasm --out-dir ./pkg --target web
node helpers/patch_wasm.js pkg/poultry_wars.js
cp helpers/db_bridge.js pkg/db_bridge.js
cd ..
rm -r web_release/pkg
cp -r web/pkg web_release