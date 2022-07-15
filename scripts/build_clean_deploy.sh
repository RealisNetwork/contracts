#!/usr/bin/env sh

export CONTRACT_NAME="contract.realis.testnet"
export OWNER_ID="realis.testnet"



  cargo build --target wasm32-unknown-unknown --release --features dev
   ./scripts/clean_state.sh
   ./scripts/deploy_contract.sh
