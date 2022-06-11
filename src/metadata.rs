use near_contract_standards::{
    fungible_token::metadata::{FungibleTokenMetadata, FungibleTokenMetadataProvider},
    non_fungible_token::{
        enumeration::NonFungibleTokenEnumeration,
        metadata::{NFTContractMetadata, NonFungibleTokenMetadataProvider},
        Token,
    },
};
use near_sdk::{
    env,
    json_types::{Base64VecU8, U128},
    AccountId,
};

use crate::Contract;

/// `SPEC_TOKEN` a string.
/// Should be ft-1.0.0 to indicate that a Fungible Token contract
/// adheres to the current versions of this Metadata and
/// the Fungible Token Core specs. This will allow consumers
/// of the Fungible Token to know if they support the features of a given
/// contract.
pub const FT_SPEC_TOKEN: &str = "ft-0.1.0";
pub const NFT_SPEC_TOKEN: &str = "nft-0.1.0";
/// `TOKEN_NAME` the human-readable name of the token.
pub const FT_TOKEN_NAME: &str = "Realis";
pub const NFT_TOKEN_NAME: &str = "Realis NFT";
/// `TOKEN_REFERENCE`a link to a valid JSON file containing
/// various keys offering supplementary details on the token.
/// Example: /ipfs/QmdmQXB2mzChmMeKY47C43LxUdg1NDJ5MWcKMKxDu7RgQm,
/// https://example.com/token.json, etc.
/// If the information given in this document conflicts with the on-chain
/// attributes, the values in reference shall be considered the source of truth.
pub const FT_TOKEN_SYMBOL: &str = "LIS";
pub const FT_TOKEN_ICON: &str = "";
pub const NFT_TOKEN_ICON: &str = "";
pub const NFT_TOKEN_SYMBOL: &str = "LIS";
/// `TOKEN_REFERENCE` URL to an off-chain JSON file with more info.
pub const FT_TOKEN_REFERENCE: &str = "";
pub const NFT_TOKEN_REFERENCE: &str = "";
/// Used in frontends to show the proper significant digits of a token.
pub const FT_TOKEN_DECIMALS: u8 = 12;
/// Centralized gateway known to have reliable access to decentralized storage
/// assets referenced by reference or media URLs. Can be used by other frontends
/// for initial retrieval of assets, even if these frontends then replicate the
/// data to their own decentralized nodes, which they are encouraged to do.
pub const NFT_BASE_URI: &str = "";

impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_SPEC_TOKEN.to_owned(),
            name: FT_TOKEN_NAME.to_owned(),
            symbol: FT_TOKEN_SYMBOL.to_owned(),
            icon: Some(FT_TOKEN_ICON.to_owned()),
            reference: Some(FT_TOKEN_REFERENCE.to_owned()),
            // the base64-encoded sha256 hash of the JSON file contained in the reference field.
            // This is to guard against off-chain tampering.
            reference_hash: Some(Base64VecU8::from(env::sha256(
                FT_TOKEN_REFERENCE.as_bytes(),
            ))),
            decimals: FT_TOKEN_DECIMALS,
        }
    }
}

impl NonFungibleTokenMetadataProvider for Contract {
    fn nft_metadata(&self) -> NFTContractMetadata {
        NFTContractMetadata {
            spec: NFT_SPEC_TOKEN.to_owned(),
            name: NFT_TOKEN_NAME.to_owned(),
            symbol: NFT_TOKEN_SYMBOL.to_owned(),
            icon: Some(NFT_TOKEN_ICON.to_owned()),
            base_uri: Some(NFT_BASE_URI.to_owned()),
            reference: Some(NFT_TOKEN_REFERENCE.to_owned()),
            // the base64-encoded sha256 hash of the JSON file contained in the reference field.
            // This is to guard against off-chain tampering.
            reference_hash: Some(Base64VecU8::from(env::sha256(
                NFT_TOKEN_REFERENCE.as_bytes(),
            ))),
        }
    }
}

impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> U128 {
        U128::from(self.nfts.nft_count() as u128)
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        // Start index
        let from = from_index.unwrap_or_else(|| U128::from(0));
        // Limit
        let limit = limit.unwrap_or_else(|| self.nfts.nft_count());
        self.nfts
            .get_nft_map()
            .iter()
            .skip(from.0 as usize)
            .take(limit as usize)
            .map(|(key, value)| Token {
                token_id: key.to_string(),
                owner_id: value.owner_id,
                metadata: None,
                approved_account_ids: None,
            })
            .collect()
    }

    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        let count = self
            .nfts
            .get_nft_map()
            .values()
            .filter(|value| value.owner_id == account_id)
            .count();
        U128::from(count as u128)
    }

    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        // Start index
        let from = from_index.unwrap_or_else(|| U128::from(0));
        // Limit
        let limit = limit.unwrap_or_else(|| self.nfts.nft_count());
        self.nfts
            .get_nft_map()
            .iter()
            .filter(|(_key, value)| value.owner_id == account_id)
            .skip(from.0 as usize)
            .take(limit as usize)
            .map(|(key, value)| Token {
                token_id: key.to_string(),
                owner_id: account_id.clone(),
                metadata: None,
                approved_account_ids: None,
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
    use near_sdk::{
        collections::{LookupMap, UnorderedMap, UnorderedSet},
        json_types::U128,
        test_utils::VMContextBuilder,
        testing_env, AccountId, Gas, RuntimeFeesConfig, VMConfig, VMContext,
    };

    use crate::{Contract, Nft, NftMap, State};

    pub fn get_contract() -> Contract {
        let mut contract = Contract {
            constant_fee: 0,
            percent_fee: 0,
            accounts: LookupMap::new(b"m"),
            nfts: NftMap::new(),
            owner_id: AccountId::new_unchecked("id".to_string()),
            backend_id: AccountId::new_unchecked("id".to_string()),
            beneficiary_id: AccountId::new_unchecked("id".to_string()),
            state: State::Paused,
            registered_accounts: LookupMap::new(b"a"),
        };
        for i in 0..10 {
            contract.nfts.mint_nft(
                AccountId::new_unchecked(format!("id_{}", i)),
                "some".to_string(),
            );
        }

        contract
    }

    pub fn get_context(caller_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::new_unchecked(caller_id))
            .is_view(false)
            .build()
    }

    #[test]
    fn test_nft_total_supply() {
        let contract = get_contract();
        let context = get_context("id".to_string());
        testing_env!(context, VMConfig::free(), RuntimeFeesConfig::free());
        let result = contract.nft_total_supply();
        assert_eq!(result, U128::from(10))
    }

    #[test]
    fn test_nft_tokens() {
        let contract = get_contract();
        let context = get_context("id".to_string());
        testing_env!(context, VMConfig::free(), RuntimeFeesConfig::free());
        assert_eq!(contract.nft_tokens(Some(U128::from(5)), Some(2)).len(), 2);
        assert_eq!(contract.nft_tokens(Some(U128::from(9)), Some(2)).len(), 1);
    }

    #[test]
    fn test_nft_supply_for_owner() {
        let contract = get_contract();
        let context = get_context("id".to_string());
        testing_env!(context, VMConfig::free(), RuntimeFeesConfig::free());
        let result = contract.nft_supply_for_owner(AccountId::new_unchecked("id_4".to_string()));
        assert_eq!(result, U128::from(1))
    }

    #[test]
    fn test_nft_tokens_for_owner() {
        let contract = get_contract();
        let context = get_context("id".to_string());
        testing_env!(context, VMConfig::free(), RuntimeFeesConfig::free());
        let result =
            contract.nft_tokens_for_owner(AccountId::new_unchecked("id_4".to_string()), None, None);
        assert_eq!(result.len(), 1)
    }
}
