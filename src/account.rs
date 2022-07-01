use crate::{
    events::{EventLog, EventLogVariant, IncreaseBalance, LockupClaimed},
    lockup::{Lockup, LockupInfo},
    Deserialize, NftId, Serialize, StorageKey,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::UnorderedSet,
    env,
    json_types::U128,
    require, AccountId, Balance,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum VAccount {
    V1(Account),
}

impl From<VAccount> for Account {
    fn from(vaccount: VAccount) -> Self {
        match vaccount {
            VAccount::V1(account) => account,
        }
    }
}

#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct Account {
    free: Balance,
    pub x_staked: Balance,
    pub lockups: UnorderedSet<Lockup>,
    pub nfts: UnorderedSet<NftId>,
}

impl Account {
    pub fn new(account_id: AccountId, balance: Balance) -> Self {
        let hash = env::sha256(account_id.as_bytes());
        Self {
            free: balance,
            x_staked: 0,
            lockups: UnorderedSet::new(StorageKey::AccountLockup { hash: hash.clone() }),
            nfts: UnorderedSet::new(StorageKey::AccountNftId { hash }),
        }
    }

    pub fn get_balance(&self) -> Balance {
        self.free
    }

    pub fn increase_balance(&mut self, amount: Balance) -> &mut Account {
        self.free += amount;
        EventLog::from(EventLogVariant::IncreaseBalance(IncreaseBalance {
            amount: &U128(amount),
        }))
        .emit();
        self
    }

    pub fn decrease_balance(&mut self, amount: Balance) -> &mut Account {
        require!(self.free >= amount, "Not enough balance");
        self.free -= amount;
        // TODO: emit event
        self
    }

    pub fn claim_all_lockups(&mut self, account_id: AccountId) -> u128 {
        let collection = self.lockups.to_vec();
        let mut events: Vec<LockupClaimed> = Vec::new();

        let fold = collection
            .iter()
            .filter(|lock| lock.is_expired())
            .map(|lockup| {
                self.lockups.remove(lockup);
                events.push(LockupClaimed {
                    amount: U128(lockup.get_amount().unwrap_or_default()),
                    account_id: &account_id,
                });

                lockup
            })
            .fold(0, |acc, lockup| {
                acc + lockup.get_amount().unwrap_or_default()
            });
        EventLog::from(EventLogVariant::LockupClaimed(events)).emit();

        self.increase_balance(fold);

        fold
    }

    pub fn claim_lockup(&mut self, amount: u128, account_id: AccountId) -> u128 {
        let lockup = self
            .lockups
            .iter()
            .find(|lockup| lockup.get_amount().unwrap_or_default() == amount && lockup.is_expired())
            .unwrap_or_else(|| env::panic_str("No such lockup"));
        if lockup.get_amount().is_none() {
            return 0;
        }
        self.increase_balance(lockup.get_amount().unwrap());
        self.lockups.remove(&lockup);

        EventLog::from(EventLogVariant::LockupClaimed(vec![LockupClaimed {
            amount: U128(lockup.get_amount().unwrap_or_default()),
            account_id: &account_id,
        }]))
        .emit();

        self.free
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
            .fold(0, |acc, lockup| {
                acc + lockup.get_amount().unwrap_or_default()
            })
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

#[derive(Serialize, Debug, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub struct AccountInfo {
    pub free: U128,
    pub x_staked: U128,
    pub lockups: Vec<LockupInfo>,
    pub nfts: Vec<NftId>,
    pub lockups_free: U128,
}

impl From<Account> for AccountInfo {
    fn from(account: Account) -> Self {
        AccountInfo {
            free: account.get_balance().into(),
            x_staked: U128(account.x_staked),
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
        let (_contract, mut context) = init_test_env(None, None, None);

        let account_id = accounts(0);
        let mut account = Account::new(account_id.clone(), 5);
        // Just locked (will unlock in 3 days (default lifetime))
        account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup::new(55, None)));
        account.lockups.insert(&Lockup::GooglePlayBuy(SimpleLockup {
            amount: 5,
            expire_on: 1,
        })); // Lock from 1970

        // Balance of lock from 1970 will be transferred to main balance
        testing_env!(context.block_timestamp(999).build());

        account.claim_all_lockups(account_id);

        assert_eq!(account.free, 10);
    }

    #[test]
    pub fn check_lockup() {
        let (_contract, mut context) = init_test_env(None, None, None);

        let account_id = accounts(0);
        let mut account = Account::new(account_id.clone(), 5);
        // Just locked (will unlock in 3 days (default lifetime))
        account
            .lockups
            .insert(&Lockup::GooglePlayBuy(SimpleLockup::new(55, None)));
        account.lockups.insert(&Lockup::GooglePlayBuy(SimpleLockup {
            amount: 5,
            expire_on: 0,
        })); // Lock from 1970
        account.lockups.insert(&Lockup::GooglePlayBuy(SimpleLockup {
            amount: 8,
            expire_on: 16457898,
        })); // Lock from 1970

        testing_env!(context.block_timestamp(16457899).build());

        // Balance of lock from 1970 will be transferred to main balance
        account.claim_lockup(8, account_id);

        assert_eq!(account.free, 13);
    }
}
