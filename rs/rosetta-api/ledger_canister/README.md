


```
# For run simple build
cargo build --target wasm32-unknown-unknown --package ledger-canister --release --target-dir build-output-2

# optimize
ic-cdk-optimizer --output ledger-canister-optimized.wasm ledger-canister.wasm
```