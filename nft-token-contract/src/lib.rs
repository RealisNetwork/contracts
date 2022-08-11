use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::{
    assert_one_yocto,
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    env, near_bindgen, require, AccountId, PanicOnDefault,
};
use token::Token;

pub mod approval;
pub mod core;
pub mod receiver;
pub mod resolver;
pub mod token;
pub mod view;

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub token_by_id: UnorderedMap<TokenId, Token>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
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
            token_by_id: UnorderedMap::new(b"a"),
            tokens_per_owner: LookupMap::new(b"b"),
        }
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
            "Not enought permission"
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
    } // LGTM
}

impl Contract {
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

        let mut tokens_per_owner = self
            .tokens_per_owner
            .get(&token.owner_id)
            .unwrap_or_else(|| env::panic_str("Account not found"));
        tokens_per_owner.remove(token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        token.owner_id = receiver_id;

        let mut tokens_per_owner = self
            .tokens_per_owner
            .get(&token.owner_id)
            .unwrap_or_else(|| UnorderedSet::new(env::sha256(token.owner_id.as_bytes())));
        tokens_per_owner.insert(token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        self.token_by_id.insert(token_id, &token);
    }
}
