use crate::{
    events::{EventLog, EventLogVariant, LockupLog},
    lockup::Lockup,
    LockupInfo, NftId, Serialize, StorageKey,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    json_types::U128,
    Balance,
};

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
    pub lockups: UnorderedSet<Lockup>,
    pub nfts: UnorderedSet<NftId>,
}

impl Account {
    pub fn new(balance: Balance) -> Self {
        Self {
            free: balance,
            lockups: UnorderedSet::new(StorageKey::Lockups),
            nfts: UnorderedSet::new(StorageKey::NftId),
        }
    }

    pub fn claim_all_lockups(&mut self) -> u128 {
        let collection = self.lockups.to_vec();

        let fold = collection
            .iter()
            .filter(|lock| lock.is_expired())
            .map(|lock| {
                self.lockups.remove(lock);
                lock
            })
            .fold(0, |acc, lock| acc + lock.amount);
        self.free += fold;

        EventLog::from(EventLogVariant::LockupLog(LockupLog { amount: U128(fold) })).emit();

        fold
    }

    pub fn claim_lockup(&mut self, expire_on_ts: u64) -> u128 {
        let collection = self.lockups.to_vec();

        let fold = collection
            .iter()
            .filter(|lock| lock.expire_on == expire_on_ts && lock.is_expired())
            .map(|lock| {
                self.lockups.remove(lock);
                lock
            })
            .fold(0, |acc, lock| acc + lock.amount);

        self.free += fold;

        EventLog::from(EventLogVariant::LockupLog(LockupLog { amount: U128(fold) })).emit();

        fold
    }

    pub fn get_lockups(&self, from_index: Option<usize>, limit: Option<usize>) -> Vec<LockupInfo> {
        self.lockups
            .iter()
            .skip(from_index.unwrap_or(0))
            .take(limit.unwrap_or_else(|| self.lockups.len() as usize))
            .map(|lockup| lockup.into())
            .collect::<Vec<LockupInfo>>()
    }

    pub fn get_lockups_free(&self) -> u128 {
        self.lockups
            .iter()
            .filter(|lock| lock.is_expired())
            .fold(0, |acc, lock| acc + lock.amount)
    }

    pub fn get_nfts(&self) -> Vec<NftId> {
        self.nfts.iter().collect()
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

#[derive(Serialize, Debug)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
    pub free: U128,
    pub lockups: Vec<LockupInfo>,
    pub nfts: Vec<NftId>,
    pub lockups_free: U128,
}

impl From<Account> for AccountInfo {
    fn from(account: Account) -> Self {
        AccountInfo {
            free: U128(account.free),
            lockups: account.get_lockups(None, None),
            nfts: account.get_nfts(),
            lockups_free: U128(account.get_lockups_free()),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests_utils::*;

    #[test]
    pub fn check_lockups() {
        let (contract, mut context) = init_test_env(None, None, None);

        let mut account = Account::new(5);
        // Just locked (will unlock in 3 days (default lifetime))
        account.lockups.insert(&Lockup::new(55, None));
        account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 1,
        }); // Lock from 1970

        // Balance of lock from 1970 will be transferred to main balance
        testing_env!(context
            .block_timestamp(999)
            .predecessor_account_id(accounts(0))
            .build());

        account.claim_all_lockups();

        assert_eq!(account.free, 10);
    }

    #[test]
    pub fn check_lockup() {
        let (contract, mut context) = init_test_env(None, None, None);

        let mut account = Account::new(5);
        // Just locked (will unlock in 3 days (default lifetime))
        account.lockups.insert(&Lockup::new(55, None));
        account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 0,
        }); // Lock from 1970
        account.lockups.insert(&Lockup {
            amount: 8,
            expire_on: 16457898,
        }); // Lock from 1970

        testing_env!(context
            .block_timestamp(16457899)
            .predecessor_account_id(accounts(0))
            .build());

        // Balance of lock from 1970 will be transferred to main balance
        account.claim_lockup(16457898);

        assert_eq!(account.free, 13);
    }
}
