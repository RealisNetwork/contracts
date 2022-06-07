use crate::*;
use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider,
};

impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            // Should be equal to version in Cargo.toml and tag on github
            spec: "lis-ft-1.0.0".to_string(),
            name: "Realis LIS token".to_string(),
            symbol: "LIS".to_string(),
            icon: None,
            reference: None,
            reference_hash: None,
            decimals: 12, // TODO: ask Valentin or Sergei about this number
        }
    }
}

impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        NFTContractMetadata {
            spec: "nft-1.0.0".to_string(),
            name: "Cats Edition".to_string(),
            symbol: "CATS".to_string(),
            icon: None,
            base_uri: Some("https://realis.network/".to_string()),
            reference: None,
            reference_hash: None,
        }
    }
}
