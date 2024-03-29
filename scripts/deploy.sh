#!/usr/bin/env sh

export NEAR_ENV=mainnet
export OWNER_ID="realisnetwork.near"
export BACKEND_ID="backend.$OWNER_ID"
export ROOT_CONTRACT_ID="v1.$OWNER_ID"
export TOKEN_CONTRACT_ID="token.$ROOT_CONTRACT_ID"
export STAKING_CONTRACT_ID="staking.$ROOT_CONTRACT_ID"
export LOCKUP_CONTRACT_ID="lockup.$ROOT_CONTRACT_ID"
export NFT_CONTRACT_ID="nft.$ROOT_CONTRACT_ID"

if ! [ -x "$(command -v cargo)" ];
then
    echo "Installing cargo"
    curl https://sh.rustup.rs -sSf | sh
    echo "Installing target"
    rustup target add wasm32-unknown-unknown
fi

echo "Building contracts"
cargo wasm

if ! [ -x "$(command -v near)" ];
then
    echo "Installing near cli"
    npm install -g near-cli
fi

# Creating backend account for contracts if not exists
if [[ $(near state $BACKEND_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $BACKEND_ID \
        --masterAccount $OWNER_ID \
        --initialBalance 20
fi

# TODO: Check backend for enough balance and transfer near

# Creating root account for contracts if not exists
if [[ $(near state $ROOT_CONTRACT_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $ROOT_CONTRACT_ID \
        --masterAccount $OWNER_ID \
        --initialBalance 210
fi

# TODO: Check root for enough balance for deploy and transfer near

# Creating account for token contracts if not exists
if [[ $(near state $TOKEN_CONTRACT_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $TOKEN_CONTRACT_ID \
        --masterAccount $ROOT_CONTRACT_ID \
        --initialBalance 50
fi

# Creating account for lockup contracts if not exists
if [[ $(near state $LOCKUP_CONTRACT_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $LOCKUP_CONTRACT_ID \
        --masterAccount $ROOT_CONTRACT_ID \
        --initialBalance 50
fi

# Creating account for staking contracts if not exists
if [[ $(near state $STAKING_CONTRACT_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $STAKING_CONTRACT_ID \
        --masterAccount $ROOT_CONTRACT_ID \
        --initialBalance 50
fi

# Creating account for nft contracts if not exists
if [[ $(near state $NFT_CONTRACT_ID) == *"not found"* ]];
then
    echo "Creating account for contract"
    near create-account $NFT_CONTRACT_ID \
        --masterAccount $ROOT_CONTRACT_ID \
        --initialBalance 50
fi

# Deploying token contracts if not exists, otherwise update
if [[ $(near view-state $TOKEN_CONTRACT_ID --finality final) == *"[]"* ]];
then
    echo "Deploying contract"
    near deploy --accountId $TOKEN_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/ft_token_contract.wasm \
        --initFunction "new" \
        --initArgs '{"owner_id": "'$OWNER_ID'", "staking_id": "'$STAKING_CONTRACT_ID'"}' \
        --initGas 300000000000000
else
    echo "Updating contract"
    echo y | near deploy --accountId $TOKEN_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/ft_token_contract.wasm \
        --initFunction "update" \
        --initArgs '{}' \
        --initGas 300000000000000
fi

# Deploying lockup contracts if not exists, otherwise update
if [[ $(near view-state $LOCKUP_CONTRACT_ID --finality final) == *"[]"* ]];
then
    echo "Deploying contract"
    near deploy --accountId $LOCKUP_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/ft_lockup_contract.wasm \
        --initFunction "new" \
        --initArgs '{"token_account_id": "'$TOKEN_CONTRACT_ID'", "deposit_whitelist": ["'$OWNER_ID'", "'$STAKING_CONTRACT_ID'"]}' \
        --initGas 300000000000000
    echo "Register in token contract"
    near call $TOKEN_CONTRACT_ID storage_deposit '{"account_id": "'$LOCKUP_CONTRACT_ID'"}' --accountId $OWNER_ID --deposit 1
else
    echo "Updating contract"
    echo y | near deploy --accountId $LOCKUP_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/ft_lockup_contract.wasm \
        --initGas 300000000000000
fi

# Deploying staking contracts if not exists, otherwise update
if [[ $(near view-state $STAKING_CONTRACT_ID --finality final) == *"[]"* ]];
then
    echo "Deploying contract"
    near deploy --accountId $STAKING_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/ft_staking_contract.wasm \
        --initFunction "new" \
        --initArgs '{"owner_id": "'$OWNER_ID'", "token_account_id": "'$TOKEN_CONTRACT_ID'", "lockup_account_id": "'$LOCKUP_CONTRACT_ID'"}' \
        --initGas 300000000000000
else
    echo "Updating contract"
    echo y | near deploy --accountId $STAKING_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/ft_staking_contract.wasm \
        --initFunction "update" \
        --initArgs '{}' \
        --initGas 300000000000000
fi

# Deploying nft contracts if not exists, otherwise update
if [[ $(near view-state $NFT_CONTRACT_ID --finality final) == *"[]"* ]];
then
    echo "Deploying contract"
    near deploy --accountId $NFT_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/nft_token_contract.wasm \
        --initFunction "new" \
        --initArgs '{"owner_id": "'$OWNER_ID'", "backend_id": "'$BACKEND_ID'"}' \
        --initGas 300000000000000
else
    echo "Updating contract"
    echo y | near deploy --accountId $NFT_CONTRACT_ID \
        --wasmFile ./target/wasm32-unknown-unknown/release/nft_token_contract.wasm \
        --initFunction "update" \
        --initArgs '{}' \
        --initGas 300000000000000
fi
