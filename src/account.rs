use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::Balance;

use crate::{NftId, Serialize, StorageKey};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V1(Account),
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub free: Balance,
    // pub lockups: Vec<Lock>,
    pub nfts: LookupSet<NftId>,
}

impl Account {
    pub fn new(balance: Balance) -> Self {
        Self {
            free: balance,
            // lockups: vec![],
            nfts: LookupSet::new(StorageKey::NftId),
        }
    }
}

impl From<Account> for VAccount {
    fn from(account: Account) -> Self {
        VAccount::V1(account)
    }
}

impl Default for VAccount {
    fn default() -> Self {
        VAccount::V1(Account {
            free: 0,
            nfts: LookupSet::new(StorageKey::NftId),
        })
    }
}
