use near_contract_standards::non_fungible_token::{
    events::{NftBurn, NftMint, NftTransfer},
    metadata::TokenMetadata,
    TokenId,
};
use near_sdk::{
    assert_one_yocto,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LazyOption, LookupMap, UnorderedMap, UnorderedSet},
    env, near_bindgen, require, AccountId, BorshStorageKey,
};
use token::Token;

pub mod approval;
pub mod nft_core;
pub mod receiver;
pub mod resolver;
pub mod token;
pub mod view;

#[derive(BorshStorageKey, BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    TokenById,
    TokensPerOwner,
    AccountTokens { hash: Vec<u8> },
    TokenMetadata { hash: Vec<u8> },
    TokenApprovals { hash: Vec<u8> },
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub token_by_id: UnorderedMap<TokenId, Token>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

impl Default for Contract {
    fn default() -> Self {
        Self::new(None, None)
    }
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(owner_id: Option<AccountId>, backend_id: Option<AccountId>) -> Self {
        let owner_id = owner_id.unwrap_or_else(env::predecessor_account_id);
        let backend_id = backend_id.unwrap_or_else(env::predecessor_account_id);
        Self {
            owner_id,
            backend_id,
            token_by_id: UnorderedMap::new(StorageKey::TokenById),
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner),
        }
    }

    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        metadata: Option<TokenMetadata>,
    ) {
        assert_one_yocto();
        require!(
            env::predecessor_account_id() == self.owner_id,
            "Predecessor must be contract owner"
        );
        require!(
            self.token_by_id.get(&token_id).is_none(),
            "Token with such id exists"
        );

        let token = Token {
            token_id: token_id.clone(),
            owner_id: owner_id.clone(),
            metadata: LazyOption::new(
                StorageKey::TokenMetadata {
                    hash: env::sha256(owner_id.as_bytes()),
                },
                metadata.as_ref(),
            ),
            approved_account_ids: UnorderedMap::new(StorageKey::TokenApprovals {
                hash: env::sha256(owner_id.as_bytes()),
            }),
            next_approval_id: 0,
        };

        self.token_by_id.insert(&token.token_id, &token);
        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.insert(&token.token_id);
        self.tokens_per_owner.insert(&owner_id, &tokens_per_owner);

        NftMint {
            owner_id: &owner_id,
            token_ids: &[&token_id],
            memo: None,
        }
        .emit();
    }

    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId) {
        assert_one_yocto();
        let owner_id = env::predecessor_account_id();
        let token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("No such token"));
        require!(
            token.owner_id == owner_id,
            "Predecessor must be token owner"
        );

        self.token_by_id.remove(&token_id);
        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.remove(&token_id);
        self.tokens_per_owner.insert(&owner_id, &tokens_per_owner);

        NftBurn {
            owner_id: &owner_id,
            token_ids: &[&token_id],
            authorized_id: None,
            memo: None,
        }
        .emit();
    }

    /// Transfer a given `token_id` from current owner to `receiver_id`.
    /// Same as nft_transfer but can be called only by backend
    /// and save approval on this nft for backend account
    #[payable]
    pub fn nft_transfer_backend(
        &mut self,
        receiver_id: AccountId,
        token_id: TokenId,
        approval_id: Option<u64>,
        #[allow(unused)] memo: Option<String>,
    ) {
        assert_one_yocto();
        require!(
            env::predecessor_account_id() == self.backend_id,
            "Predecessor must be backend account"
        );

        let mut token = self.get_token_internal(&token_id);

        require!(
            token.check_approve_and_revoke_all(&env::predecessor_account_id(), approval_id),
            "Not enought permission"
        );
        let approval_id = token.next_approval_id();
        token
            .approved_account_ids
            .insert(&self.backend_id, &approval_id);

        self.nft_transfer_internal(&token_id, Some(token), receiver_id);
    }
}

impl Contract {
    fn get_tokens_per_owner_internal(&self, account_id: &AccountId) -> UnorderedSet<TokenId> {
        self.tokens_per_owner.get(account_id).unwrap_or_else(|| {
            UnorderedSet::new(StorageKey::AccountTokens {
                hash: env::sha256(account_id.as_bytes()),
            })
        })
    }

    fn get_token_internal(&self, token_id: &TokenId) -> Token {
        self.token_by_id
            .get(token_id)
            .unwrap_or_else(|| env::panic_str("Token not found"))
    }

    fn nft_transfer_internal(
        &mut self,
        token_id: &TokenId,
        token: Option<Token>,
        receiver_id: AccountId,
    ) {
        let mut token = token.unwrap_or_else(|| self.get_token_internal(token_id));
        let old_owner_id = token.owner_id.clone();
        let mut tokens_per_owner = self
            .tokens_per_owner
            .get(&token.owner_id)
            .unwrap_or_else(|| env::panic_str("Account not found"));
        tokens_per_owner.remove(token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        token.owner_id = receiver_id;

        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.insert(token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        self.token_by_id.insert(token_id, &token);

        NftTransfer {
            old_owner_id: &old_owner_id,
            new_owner_id: &token.owner_id,
            token_ids: &[token_id],
            authorized_id: None,
            memo: None,
        }
        .emit();
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn nft_mint_assert_one_yocto() {
        todo!()
    }

    #[test]
    #[should_panic = "Predecessor must be contract owner"]
    fn nft_mint_should_panic_if_called_not_by_contract_owner() {
        todo!()
    }

    #[test]
    #[should_panic = "Token with such id exists"]
    fn nft_mint_shoul_panic_if_mint_token_with_same_token_id() {
        todo!()
    }

    #[test]
    fn nft_mint() {
        todo!()
    }

    #[test]
    fn nft_burn_assert_one_yocto() {
        todo!()
    }

    #[test]
    #[should_panic = "Predecessor must be token owner"]
    fn nft_burn_should_panic_if_called_not_by_token_owner() {
        todo!()
    }

    #[test]
    fn nft_burn() {
        todo!()
    }

    #[test]
    fn nft_transfer_backend_assert_one_yocto() {
        todo!()
    }

    #[test]
    #[should_panic = "Predecessor must be backend account"]
    fn nft_transfer_backend_should_panic_if_called_not_by_backend_account() {
        todo!()
    }

    #[test]
    #[should_panic = "Not enought permission"]
    fn nft_transfer_backend_should_panic_if_backend_account_not_approved() {
        todo!()
    }

    #[test]
    fn nft_transfer_backend() {
        todo!()
    }
}
