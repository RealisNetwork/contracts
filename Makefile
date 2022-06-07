CONTRACT_NAME = token_contract_v2
ROOT_ACCOUNT = testnetacc.testnet

.PHONY: deploy
deploy:
	cargo build --target wasm32-unknown-unknown --release
	near delete $(CONTRACT_NAME).$(ROOT_ACCOUNT) $(ROOT_ACCOUNT)
	near create-account $(CONTRACT_NAME).$(ROOT_ACCOUNT) --masterAccount $(ROOT_ACCOUNT)
	near deploy --accountId $(CONTRACT_NAME).$(ROOT_ACCOUNT) \
				--wasmFile ./target/wasm32-unknown-unknown/release/lis_token.wasm \
				--initFunction new \
				--initArgs '{"total_supply": "3000000", "fee": '5', "beneficiary_pk": "FG6aRApk5Ym9nDwzdWFg22ti5GWeW8mBqCKL7M3LZH62"}'

.PHONY: create
create:
	near call $(CONTRACT_NAME).$(ROOT_ACCOUNT) \
		create_account '{"account_id": "test_create_account5.token_contract_v2.testnetacc.testnet", "pk": "GSQcQNtxfya44TjeSU5NyD113qft7YRLUKhZwXPRsdcC", "amount": "2000000000000000000000"}' \
		--accountId $(CONTRACT_NAME).$(ROOT_ACCOUNT)

.PHONY: delete-account
delete-account:
	near delete $(CONTRACT_NAME).$(ROOT_ACCOUNT) $(ROOT_ACCOUNT)

.PHONY: build
build:
	cargo build --target wasm32-unknown-unknown --release