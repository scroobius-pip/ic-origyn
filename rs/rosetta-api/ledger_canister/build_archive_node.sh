#!/bin/bash

set -euo pipefail

# This is a helper script to compile Ledger Archive canister Rust code to Wasm
# and then round trip through wasm2wat and wat2wasm in order to minimize the
# output

echo "Compiling Rust to Wasm"
cargo build --target wasm32-unknown-unknown --release --bin ledger-archive-node-canister

echo "Converting Wasm to Wat"
wasm2wat ../target/wasm32-unknown-unknown/release/ledger-archive-node-canister.wasm -o ../target/wasm32-unknown-unknown/release/ledger-archive-node-canister.wat
echo "Converting Wat to Wasm"
wat2wasm ../target/wasm32-unknown-unknown/release/ledger-archive-node-canister.wat -o ../target/wasm32-unknown-unknown/release/ledger-archive-node-canister.wasm
echo "Removing Wat file"
rm ../target/wasm32-unknown-unknown/release/ledger-archive-node-canister.wat
echo "Done!"