# MIT License
# Copyright (c) 2024 largenumbergoeshere
cargo +nightly install wasm-bindgen-cli 
cargo build --release --target=wasm32-unknown-unknown
rm -d ./target/test_demployment -ErrorAction SilentlyContinue
wasm-bindgen target/wasm32-unknown-unknown/release/no_modules.wasm --out-dir ./target/test_deployment  --target web    --no-typescript
cp -Force .\templates\index.html .\target\test_deployment

python -m http.server -d .\target\test_deployment