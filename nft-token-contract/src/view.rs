use crate::*;
use near_contract_standards::non_fungible_token::{
    enumeration::NonFungibleTokenEnumeration, Token,
};
use near_sdk::{json_types::U128, near_bindgen, AccountId};

const NFT_VIEW_LIMIT: u64 = 50;

#[near_bindgen]
impl NonFungibleTokenEnumeration for Contract {
    /// Returns the total supply of non-fungible tokens as a string representing
    /// an unsigned 128-bit integer to avoid JSON number limit of 2^53.
    fn nft_total_supply(&self) -> U128 {
        U128(self.token_by_id.len() as u128)
    }

    /// Get a list of all tokens
    ///
    /// Arguments:
    /// * `from_index`: a string representing an unsigned 128-bit integer, representing the starting
    ///   index of tokens to return
    /// * `limit`: the maximum number of tokens to return
    ///
    /// Returns an array of Token objects, as described in Core standard
    fn nft_tokens(&self, from_index: Option<U128>, limit: Option<u64>) -> Vec<Token> {
        self.token_by_id
            .iter()
            .skip(from_index.map(|v| v.0).unwrap_or_default() as usize)
            .take(limit.unwrap_or(NFT_VIEW_LIMIT) as usize)
            .map(|(_, token)| token.into())
            .collect()
    }

    /// Get number of tokens owned by a given account
    ///
    /// Arguments:
    /// * `account_id`: a valid NEAR account
    ///
    /// Returns the number of non-fungible tokens owned by given `account_id` as
    /// a string representing the value as an unsigned 128-bit integer to avoid
    /// JSON number limit of 2^53.
    fn nft_supply_for_owner(&self, account_id: AccountId) -> U128 {
        self.tokens_per_owner
            .get(&account_id)
            .map(|tokens| tokens.len() as u128)
            .unwrap_or_default()
            .into()
    }

    /// Get list of all tokens owned by a given account
    ///
    /// Arguments:
    /// * `account_id`: a valid NEAR account
    /// * `from_index`: a string representing an unsigned 128-bit integer, representing the starting
    ///   index of tokens to return
    /// * `limit`: the maximum number of tokens to return
    ///
    /// Returns a paginated list of all tokens owned by this account
    fn nft_tokens_for_owner(
        &self,
        account_id: AccountId,
        from_index: Option<U128>,
        limit: Option<u64>,
    ) -> Vec<Token> {
        self.tokens_per_owner
            .get(&account_id)
            .map(|token_ids| {
                token_ids
                    .iter()
                    .skip(from_index.map(|v| v.0).unwrap_or_default() as usize)
                    .take(limit.unwrap_or(NFT_VIEW_LIMIT) as usize)
                    .filter_map(|id| self.token_by_id.get(&id).map(|token| token.into()))
                    .collect()
            })
            .unwrap_or_default()
    }
}
