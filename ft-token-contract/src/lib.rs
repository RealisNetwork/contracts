use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC},
    FungibleToken,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseOrValue, Timestamp,
};

mod ft_core;
mod lis_token;
mod storage_impl;
mod update;

pub const DEFAULT_MINT_AMOUNT: u128 = 3_000_000_000 * 10_u128.pow(12);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub staking_contract: AccountId,
    pub ft: FungibleToken,
    pub last_mint: Timestamp,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: Option<AccountId>, staking_id: AccountId) -> Self {
        let owner_id = owner_id.unwrap_or_else(env::predecessor_account_id);
        let mut this = Self {
            owner_id: owner_id.clone(),
            staking_contract: staking_id.clone(),
            ft: FungibleToken::new(b"a".to_vec()),
            last_mint: env::block_timestamp(),
        };

        this.ft.internal_register_account(&owner_id);
        this.ft.internal_register_account(&staking_id);

        this.ft.internal_deposit(&owner_id, DEFAULT_MINT_AMOUNT);
        near_contract_standards::fungible_token::events::FtMint {
            owner_id: &owner_id,
            amount: &DEFAULT_MINT_AMOUNT.into(),
            memo: None,
        }
        .emit();

        this
    }

    pub fn register(&mut self, account_id: AccountId) {
        self.ft.internal_register_account(&account_id);
    }
}

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Realis Network LIS token"),
            symbol: String::from("LIS"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 12,
        }
    }
}
