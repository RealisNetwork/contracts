#!/usr/bin/env sh

export CONTRACT_NAME="contract.realis.testnet"
export OWNER_ID="realis.testnet"

Result=$(near view $CONTRACT_NAME get_contract_settings '{}' --account-id $OWNER_ID)
if [[ $Result != *"contract.realis.testnet.get_contract_settings({})"* ]];
then
  echo "Deploy with initialization"
  near deploy --accountId $OWNER_ID \
           --wasmFile  ./target/wasm32-unknown-unknown/release/realis_near.wasm \
           --initFunction "new" \
           --initArgs '{"total_supply": "3000000000","constant_fee":"12", "percent_fee": 10, "beneficiary_id": "'$OWNER_ID'","backend_id": "'$OWNER_ID'"}' \
           --initGas 300000000000000
  else echo "Redeploy contract."
  echo y | near deploy --wasmFile ./target/wasm32-unknown-unknown/release/realis_near.wasm  --accountId $OWNER_ID
fi





