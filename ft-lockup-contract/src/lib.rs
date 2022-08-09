use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    assert_one_yocto,
    borsh::{self, maybestd::collections::HashSet, BorshDeserialize, BorshSerialize},
    collections::{LookupMap, UnorderedSet},
    env,
    json_types::{U128, U64},
    near_bindgen, require,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey, Gas, PanicOnDefault, Promise, PromiseOrValue,
};

pub mod ft_token_receiver;
pub mod lockup;

use crate::lockup::*;

pub type LockupIndex = u32;
pub const GAS_FOR_CLAIM_CALLBACK: Gas = Gas(10_000_000_000_000);
pub const GAS_FOR_FT_TRANSFER: Gas = Gas(25_000_000_000_000);

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    /// Token contract account id to receive tokens for lockup
    pub token_account_id: AccountId,
    /// Account IDs that can create new lockups.
    pub deposit_whitelist: UnorderedSet<AccountId>,
    /// All lockups
    pub lockups: LookupMap<LockupIndex, Lockup>,
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
    pub fn new(token_account_id: AccountId, deposit_whitelist: Vec<AccountId>) -> Self {
        let mut deposit_whitelist_set = UnorderedSet::new(StorageKey::DepositWhitelist);
        deposit_whitelist_set.extend(deposit_whitelist.into_iter().map(|a| a.into()));
        Self {
            lockups: LookupMap::new(StorageKey::Lockups),
            account_lockups: LookupMap::new(StorageKey::AccountLockups),
            token_account_id: token_account_id,
            deposit_whitelist: deposit_whitelist_set,
            index: 0,
        }
    }

    #[payable]
    pub fn claim(&mut self, index: LockupIndex) -> Promise {
        assert_one_yocto();
        let account_id = env::predecessor_account_id();
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
}

impl Contract {
    pub fn next_index(&mut self) -> LockupIndex {
        while self.lockups.contains_key(&self.index) {
            self.index += 1;
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
