mod utils;
mod account;
mod public_api;
mod backend_api;
mod types;
mod owner;
mod account_manager;
mod nft;

use near_sdk::AccountId;
use near_sdk::borsh::{BorshSerialize, BorshDeserialize};
use near_sdk::collections::LookupMap;
use crate::types::NftId;

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
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
    pub fn new(total_supply: U128, fee: u8, beneficiary_id: Option<AccountId>, backend_id: Option<AccountId>) -> Self {
        let owner_id = env::signer_account_id();

        let mut accounts = LookupMap::new(StorageKey::Accounts);
        accounts.insert(&owner_id, &Account::new(total_supply.0).into());

        Self {
            nfts: LookupMap::new(StorageKey::Nfts),
            owner_id: owner_id.clone(),
            backend_id: backend_id.unwrap_or(owner_id.clone()),
            beneficiary_id: beneficiary_id.unwrap_or(owner_id),
            fee,
            state: State::Running,
            accounts,
            registered_accounts: LookupMap::new(StorageKey::RegisteredAccounts)
        }
    }
}