use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::AccountId;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Nft {
    // TODO add fields
    pub owner_id: AccountId,
    pub metadata: String,
}
