#!/bin/bash

set -euo pipefail

# This is a helper script to compile Ledger Archive canister Rust code to Wasm
# and then round trip through wasm2wat and wat2wasm in order to minimize the
# output

echo "Compiling Rust to Wasm"
cargo build --target wasm32-unknown-unknown --release --bin ledger-canister -p ledger-canister

echo "Converting Wasm to Wat"
wasm2wat ../target/wasm32-unknown-unknown/release/ledger-canister.wasm -o ../target/wasm32-unknown-unknown/release/ledger-canister.wat
echo "Converting Wat to Wasm"
wat2wasm ../target/wasm32-unknown-unknown/release/ledger-canister.wat -o ../target/wasm32-unknown-unknown/release/ledger-canister-min.wasm
echo "Removing Wat file"
rm ../target/wasm32-unknown-unknown/release/ledger-canister.wat
echo "Done!"