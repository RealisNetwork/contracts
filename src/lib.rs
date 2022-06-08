mod utils;
mod account;
mod public_api;
mod backend_api;
mod types;
mod owner;

use near_sdk::AccountId;
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};
use near_sdk::collections::LookupMap;
use crate::types::NftId;

pub enum State {
    Paused,
    Running,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, PanicOnDefault)]
pub struct Contract {
    pub nfts: LookupMap<NftId, Nft>,
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub beneficiary_id: AccountId,
    pub fee: u8,
    pub state: State
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new() -> Self {
        todo!()
    }
}