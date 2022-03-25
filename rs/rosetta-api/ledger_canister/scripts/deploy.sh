#!/usr/bin/env bash

set -o xtrace

dfx identity new ic_admin

MINT_ACC=$(dfx ledger account-id)

mkdir ~/.config/dfx/identity/admin/

echo ${DFX_IDENTITY} > ~/.config/dfx/identity/admin/identity.pem

sed -i 's/\\r\\n/\r\n/g' ~/.config/dfx/identity/admin/identity.pem

dfx identity use admin

LEDGER_ACC=$(dfx ledger account-id)

dfx identity whoami

dfx identity set-wallet rwlgt-iiaaa-aaaaa-aaaaa-cai

dfx deploy ledger --argument "record {minting_account = \"${MINT_ACC}\"; initial_values = vec { record { \"${LEDGER_ACC}\"; record { e8s = 18446744073709551615 } } }; max_message_size_bytes = null; transaction_window = null; archive_options = null; send_whitelist = vec {}}" --network=local