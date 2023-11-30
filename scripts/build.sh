#!/bin/bash
set -eox pipefail

source .env

echo ">> Building contract: $CONTRACT_NAME"

rustup target add wasm32-unknown-unknown
cargo build -p $CONTRACT_NAME --target wasm32-unknown-unknown --profile=contract

cp ./target/wasm32-unknown-unknown/contract/$CONTRACT_NAME.wasm res/$CONTRACT_NAME.wasm
