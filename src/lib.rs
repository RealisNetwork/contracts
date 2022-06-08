#![feature(gen_future)]

use near_sdk::AccountId;
use near_sdk::borsh::{BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet};
use near_sdk::{near_bindgen};
use near_sdk::PanicOnDefault;
use near_sdk::serde::{Serialize,Deserialize};

// mod utils;
// mod account;
// mod public_api;
// mod backend_api;
// mod types;
mod owner;
#[derive(BorshDeserialize, BorshSerialize,Serialize,Deserialize)]
pub enum State {
    Paused,
    Running,
}

#[near_bindgen]
#[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
pub struct Contract {
    // AccountId -> Account
    // NftId -> Nft
    owner_id: AccountId,
    nft_ids: LookupSet<String>,
    users_nft: LookupMap<AccountId, LookupSet<String>>,
    // backend_id: AccountId
    // beneficiary_id: AccountId
    // fee: ???
    // state: Running|Paused
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        todo!()
    }
}