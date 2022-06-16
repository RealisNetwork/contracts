CONTRACT_NAME = token_contract_v2
ROOT_ACCOUNT = ruslantahiiev.testnet

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

.PHONY: udeps
udeps:
	SKIP_WASM_BUILD=1 cargo +nightly udeps

.PHONY: pre_commit
pre_commit:
	cargo build --release
	cargo +nightly fmt
	cargo clippy -- -D warnings

# MANUAL TESTING
OWNER_ACC?=dev-1655368469513-67895817176815
ACC1?=ruslantahiiev.testnet
ACC2?=gleb_protasov.testnet


.PHONY: dev-deploy
dev-deploy:
	cargo build --target wasm32-unknown-unknown --release
	near dev-deploy target/wasm32-unknown-unknown/release/realis_near.wasm

.PHONY: dev-call-new
dev-call-new:
	near call $(OWNER_ACC) \
		new '{"total_supply": "3000000", "constant_fee": "1", "percent_fee": '2', "beneficiary_id": "testnetacc.testnet", "backend_id": "testnetacc.testnet"}' \
		--accountId $(OWNER_ACC)

.PHONY: dev-call-mint
dev-call-mint:
	near call $(OWNER_ACC) mint '{"recipient_id":"$(ACC1)","nft_metadata":"SOME METADATA"}' --accountId $(OWNER_ACC)

.PHONY: dev-call-burn
dev-call-burn:
	near call $(OWNER_ACC) burn '{"nft_id":"10"}' --accountId $(ACC1)

.PHONY: dev-call-transfer-tokens
dev-call-transfer-tokens:
	near call $(OWNER_ACC) transfer '{"recipient_id":"$(ACC2)","amount":"200"}' --accountId $(ACC1)

.PHONY: dev-call-transfer-nft
dev-call-transfer-nft:
	near call $(OWNER_ACC) transfer_nft '{"recipient_id":"$(ACC2)","nft_id":"11"}' --accountId $(ACC1)

.PHONY: dev-call-buy-nft
dev-call-buy-nft:
	near call $(OWNER_ACC) buy_nft '{"nft_id":"12"}' --accountId $(OWNER_ACC)

.PHONY: dev-call-sell-nft
dev-call-sell-nft:
	near call $(OWNER_ACC) sell_nft '{"nft_id":"8","price":"100"}' --accountId $(ACC1)

.PHONY: dev-call-change_price_nft
dev-call-change_price_nft:
	near call $(OWNER_ACC) change_price '{"nft_id":"8","price":"909"}' --accountId $(ACC1)

.PHONY:
dev-call-start_auction:
	near call $(OWNER_ACC) auction '{"nft_id":"5","price":"100", "deadline": "1755297494081"}' --accountId $(ACC1)

.PHONY: dev-call-make_bid
dev-call-make_bid:
	near call $(OWNER_ACC) bid '{"nft_id":"5","price":"150"}' --accountId $(OWNER_ACC)

.PHONY: dev-call-confirm_deal
dev-call-confirm_deal:
	near call $(OWNER_ACC) confirm '{"nft_id":"5"}' --accountId $(ACC1)

.PHONY: dev-call-get_nft_info
dev-call-get_nft_info:
	near view $(OWNER_ACC) get_nft_info '{"nft_id":"8"}' --accountId $(OWNER_ACC)

.PHONY: dev-call-get_nft_price
dev-call-get_nft_price:
	near view $(OWNER_ACC) get_nft_price '{"nft_id":"8"}' --accountId $(OWNER_ACC)

.PHONY: dev-call-get_balance_info
dev-call-get_balance_info:
	near call $(OWNER_ACC) get_balance_info '{"account_id":"$(ACC2)"}' --accountId $(ACC1)

.PHONY: dev-call-get_account_info
dev-call-get_account_info:
	near call $(OWNER_ACC) get_account_info '{"account_id":"$(ACC1)"}' --accountId $(ACC1)

.PHONY: dev-call-lockups_info
dev-call-lockups_info:
	near call $(OWNER_ACC) get_lockups_info '{"account_id":"$(ACC1),"from_index":'0',"limit":'5'"}' --accountId $(ACC1)

.PHONY: dev-call-create_and_register_account
dev-call-create_and_register_account:
	near call $(OWNER_ACC) create_and_register_account '{"account_id":"my_account","new_public_key":"CqnRfKbFT8jN4joAu6HqJHT7RyJFKEH23L6vcsLkdxsL"}' --accountId $(OWNER_ACC)

.PHONY: dev-call-unregister_account
dev-call-unregister_account:
	near call $(OWNER_ACC) unregister_account '{"public_key":"CqnRfKbFT8jN4joAu6HqJHT7RyJFKEH23L6vcsLkdxsL"}' --accountId $(ACC1)


# TEST REGISTER!!!
.PHONY: dev-call_register
dev-call_register:
	near call dev-1655281896313-38697444886102 _register_account --accountId dev-1655281896313-38697444886102
