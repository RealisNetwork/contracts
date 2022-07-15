#!/usr/bin/env sh

export CONTRACT_NAME="contract.realis.testnet"
export OWNER_ID="realis.testnet"

  cargo build --target wasm32-unknown-unknown --release --features dev
#todo:fix problem Failed to deserialize input from JSON EOF while parsing a value line: 1, column: 0)', src/update.rs:13:1
   ./scripts/clean_state.sh
   ./scripts/deploy_contract.sh
