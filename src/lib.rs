mod account;
mod backend_api;
mod nft;
mod owner;
mod public_api;
mod tokens;
mod types;
mod utils;
mod Lock;

use crate::account::{Account, VAccount};
use crate::nft::Nft;
use crate::types::NftId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId, require};
use near_sdk::{near_bindgen, BorshStorageKey, PanicOnDefault};
use near_sdk::log;

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

    pub fn view_locks_info(& self, from_index: Option<usize>, limit: Option<usize>, account_id: AccountId) -> String{

        let mut messege_log = "".to_string();

        match self.accounts.get(&account_id) {

            Some(user)=>{
                let user_account: Account = user.into();
                let header = format!("------------locks for <{}>------------", account_id);

                let mut locks_msg = "".to_string();
                let locks = user_account.lockups.to_vec();

                let from_index = from_index.unwrap_or(0);
                let end_index = from_index + limit.unwrap_or(locks.len());

                require!(from_index <= locks.len() && end_index <= locks.len(), "Index error");

                for lock in &locks[from_index..end_index] {
                    // TODO: check if there are not something kinda null pointer
                    locks_msg = format!("{}\n{:#?}",locks_msg ,lock);
                }
                messege_log = format!("{}\n{}", header,locks_msg);
            }

            None => {
                env::panic_str("User not found");
            }
        };

        messege_log
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

        account.lockups.insert(&Lock::Lock{
            amount: 250,
            expire_on: 60
        });

        account.lockups.insert(&Lock::Lock::new(25,None));
        account.lockups.insert(&Lock::Lock::new(35,Some(20)));

        contract
            .accounts
            .insert(&account_id, &account.into());

        println!("{}",contract.view_locks_info(None,None, account_id));
    }

    #[test]
    fn info_no_locks(){ // There are no locks
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        contract
            .accounts
            .insert(&account_id, &account.into());

        println!("{}",contract.view_locks_info(None,None, account_id));
    }

    #[test]
    #[should_panic = "User not found"]
    fn info_no_user(){ // There are no locks
        let mut contract = get_contract();

        let account_id = AccountId::from_str("user.testnet").unwrap();

        println!("{}",contract.view_locks_info(None,None, account_id));
    }

    #[test]
    #[should_panic = "Index error"]
    fn info_index_error_test(){ // Indexes are default
        let mut contract = get_contract();
        let mut account: Account = Account::new(250);
        let account_id = AccountId::from_str("user.testnet").unwrap();

        account.lockups.insert(&Lock::Lock{
            amount: 250,
            expire_on: 60
        });

        account.lockups.insert(&Lock::Lock::new(25,None));
        account.lockups.insert(&Lock::Lock::new(35,Some(20)));

        contract
            .accounts
            .insert(&account_id, &account.into());

        println!("{}",contract.view_locks_info(Some(3),Some(50), account_id));
    }

}