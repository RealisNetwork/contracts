use near_contract_standards::non_fungible_token::{
    metadata::TokenMetadata, NonFungibleToken, Token, TokenId,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, near_bindgen, require, AccountId, PanicOnDefault, Promise, PromiseOrValue,
};

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub nft: NonFungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let owner_id = env::predecessor_account_id();
        Self {
            nft: NonFungibleToken::new(
                b"a".to_vec(),
                owner_id,
                Some(b"b".to_vec()),
                Some(b"c".to_vec()),
                Some(b"d".to_vec()),
            ),
        }
    }

    // TODO: move to separete file, maybe
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        account_id: AccountId,
        metadata: Option<TokenMetadata>,
    ) {
        require!(
            env::predecessor_account_id() == self.nft.owner_id,
            "Not enough permission"
        );
        self.nft.internal_mint(token_id, account_id, metadata);
    }

    pub fn nft_burn(&mut self, token_id: TokenId) {
        let owner_id = self
            .nft
            .owner_by_id
            .remove(&token_id)
            .unwrap_or_else(|| env::panic_str("No such token"));
        require!(
            env::predecessor_account_id() == owner_id,
            "Not enough permission"
        );
        self.nft
            .token_metadata_by_id
            .as_mut()
            .and_then(|by_id| by_id.remove(&token_id));

        self.nft
            .tokens_per_owner
            .as_mut()
            .and_then(|by_id| by_id.remove(&owner_id));

        self.nft
            .approvals_by_id
            .as_mut()
            .and_then(|by_id| by_id.remove(&token_id));

        // TODO: emit event
    }
}

near_contract_standards::impl_non_fungible_token_core!(Contract, nft);
near_contract_standards::impl_non_fungible_token_approval!(Contract, nft);
near_contract_standards::impl_non_fungible_token_enumeration!(Contract, nft);
