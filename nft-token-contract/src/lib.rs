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
use token::{Token, VersionedToken};

pub mod approval;
pub mod nft_core;
pub mod receiver;
pub mod resolver;
pub mod token;
pub mod update;
pub mod view;

#[derive(BorshStorageKey, BorshDeserialize, BorshSerialize)]
pub enum StorageKey {
    TokenById,
    TokensPerOwner,
    AccountTokens { hash: Vec<u8> },
    TokenMetadata { hash: Vec<u8> },
    TokenApprovals { hash: Vec<u8> },
    LockedTokens,
    AccountLockedTokens { hash: Vec<u8> },
    MintAccounts,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize)]
pub struct Contract {
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub mint_accounts: UnorderedSet<AccountId>,
    pub token_by_id: UnorderedMap<TokenId, VersionedToken>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub locked_tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
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
        let mut this = Self {
            owner_id: owner_id.clone(),
            backend_id,
            mint_accounts: UnorderedSet::new(StorageKey::MintAccounts),
            token_by_id: UnorderedMap::new(StorageKey::TokenById),
            tokens_per_owner: LookupMap::new(StorageKey::TokensPerOwner),
            locked_tokens_per_owner: LookupMap::new(StorageKey::LockedTokens),
        };
        this.measure_nft_storage_usage();
        this.mint_accounts.insert(&owner_id);
        this
    }

    fn measure_nft_storage_usage(&mut self) {
        let initial_storage_usage = env::storage_usage();
        let token_id = "a".repeat(64);
        let token = Token {
            token_id: token_id.clone(),
            owner_id: AccountId::new_unchecked("a".repeat(64)),
            metadata: LazyOption::new(
                StorageKey::TokenMetadata {
                    hash: env::sha256(token_id.as_bytes()),
                },
                None,
            ),
            approved_account_ids: UnorderedMap::new(StorageKey::TokenApprovals {
                hash: env::sha256(token_id.as_bytes()),
            }),
            next_approval_id: 0,
        };
        self.token_by_id.insert(&token_id, &token.into());
        let nft_storage_usage = env::storage_usage() - initial_storage_usage;
        self.token_by_id.remove(&token_id);
        env::log_str(&format!(
            "{}",
            nft_storage_usage as u128 * env::storage_byte_cost()
        ));
    }

    /// Simple mint. Create token with a given `token_id` for `owner_id`.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than `contract.mint_accounts`
    #[payable]
    pub fn nft_mint(
        &mut self,
        token_id: TokenId,
        owner_id: AccountId,
        metadata: Option<TokenMetadata>,
        memo: Option<String>,
    ) {
        assert_one_yocto();
        require!(
            self.mint_accounts.contains(&env::predecessor_account_id()),
            "Predecessor must be one of mint_accounts"
        );
        require!(
            self.token_by_id.get(&token_id).is_none(),
            "Token with such id exists"
        );

        let mut token = Token {
            token_id: token_id.clone(),
            owner_id: owner_id.clone(),
            metadata: LazyOption::new(
                StorageKey::TokenMetadata {
                    hash: env::sha256(token_id.as_bytes()),
                },
                metadata.as_ref(),
            ),
            approved_account_ids: UnorderedMap::new(StorageKey::TokenApprovals {
                hash: env::sha256(token_id.as_bytes()),
            }),
            next_approval_id: 0,
        };

        let approval_id = token.next_approval_id();
        token
            .approved_account_ids
            .insert(&self.backend_id, &approval_id);

        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.insert(&token.token_id);
        self.tokens_per_owner.insert(&owner_id, &tokens_per_owner);
        self.token_by_id
            .insert(&token.token_id.clone(), &token.into());

        NftMint {
            owner_id: &owner_id,
            token_ids: &[&token_id],
            memo: memo.as_deref(),
        }
        .emit();
    }

    /// Simple burn. Remove a given `token_id` from current owner.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or
    ///  one of the approved accounts
    #[payable]
    pub fn nft_burn(&mut self, token_id: TokenId, approval_id: Option<u64>, memo: Option<String>) {
        assert_one_yocto();
        let mut token: Token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("No such token"))
            .into();
        require!(
            token.is_approved_or_owner(&env::predecessor_account_id(), approval_id),
            "Not enough permission"
        );
        token.clear();

        self.token_by_id.remove(&token_id);
        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.remove(&token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        NftBurn {
            owner_id: &token.owner_id,
            token_ids: &[&token_id],
            authorized_id: None,
            memo: memo.as_deref(),
        }
        .emit();
    }

    /// Transfer a given `token_id` from current owner to `receiver_id`.
    /// Same as nft_transfer but can be called only by backend
    /// and save approval on this nft for backend account
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than `contract.backend_id` or,
    ///  if `contract.backend_id` not one of the approved accounts
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

        let token = self.get_token_internal(&token_id);

        require!(
            token.is_approved_or_owner(&env::predecessor_account_id(), approval_id),
            "Not enough permission"
        );
        self.nft_transfer_internal(&token_id, Some(token), receiver_id);

        // Save backend account as approval account for this token
        let mut token = self.get_token_internal(&token_id);
        let approval_id = token.next_approval_id();
        token
            .approved_account_ids
            .insert(&self.backend_id, &approval_id);
        self.token_by_id.insert(&token_id, &token.into());
    }

    /// Lock token by a given `token_id`. Remove given `token_id` from
    /// list of "free" tokens and add to locked.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than token owner or
    ///  one of the approved accounts
    #[payable]
    pub fn nft_lock(&mut self, token_id: TokenId, approval_id: Option<u64>) {
        assert_one_yocto();
        let token: Token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("No such token"))
            .into();

        require!(
            token.is_approved_or_owner(&env::predecessor_account_id(), approval_id)
                || env::predecessor_account_id() == token.owner_id,
            "Not enough permission"
        );

        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        require!(
            tokens_per_owner.remove(&token_id),
            "Token is already locked"
        );
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        let mut locked_tokens_per_owner =
            self.get_locked_tokens_per_owner_internal(&token.owner_id);
        locked_tokens_per_owner.insert(&token_id);
        self.locked_tokens_per_owner
            .insert(&token.owner_id, &locked_tokens_per_owner);
    }

    /// Unlock token by a given `token_id`. Remove given `token_id` from
    /// locked and add to "Free"
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than backend account
    #[payable]
    pub fn nft_unlock(&mut self, token_id: TokenId) {
        assert_one_yocto();
        require!(
            env::predecessor_account_id() == self.backend_id,
            "Predecessor must be backend account"
        );

        let token: Token = self
            .token_by_id
            .get(&token_id)
            .unwrap_or_else(|| env::panic_str("No such token"))
            .into();

        let mut locked_tokens_per_owner =
            self.get_locked_tokens_per_owner_internal(&token.owner_id);
        require!(
            locked_tokens_per_owner.remove(&token_id),
            "Token not locked"
        );
        self.locked_tokens_per_owner
            .insert(&token.owner_id, &locked_tokens_per_owner);

        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.insert(&token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);
    }

    /// Unlock token by a given `token_id`. Remove given `token_id` from
    /// locked and transfer to `receiver_id`.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than backend account
    #[payable]
    pub fn nft_unlock_and_transfer_backend(&mut self, token_id: TokenId, receiver_id: AccountId) {
        self.nft_unlock(token_id.clone());
        self.nft_transfer_backend(receiver_id, token_id, None, None);
    }

    /// Add new accounts that have permissions to call `nft_mint`.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than contract owner
    #[payable]
    pub fn add_mint_accounts(&mut self, account_ids: Vec<AccountId>) {
        assert_one_yocto();

        require!(
            env::predecessor_account_id() == self.owner_id,
            "Predecessor must be contract owner"
        );

        self.mint_accounts.extend(account_ids);
    }

    /// Remove accounts that have permissions to call `nft_mint`.
    ///
    /// Requirements
    /// * Caller of the method must attach a deposit of 1 yoctoⓃ for security purposes
    /// * Contract MUST panic if called by someone other than contract owner
    #[payable]
    pub fn remove_mint_accounts(&mut self, account_ids: Vec<AccountId>) {
        assert_one_yocto();

        require!(
            env::predecessor_account_id() == self.owner_id,
            "Predecessor must be contract owner"
        );

        account_ids.iter().for_each(|account_id| {
            self.mint_accounts.remove(account_id);
        });
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

    fn get_locked_tokens_per_owner_internal(
        &self,
        account_id: &AccountId,
    ) -> UnorderedSet<TokenId> {
        self.locked_tokens_per_owner
            .get(account_id)
            .unwrap_or_else(|| {
                UnorderedSet::new(StorageKey::AccountLockedTokens {
                    hash: env::sha256(account_id.as_bytes()),
                })
            })
    }

    fn get_token_internal(&self, token_id: &TokenId) -> Token {
        self.token_by_id
            .get(token_id)
            .unwrap_or_else(|| env::panic_str("Token not found"))
            .into()
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

        token.owner_id = receiver_id.clone();
        token.approved_account_ids.clear();

        let mut tokens_per_owner = self.get_tokens_per_owner_internal(&token.owner_id);
        tokens_per_owner.insert(token_id);
        self.tokens_per_owner
            .insert(&token.owner_id, &tokens_per_owner);

        self.token_by_id.insert(token_id, &token.into());

        NftTransfer {
            old_owner_id: &old_owner_id,
            new_owner_id: &receiver_id,
            token_ids: &[token_id],
            authorized_id: None,
            memo: None,
        }
        .emit();
    }
}

