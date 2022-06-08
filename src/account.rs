use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
//use crate::{BorshSerialize, BorshDeserialize};




pub enum VAccount {
    V1(Account),
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    // pub pub free: Balance
    pub free: u128,
    // pub lockups: Vec<Lockup>
    // pub nfts: Vec<NftId>
}

impl Default for Account {
    fn default() -> Self {
        Self {
            free: 0
        }
    }
}
