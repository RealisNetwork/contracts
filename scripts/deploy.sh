#!/usr/bin/env sh

export NEAR_ENV=testnet #mainnet
export OWNER_ID="example.testnet"
export CONTRACT_ID="contract.$OWNER_ID"

if ! [ -x "$(command -v cargo)" ];
then
    echo "Installing cargo"
    curl https://sh.rustup.rs -sSf | sh
    echo "Installing target"
    rustup target add wasm32-unknown-unknown
fi

echo "Building contract"
cargo build --target wasm32-unknown-unknown --release

if ! [ -x "$(command -v near)" ];
then
    echo "Installing near cli"
    npm install -g near-cli
fi

near login

if [[ $(near state $CONTRACT_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $CONTRACT_ID \
        --masterAccount $OWNER_ID \
        --initialBalance 50
fi

if [[ $(near view-state $CONTRACT_ID --finality final) == *"[]"* ]];
then
    echo "Deploying contract"
    near deploy --accountId $CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/realis_near.wasm \
        --initFunction "new" \
        --initArgs '{"constant_fee": "0", "percent_fee": 0}' \
        --initGas 300000000000000
else
    echo "Updating contract"
    echo y | near deploy --accountId $CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/realis_near.wasm \
        --initFunction "migrate" \
        --initArgs '{}' \
        --initGas 300000000000000
fi
