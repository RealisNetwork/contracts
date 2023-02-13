use near_contract_standards::{fungible_token::receiver::FungibleTokenReceiver, upgrade::Ownable};
use near_sdk::{
    assert_one_yocto,
    borsh::{self, maybestd::collections::HashSet, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedMap, UnorderedSet},
    env,
    json_types::{U128, U64},
    near_bindgen, require,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue,
};

pub mod ft_token_receiver;
pub mod lockup;
pub mod update;
pub mod view;

use crate::lockup::*;

pub type LockupIndex = u32;
pub const GAS_FOR_CLAIM_CALLBACK: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER: Gas = Gas(25_000_000_000_000);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    pub owner_id: AccountId,
    /// Token contract account id to receive tokens for lockup
    pub token_account_id: AccountId,
    /// Account IDs that can create new lockups.
    pub deposit_whitelist: UnorderedSet<AccountId>,
    /// All lockups
    pub lockups: UnorderedMap<LockupIndex, Lockup>,
    /// Lockups indexes by AccountId
    pub account_lockups: LookupMap<AccountId, HashSet<LockupIndex>>,

    pub index: LockupIndex,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Lockups,
    AccountLockups,
    DepositWhitelist,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        owner_id: Option<AccountId>,
        token_account_id: AccountId,
        deposit_whitelist: Vec<AccountId>,
    ) -> Self {
        let mut deposit_whitelist_set = UnorderedSet::new(StorageKey::DepositWhitelist);
        deposit_whitelist_set.extend(deposit_whitelist.into_iter());
        Self {
            owner_id: owner_id.unwrap_or_else(env::predecessor_account_id),
            lockups: UnorderedMap::new(StorageKey::Lockups),
            account_lockups: LookupMap::new(StorageKey::AccountLockups),
            token_account_id,
            deposit_whitelist: deposit_whitelist_set,
            index: 0,
        }
    }

    #[payable]
    pub fn claim(&mut self, index: LockupIndex, account_id: Option<AccountId>) -> Promise {
        assert_one_yocto();
        let account_id = account_id.unwrap_or_else(env::predecessor_account_id);
        let account_lockups = self
            .account_lockups
            .get(&account_id)
            .unwrap_or_else(|| env::panic_str("No lockups found"));
        require!(
            account_lockups.contains(&index),
            "No such lockup for this account"
        );

        let mut lockup = self
            .lockups
            .get(&index)
            .unwrap_or_else(|| env::panic_str("No such lockup for this account"));

        let promise = lockup.claim(self.token_account_id.clone(), account_id, index);
        self.lockups.insert(&index, &lockup);

        promise
    }

    #[payable]
    pub fn extend_deposit_whitelist(&mut self, account_ids: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();
        self.deposit_whitelist.extend(account_ids.into_iter())
    }

    #[payable]
    pub fn reduce_deposit_whitelist(&mut self, account_ids: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();
        account_ids.into_iter().for_each(|account_id| {
            self.deposit_whitelist.remove(&account_id);
        });
    }
}

impl Contract {
    pub fn next_index(&mut self) -> LockupIndex {
        while self.lockups.get(&self.index).is_some() {
            self.index = self
                .index
                .checked_add(1)
                .unwrap_or_else(|| env::panic_str("Add will overflow"));
        }
        self.index
    }

    pub fn assert_deposit_whitelist(&self, account_id: &AccountId) {
        require!(
            self.deposit_whitelist.contains(account_id),
            "Not in deposit whitelist"
        );
    }
}
