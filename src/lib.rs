mod account;
mod account_manager;
mod backend_api;
mod lockup;
mod events;
mod metadata;
mod nft;
mod owner;
mod public_api;
mod tokens;
mod types;
mod update;
mod utils;

use crate::account::{Account, AccountInfo, VAccount};
use crate::lockup::LockupInfo;
use crate::nft::Nft;
use crate::types::NftId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{log, PublicKey};
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, require, AccountId};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Paused,
    Running,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub constant_fee: u128,
    pub percent_fee: u8, // Commission in percents over transferring amount. for example, 10 (like 10%)
    pub accounts: LookupMap<AccountId, VAccount>,
    pub nfts: UnorderedMap<NftId, Nft>,
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub beneficiary_id: AccountId,
    pub state: State,

    pub nft_id_counter: u128,
    pub registered_accounts: LookupMap<PublicKey, AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize, BorshDeserialize)]
pub(crate) enum StorageKey {
    Accounts,
    Nfts,
    NftId,
    RegisteredAccounts,
    Lockups
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        total_supply: U128,
        constant_fee: U128,
        percent_fee: u8,
        beneficiary_id: Option<AccountId>,
        backend_id: Option<AccountId>,
    ) -> Self {
        let owner_id = env::signer_account_id();

        let mut accounts = LookupMap::new(StorageKey::Accounts);
        accounts.insert(&owner_id, &Account::new(total_supply.0).into());

        Self {
            constant_fee: constant_fee.0,
            percent_fee,
            nfts: UnorderedMap::new(StorageKey::Nfts),
            owner_id: owner_id.clone(),
            backend_id: backend_id.unwrap_or(owner_id.clone()),
            beneficiary_id: beneficiary_id.unwrap_or(owner_id),
            state: State::Running,
            accounts,

            nft_id_counter: 0,

            registered_accounts: LookupMap::new(StorageKey::RegisteredAccounts),
        }
    }

    pub fn loockups_info(
        &self,
        account_id: AccountId,
        from_index: Option<usize>,
        limit: Option<usize>,
    ) -> Vec<LockupInfo> {
        match self.accounts.get(&account_id) {
            Some(user) => {
                let user_account: Account = user.into();
                user_account.get_lockups(from_index, limit)
            }

            None => {
                vec![]
            }
        }
    }

    pub fn get_balance_info(&self, account_id: AccountId) -> U128 {
        match self.accounts.get(&account_id) {
            Some(user) => {
                let user_account: Account = user.into();
                U128(user_account.free)
            }

            None => U128(0u128),
        }
    }

    pub fn get_account_info(&self, account_id: AccountId) -> AccountInfo {
        match self.accounts.get(&account_id) {
            Some(user) => {
                let user_account: Account = user.into();
                user_account.into()
            }

            None => Account::default().into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::tokens::tests::get_contract;
    use super::*;
    use near_sdk::collections::{LookupMap, LookupSet};
    use near_sdk::json_types::U64;
    use near_sdk::test_utils::accounts;
    use std::str::FromStr;

    #[test]
    fn info_log_test() {
        // Indexes are default
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        account.lockups.insert(&lockup::Lockup {
            amount: 250,
            expire_on: 60,
        });

        account.lockups.insert(&lockup::Lockup::new(25, None));
        account.lockups.insert(&lockup::Lockup::new(35, Some(20)));

        contract.accounts.insert(&account_id, &account.into());

        println!("{:#?}", contract.loockups_info(account_id, None, None));
    }

    #[test]
    fn info_no_locks() {
        // There are no locks
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        contract.accounts.insert(&account_id, &account.into());

        println!("{:#?}", contract.loockups_info(account_id, None, None));
    }

    #[test]
    fn info_get_balance_test() {
        // Indexes are default
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        contract.accounts.insert(&account_id, &account.into());
        assert_eq!(contract.get_balance_info(account_id).0, 250);
    }

    #[test]
    fn get_account_info_test() {
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        account.lockups.insert(&lockup::Lockup {
            amount: 250,
            expire_on: 60,
        });
        account.lockups.insert(&lockup::Lockup::new(25, None));
        account.lockups.insert(&lockup::Lockup::new(35, Some(20)));

        contract.accounts.insert(&account_id, &account.into());
        println!("{:#?}", contract.get_account_info(account_id));
    }
}
