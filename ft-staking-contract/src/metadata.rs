use crate::*;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider, FT_METADATA_SPEC,
};

impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_METADATA_SPEC.to_string(),
            name: String::from("Realis Network XLIS token"),
            symbol: String::from("XLIS"),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 12,
        }
    }
}
