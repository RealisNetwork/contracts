mod utils;
mod account;
mod public_api;
mod backend_api;
mod types;
mod owner;
mod account_manager;
mod nft;

use near_sdk::{PanicOnDefault, BorshStorageKey, near_bindgen, AccountId, PublicKey, env};
use near_sdk::borsh::{self, BorshSerialize, BorshDeserialize};
use near_sdk::serde::{Serialize, Deserialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap};
use near_sdk::env::predecessor_account_id;
use near_sdk::json_types::U128;
use crate::account::{Account, VAccount};
use crate::nft::Nft;
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
    pub state: State,
    pub accounts: LookupMap<AccountId, VAccount>,
    pub registered_accounts: LookupMap<PublicKey, AccountId>,
}

#[derive(BorshStorageKey, BorshSerialize)]
pub(crate) enum StorageKey {
    Accounts,
    Nfts,
    RegisteredAccounts,
    NftId,
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