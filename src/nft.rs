use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    AccountId,
};

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Nft {
    // TODO add fields
    pub owner_id: AccountId,
    pub metadata: String,
}
