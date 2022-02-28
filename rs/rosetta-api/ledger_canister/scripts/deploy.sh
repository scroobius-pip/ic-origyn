#!/usr/bin/env bash

set -o xtrace

dfx identity new admin

echo ${DFX_IDENTITY} > ~/.config/dfx/identity/admin/identity.pem

dfx identity use admin

dfx identity whoami

dfx identity set-wallet rwlgt-iiaaa-aaaaa-aaaaa-cai

dfx deploy --network local