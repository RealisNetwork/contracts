use near_contract_standards::fungible_token::{
    metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC},
    FungibleToken,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env,
    json_types::U128,
    near_bindgen, require, AccountId, Balance, PanicOnDefault, PromiseOrValue,
};

mod lis_token;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub ft: FungibleToken,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        let mut this = Self {
            ft: FungibleToken::new(b"a".to_vec()),
        };

        let account_id = env::predecessor_account_id();
        this.ft.internal_register_account(&account_id);
        this.ft
            .internal_deposit(&account_id, 3_000_000_000 * 10_u128.pow(12));

        this
    }

    pub fn register(&mut self, account_id: AccountId) {
        self.ft.internal_register_account(&account_id);
    }
}

near_contract_standards::impl_fungible_token_core!(Contract, ft);

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
