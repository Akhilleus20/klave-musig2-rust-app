echo building multiple apps.wasm from Rust...
rm Cargo.lock
rm -rf target
cargo clean

rm -f apps/musig-compute/src/bindings.rs
rm -f apps/musig-wallet/src/bindings.rs

cargo component build --target wasm32-unknown-unknown --release
base64 -w 0 target/wasm32-unknown-unknown/release/musig_wallet.wasm > ./musig_wallet.b64
base64 -w 0 target/wasm32-unknown-unknown/release/musig_compute.wasm > ./musig_compute.b64

echo done