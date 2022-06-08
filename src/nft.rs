use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Nft {
    // TODO add fields
    pub meta_data: String,
}