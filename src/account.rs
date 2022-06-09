use crate::{NftId, Serialize, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupSet, UnorderedSet};
use near_sdk::Balance;
use crate::lockup::Lockup;

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V1(Account),
}

impl Default for VAccount {
    fn default() -> Self {
        VAccount::V1( Account {
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


    pub fn check_lockups(&mut self) {
        let mut amount = 0;
        let mut collection = self.lockups.to_vec();

        collection.iter().for_each(|lock|  {
            if lock.is_expired() {
                amount += lock.amount;
                self.lockups.remove(lock);
            }
        });
        self.free += amount;
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
    pub fn check_lock() {
        let mut account = Account::new(5); // Current balance
        account.lockups.insert(&Lockup::new(55, None)); // Just locked (will unlock in 3 days (default lifetime))
        account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 0,
        }); // Lock from 1970

        account.check_lockups(); // Balance of lock from 1970 will be transferred to main balance

        println!("{:#?}", account.lockups.to_vec());

        assert_eq!(account.free, 10);
    }

}