use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Nft {
    // TODO add fields
    pub meta_data: String,
}
