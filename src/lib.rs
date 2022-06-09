mod account;
mod backend_api;
mod nft;
mod owner;
mod public_api;
mod tokens;
mod types;
mod utils;
mod lockup;

use crate::account::{Account, VAccount};
use crate::nft::Nft;
use crate::types::NftId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, require};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};
use near_sdk::log;
use crate::lockup::LockupInfo;

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
    pub nfts: LookupMap<NftId, Nft>,
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub beneficiary_id: AccountId,
    pub state: State,
}

#[derive(BorshStorageKey, BorshSerialize, BorshDeserialize)]
pub(crate) enum StorageKey {
    Accounts,
    Nfts,
    NftId,
    RegisteredAccounts,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        total_supply: U128,
        fee: u8,
        beneficiary_id: Option<AccountId>,
        backend_id: Option<AccountId>,
    ) -> Self {
        let owner_id = env::signer_account_id();

        let mut accounts = LookupMap::new(StorageKey::Accounts);
        accounts.insert(&owner_id, &Account::new(total_supply.0).into());

        Self {
            constant_fee: 0, // TODO: get from args
            percent_fee: fee,
            nfts: LookupMap::new(StorageKey::Nfts),
            owner_id: owner_id.clone(),
            backend_id: backend_id.unwrap_or(owner_id.clone()),
            beneficiary_id: beneficiary_id.unwrap_or(owner_id),
            state: State::Running,
            accounts,
        }
    }

    pub fn loockups_info(&self, account_id: AccountId, from_index: Option<usize>, limit: Option<usize>) -> Vec<LockupInfo> {

        match self.accounts.get(&account_id) {

            Some(user)=>{
                let user_account: Account = user.into();
                user_account
                    .lockups
                    .iter()
                    .skip(from_index.unwrap_or(0))
                    .take(limit.unwrap_or_else(|| user_account.lockups.len() as usize))
                    .map(|lockup| lockup.into())
                    .collect::<Vec<LockupInfo>>()

            }

            None => {
                vec![]
            }
        }
    }
}
#[cfg(test)]
mod tests {
    use super::tokens::tests::get_contract;
    use super::*;
    use near_sdk::collections::{LookupMap, LookupSet};
    use near_sdk::test_utils::accounts;
    use std::str::FromStr;
    use near_sdk::json_types::U64;


    #[test]
    fn info_log_test(){ // Indexes are default
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        account.lockups.insert(&lockup::Lockup {
            amount: 250,
            expire_on: 60
        });

        account.lockups.insert(&lockup::Lockup::new(25, None));
        account.lockups.insert(&lockup::Lockup::new(35, Some(20)));

        contract
            .accounts
            .insert(&account_id, &account.into());

        println!("{:#?}",contract.loockups_info(account_id, None, None));
    }

    #[test]
    fn info_no_locks(){ // There are no locks
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        contract
            .accounts
            .insert(&account_id, &account.into());

        println!("{:#?}",contract.loockups_info(account_id, None, None));
    }

}