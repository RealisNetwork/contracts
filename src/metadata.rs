use near_contract_standards::fungible_token::metadata::{
    FungibleTokenMetadata, FungibleTokenMetadataProvider,
};
use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
use near_contract_standards::non_fungible_token::metadata::{
    NFTContractMetadata, NonFungibleTokenMetadataProvider,
};
use near_contract_standards::non_fungible_token::Token;
use near_sdk::json_types::{Base64VecU8, U128};
use near_sdk::{env, AccountId};

use crate::Contract;

/// `SPEC_TOKEN` current version of token in format `nft-n.n.n`,
/// where nnn is number of version.
pub const FT_SPEC_TOKEN: &str = "ft-0.1.0";
pub const NFT_SPEC_TOKEN: &str = "nft-0.1.0";
/// `TOKEN_NAME` fool length token name.
pub const FT_TOKEN_NAME: &str = "Realis";
pub const NFT_TOKEN_NAME: &str = "Realis NFT";
/// `TOKEN_SYMBOL` short token name.
pub const FT_TOKEN_SYMBOL: &str = "LIS";
pub const NFT_TOKEN_SYMBOL: &str = "LIS";
pub const FT_ICON: &str = "";
pub const NFT_ICON: &str = "";
/// `TOKEN_REFERENCE` URL to an off-chain JSON file with more info.
pub const FT_TOKEN_REFERENCE: &str = "";
pub const NFT_TOKEN_REFERENCE: &str = "";
pub const FT_TOKEN_DECIMALS: u8 = 0;
pub const NFT_BASE_URI: &str = "";

impl FungibleTokenMetadataProvider for Contract {
    fn ft_metadata(&self) -> FungibleTokenMetadata {
        FungibleTokenMetadata {
            spec: FT_SPEC_TOKEN.to_owned(),
            name: FT_TOKEN_NAME.to_owned(),
            symbol: FT_TOKEN_SYMBOL.to_owned(),
            icon: Some(FT_ICON.to_owned()),
            reference: Some(FT_TOKEN_REFERENCE.to_owned()),
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
            icon: Some(NFT_ICON.to_owned()),
            base_uri: Some(NFT_BASE_URI.to_owned()),
            reference: Some(NFT_TOKEN_REFERENCE.to_owned()),
            reference_hash: Some(Base64VecU8::from(env::sha256(
                NFT_TOKEN_REFERENCE.as_bytes(),
            ))),
        }
    }
}

impl NonFungibleTokenEnumeration for Contract {
    fn nft_total_supply(&self) -> U128 {
        U128::from(self.nfts.len() as u128)
    }

    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        // Start index
        let from = from_index.unwrap_or(U128::from(0));
        // Limit
        let limit = limit.unwrap_or_else(|| self.nfts.len());
        self.nfts
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
        let from = from_index.unwrap_or(U128::from(0));
        // Limit
        let limit = limit.unwrap_or_else(|| self.nfts.len());
        self.nfts
            .iter()
            .filter(|(_key, value)| value.owner_id == account_id)
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
}

#[cfg(test)]
mod tests {
    use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
    use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, AccountId, Gas, RuntimeFeesConfig, VMConfig, VMContext};

    use crate::{Contract, Nft, State};

    pub fn get_contract() -> Contract {
        let mut contract = Contract {
            constant_fee: 0,
            percent_fee: 0,
            accounts: LookupMap::new(b"m"),
            nfts: UnorderedMap::new(b"s"),
            owner_id: AccountId::new_unchecked("id".to_string()),
            backend_id: AccountId::new_unchecked("id".to_string()),
            beneficiary_id: AccountId::new_unchecked("id".to_string()),
            state: State::Paused,
            nft_id_counter: 10,
            registered_accounts: LookupMap::new(b"a"),
        };
        for i in 0..10 {
            let nft = Nft {
                owner_id: AccountId::new_unchecked(format!("id_{}", i)),
                metadata: "some".to_string(),
            };
            contract.nfts.insert(&i, &nft);
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
