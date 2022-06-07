mod utils;
mod account;
mod public_api;
mod backend_api;
mod types;
mod owner;

use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

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
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        todo!()
    }
}