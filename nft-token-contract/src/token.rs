use near_contract_standards::non_fungible_token::{metadata::TokenMetadata, TokenId};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LazyOption, UnorderedMap},
    env, AccountId,
};

#[derive(BorshDeserialize, BorshSerialize)]
pub enum VersionedToken {
    V1(Token),
}

impl From<VersionedToken> for Token {
    fn from(value: VersionedToken) -> Self {
        match value {
            VersionedToken::V1(token) => token,
        }
    }
}

impl From<Token> for VersionedToken {
    fn from(value: Token) -> Self {
        Self::V1(value)
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct Token {
    pub token_id: TokenId,
    pub owner_id: AccountId,
    pub metadata: LazyOption<TokenMetadata>,
    pub approved_account_ids: UnorderedMap<AccountId, u64>,
    pub next_approval_id: u64,
}

impl From<Token> for near_contract_standards::non_fungible_token::Token {
    fn from(token: Token) -> Self {
        Self {
            token_id: token.token_id,
            owner_id: token.owner_id,
            metadata: token.metadata.get(),
            approved_account_ids: Some(token.approved_account_ids.iter().collect()),
        }
    }
}

impl Token {
    /// TODO: add fn internal_nft_approve
    pub fn next_approval_id(&mut self) -> u64 {
        self.next_approval_id
            .checked_add(1)
            .unwrap_or_else(|| env::panic_str("Add will overflow"));
        self.next_approval_id
    }

    pub fn is_approved(&self, approved_account_id: &AccountId, approval_id: Option<u64>) -> bool {
        self.approved_account_ids
            .get(approved_account_id)
            .map(|id| {
                approval_id
                    .map(|approval_id| approval_id == id)
                    .unwrap_or(true)
            })
            .unwrap_or(false)
    }

    pub fn check_approve_and_revoke_all(
        &mut self,
        account_id: &AccountId,
        approval_id: Option<u64>,
    ) -> bool {
        if &self.owner_id == account_id {
            return true;
        }

        if self.is_approved(account_id, approval_id) {
            self.approved_account_ids.clear();
            return true;
        }

        false
    }

    pub fn clear(&mut self) {
        self.approved_account_ids.clear();
        self.metadata.remove();
    }
}
