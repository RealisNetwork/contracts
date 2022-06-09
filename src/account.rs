use crate::{NftId, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::Balance;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V1(Account),
}

impl Default for VAccount {
    fn default() -> Self {
        VAccount::V1(Account::default())
    }
}

impl From<VAccount> for Account {
    fn from(vaccount: VAccount) -> Self {
        match vaccount {
            VAccount::V1(account) => account,
        }
    }
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

        }
    }
}

impl From<Account> for VAccount {
    fn from(account: Account) -> Self {
        VAccount::V1(account)
    }
}

impl Default for Account {
    fn default() -> Self {
        Self::new(0)
    }
}