#[cfg(test)]
mod tests {
    use crate::*;
    use near_contract_standards::non_fungible_token::{
        approval::NonFungibleTokenApproval, core::NonFungibleTokenCore,
        enumeration::NonFungibleTokenEnumeration,
    };
    use near_sdk::{
        json_types::U128,
        test_utils::{accounts, VMContextBuilder},
        testing_env, ONE_YOCTO,
    };

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_mint_assert_one_yocto() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);
    }

    #[test]
    #[should_panic = "Predecessor must be one of mint_accounts"]
    fn nft_mint_should_panic_if_called_not_by_contract_owner() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);
    }

    #[test]
    #[should_panic = "Token with such id exists"]
    fn nft_mint_shoul_panic_if_mint_token_with_same_token_id() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        contract.nft_mint("test".into(), accounts(1), None, None);
        contract.nft_mint("test".into(), accounts(2), None, None);
    }

    #[test]
    fn nft_mint() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        contract.nft_mint("test".into(), accounts(1), None, None);

        assert_eq!(contract.nft_total_supply(), U128(1));
        assert_eq!(contract.nft_supply_for_owner(accounts(1)), U128(1));
        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
        let token = option_token.unwrap();
        assert_eq!(token.token_id, "test");
        assert_eq!(token.owner_id, accounts(1));
        assert!(token.metadata.is_none());
        assert!(token
            .approved_account_ids
            .unwrap()
            .contains_key(&contract.backend_id));
    }

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_burn_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_burn("test".into(), None, None);
    }

    #[test]
    #[should_panic = "Not enough permission"]
    fn nft_burn_should_panic_if_called_not_by_token_owner() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_burn("test".into(), None, None);
    }

    #[test]
    fn nft_burn() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_burn("test".into(), None, None);

        assert_eq!(contract.nft_total_supply(), U128(0));
        assert_eq!(contract.nft_supply_for_owner(accounts(0)), U128(0));
        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_none());
    }

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_lock_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_lock("test".into(), None);
    }

    #[test]
    #[should_panic = "Not enough permission"]
    fn nft_lock_should_panic_if_called_not_by_token_owner() {
        let mut contract = Contract::new(Some(accounts(0)), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(1), None, None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        contract.nft_lock("test".into(), None);
    }

    #[test]
    fn nft_lock() {
        let mut contract = Contract::new(Some(accounts(0)), None);
        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();

        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_lock("test".into(), None);

        assert_eq!(contract.nft_total_supply(), U128(1));
        assert_eq!(contract.nft_supply_for_owner(accounts(0)), U128(0));
        assert_eq!(contract.nft_locked_supply_per_owner(accounts(0)), U128(1));
        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
    }

    #[test]
    #[should_panic = "Requires attached deposit of exactly 1 yoctoNEAR"]
    fn nft_transfer_backend_assert_one_yocto() {
        let mut contract = Contract::default();
        let context = VMContextBuilder::new().attached_deposit(0).build();

        testing_env!(context);
        contract.nft_transfer_backend(accounts(0), "test".into(), None, None);
    }

    #[test]
    #[should_panic = "Predecessor must be backend account"]
    fn nft_transfer_backend_should_panic_if_called_not_by_backend_account() {
        let mut contract = Contract::new(Some(accounts(0)), Some(accounts(1)));

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);

        contract.nft_transfer_backend(accounts(2), "test".into(), None, None);
    }

    #[test]
    #[should_panic = "Not enough permission"]
    fn nft_transfer_backend_should_panic_if_backend_account_not_approved() {
        let mut contract = Contract::new(Some(accounts(0)), Some(accounts(1)));

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_revoke_all("test".into());

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_transfer_backend(accounts(2), "test".into(), None, None);
    }

    #[test]
    fn nft_transfer_backend() {
        let mut contract = Contract::new(Some(accounts(0)), Some(accounts(1)));

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(0))
            .build();
        testing_env!(context);
        contract.nft_mint("test".into(), accounts(0), None, None);
        contract.nft_approve("test".into(), accounts(1), None);

        let context = VMContextBuilder::new()
            .attached_deposit(ONE_YOCTO)
            .predecessor_account_id(accounts(1))
            .build();

        testing_env!(context);
        contract.nft_transfer_backend(accounts(2), "test".into(), None, None);

        assert_eq!(contract.nft_total_supply(), U128(1));
        assert_eq!(contract.nft_supply_for_owner(accounts(2)), U128(1));
        let option_token = contract.nft_token("test".into());
        assert!(option_token.is_some());
        let token = option_token.unwrap();
        assert_eq!(token.token_id, "test");
        assert_eq!(token.owner_id, accounts(2));
        assert!(token.metadata.is_none());
        assert!(token
            .approved_account_ids
            .unwrap()
            .contains_key(&accounts(1)))
    }
}
