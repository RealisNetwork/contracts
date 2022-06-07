use near_sdk::borsh::{BorshSerialize, BorshDeserialize};

pub enum VAccount {
    V1(Account),
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    // pub free: Balance
    // pub lockups: Vec<Lockup>
    // pub nfts: Vec<NftId>
}

impl Default for Account {
    fn default() -> Self {
        todo!()
    }
}
