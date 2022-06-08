use near_sdk::{AccountId, Timestamp};
use near_sdk::json_types::U128;
use crate::*;
use near_sdk::near_bindgen;

#[near_bindgen]
impl Contract {
    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) {
        todo!()
    }

    pub fn change_state(&mut self, state: State) {
        todo!()
    }

    pub fn change_beneficiary(&mut self, new_beneficiary_id: AccountId) {
        todo!()
    }

    pub fn create_lockup(&mut self, recipient_id: AccountId, amount: U128, duration: Option<Timestamp>) -> Timestamp {
        todo!()
    }

    pub fn create_account() {
        todo!()
    }
}