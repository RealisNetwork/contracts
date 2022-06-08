mod utils;
mod account;
mod public_api;
mod backend_api;
mod types;
mod owner;
mod transfer_tokens;
use near_sdk::{AccountId, serde};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::collections::LookupMap;
use crate::account::Account;
use near_sdk::{near_bindgen, PanicOnDefault};
use near_sdk::serde::{Serialize, Serializer};

#[derive(BorshSerialize, BorshDeserialize)]
pub enum State {
    Paused,
    Running,
}

impl Serialize for State {
    fn serialize<S>(&self, serializer: S) -> Result<<S as near_sdk::serde::Serializer>::Ok, <S as near_sdk::serde::Serializer>::Error> where S: Serializer {
        todo!()
    }
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    // AccountId -> Account
    // NftId -> Nft
    // owner_id: AccountId
    // backend_id: AccountId
    beneficiary_id: AccountId,
    // fee: ???
    // state: Running|Paused
    constant_fee: u128,
    percent_fee: u8, // Commission in percents over transferring amount. for example, 10 (like 10%)
    pub accounts: LookupMap<AccountId, Account>,
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        todo!()
    }
}