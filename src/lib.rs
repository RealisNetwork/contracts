extern crate core;

mod account;
mod account_manager;
mod auction;
mod backend_api;
mod events;
mod lockup;
mod marketplace;
mod metadata;
mod nft;
mod owner;
mod public_api;
mod tokens;
mod types;
mod update;
mod utils;

use crate::{
    account::{Account, AccountInfo, VAccount},
    lockup::LockupInfo,
    nft::NftManager,
    types::NftId,
};
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env,
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey, PanicOnDefault, PublicKey,
};

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Paused,
    Running,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub constant_fee: u128,
    pub percent_fee: u8,
    // Commission in percents over transferring amount. for example, 10
    // (like 10%)
    pub accounts: LookupMap<AccountId, VAccount>,
    pub nfts: NftManager,
    // Owner of the contract. Example, `Realis.near` or `Volvo.near`
    pub owner_id: AccountId,
    // Allowed user from backend, with admin permission.
    pub backend_id: AccountId,
    // Fee collector.
    pub beneficiary_id: AccountId,
    // State of contract.
    pub state: State,
    // API accounts.
    pub registered_accounts: LookupMap<PublicKey, AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize, BorshDeserialize)]
pub(crate) enum StorageKey {
    Accounts,
    NftsMap,
    NftsMarketplace,
    NftsAuction,
    NftsAuctionBids,
    NftId,
    RegisteredAccounts,
    Lockups,
    AccountLockup { hash: Vec<u8> },
    AccountNftId { hash: Vec<u8> },
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
        accounts.insert(
            &owner_id,
            &Account::new(owner_id.clone(), total_supply.0).into(),
        );

        Self {
            constant_fee: constant_fee.0,
            percent_fee,
            nfts: NftManager::default(),
            owner_id: owner_id.clone(),
            backend_id: backend_id.unwrap_or_else(|| owner_id.clone()),
            beneficiary_id: beneficiary_id.unwrap_or(owner_id),
            state: State::Running,
            accounts,
            registered_accounts: LookupMap::new(StorageKey::RegisteredAccounts),
        }
    }

    pub fn lockups_info(
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

    pub fn get_account_info(&self, account_id: &AccountId) -> AccountInfo {
        match self.accounts.get(account_id) {
            Some(user) => {
                let user_account: Account = user.into();
                user_account.into()
            }
            None => Account::new(account_id.clone(),0).into(),
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;

    #[test]
    fn info_get_balance_test() {
        // Indexes are default
        let (mut contract, _context) = init_test_env(None, None, None);
        let account: Account = Account::new(accounts(0), 250 * ONE_LIS);
        let account_id = accounts(0);

        contract.accounts.insert(&account_id, &account.into());
        assert_eq!(contract.get_balance_info(account_id).0, 250 * ONE_LIS);
    }
}
