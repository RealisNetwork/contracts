use crate::*;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_sdk::near_bindgen;

#[near_bindgen]
impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: "ft-1.0.1".to_string(),
            name: String::from("Realis Network XLIS token"),
            symbol: String::from("XLIS"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 12,
        }
    }
}
