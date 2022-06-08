mod utils;
mod account;
mod public_api;
mod backend_api;
mod types;
mod owner;
mod account_manager;

use near_sdk::{PanicOnDefault, near_bindgen, AccountId, PublicKey};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
use near_sdk::env::predecessor_account_id;
use near_sdk::json_types::U128;
use crate::account::{Account, VAccount};
use crate::nft::Nft;
use crate::types::NftId;

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
    // AccountId -> Account
    // NftId -> Nft
    // owner_id: AccountId
    // backend_id: AccountId
    // beneficiary_id: AccountId
    // fee: ???
    // state: Running|Paused
    pub registered_accounts: LookupMap<PublicKey, AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    Nfts,
    RegisteredAccounts,
    NftId,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        todo!()
    }
}