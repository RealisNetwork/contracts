mod account;
mod backend_api;
mod nft;
mod owner;
mod public_api;
mod transfer_tokens;
mod types;
mod utils;

use crate::account::{Account, VAccount};
use crate::nft::Nft;
use crate::types::NftId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupMap;
use near_sdk::json_types::U128;
use near_sdk::serde::{Deserialize, Serialize};
use near_sdk::{env, AccountId};
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
}
