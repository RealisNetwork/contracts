#!/usr/bin/env sh

export CONTRACT_NAME="contract.realis.testnet"
export OWNER_ID="realis.testnet"

near --accountId $CONTRACT_NAME \
 call $CONTRACT_NAME clean \
 --base64 "$(node state-cleaner/cleaner.js | base64 -w0)" \
 --gas 300000000000000