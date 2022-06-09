use crate::lockup::Lockup;
use crate::{LockupInfo, NftId, Serialize, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedSet};
use near_sdk::json_types::U128;
use near_sdk::Balance;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V1(Account),
}

impl Default for VAccount {
    fn default() -> Self {
        VAccount::V1(Account {
            free: 0,
            lockups: UnorderedSet::new(b'l'),
            nfts: LookupSet::new(StorageKey::NftId),
        })
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
    pub lockups: UnorderedSet<Lockup>,
    pub nfts: LookupSet<NftId>,
}

impl Account {
    pub fn new(balance: Balance) -> Self {
        Self {
            free: balance,
            lockups: UnorderedSet::new(b'l'),
            nfts: LookupSet::new(StorageKey::NftId),
        }
    }

    pub fn claim_all_lockups(&mut self) {
        let mut amount = 0;
        let mut collection = self.lockups.to_vec();

        collection.iter().for_each(|lock| {
            if lock.is_expired() {
                amount += lock.amount;
                self.lockups.remove(lock);
            }
        });
        self.free += amount;
    }

    //TODO remember Use this method
    pub fn claim_lockup(&mut self, expire_on_ts: u64) {
        let mut amount = 0;
        let mut collection = self.lockups.to_vec();

        collection.iter().for_each(|lock| {
            if lock.expire_on == expire_on_ts && lock.is_expired() {
                amount += lock.amount;
                self.lockups.remove(lock);
            }
        });
        self.free += amount;
    }

    pub fn get_lockups(&self, from_index: Option<usize>, limit: Option<usize>) -> Vec<LockupInfo> {
        self.lockups
            .iter()
            .skip(from_index.unwrap_or(0))
            .take(limit.unwrap_or_else(|| self.lockups.len() as usize))
            .map(|lockup| lockup.into())
            .collect::<Vec<LockupInfo>>()
    }

    pub fn get_balance(&self) -> U128 {
        U128(self.free)
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn check_lockups() {
        let mut account = Account::new(5); // Current balance
        account.lockups.insert(&Lockup::new(55, None)); // Just locked (will unlock in 3 days (default lifetime))
        account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 0,
        }); // Lock from 1970

        account.claim_all_lockups(); // Balance of lock from 1970 will be transferred to main balance

        println!("{:#?}", account.lockups.to_vec());

        assert_eq!(account.free, 10);
    }

    #[test]
    pub fn check_lockup() {
        let mut account = Account::new(5); // Current balance
        account.lockups.insert(&Lockup::new(55, None)); // Just locked (will unlock in 3 days (default lifetime))
        account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 0,
        }); // Lock from 1970
        account.lockups.insert(&Lockup {
            amount: 8,
            expire_on: 16457898,
        }); // Lock from 1970

        account.claim_lockup(16457898); // Balance of lock from 1970 will be transferred to main balance

        println!("{:#?}", account.lockups.to_vec());

        assert_eq!(account.free, 13);
    }
}

#[derive(BorshSerialize, Debug)]
pub struct AccountInfo {
    pub free: U128,
    pub lockups: Vec<LockupInfo>,
    //pub nfts: LookupSet<NftId>,
}

impl From<Account> for AccountInfo {
    fn from(account: Account) -> Self {
        AccountInfo {
            free: account.get_balance(),
            lockups: account.get_lockups(None, None),
        }
    }
}

impl Default for AccountInfo {
    fn default() -> Self {
        Self {
            free: U128(0),
            lockups: vec![],
        }
    }
}
