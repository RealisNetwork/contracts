extern crate core;

use near_sdk::collections::UnorderedSet;
pub use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env,
    json_types::U128,
    near_bindgen,
    serde::{Deserialize, Serialize},
    AccountId, BorshStorageKey, PublicKey,
};

use crate::{
    account::{Account, VAccount},
    nft::NftManager,
    staking::Staking,
    types::NftId,
    utils::ONE_LIS,
};

pub mod account;
pub mod account_manager;
pub mod auction;
pub mod backend_api;
pub mod events;
pub mod lockup;
pub mod marketplace;
pub mod metadata;
pub mod nft;
pub mod owner;
pub mod public_api;
pub mod staking;
pub mod tokens;
pub mod types;
pub mod update;
pub mod utils;
pub mod view;

#[derive(BorshSerialize, BorshDeserialize, Serialize, Deserialize, PartialEq, Eq, Clone, Debug)]
#[serde(crate = "near_sdk::serde")]
pub enum State {
    Paused,
    Running,
}

#[near_bindgen]
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Contract {
    pub constant_fee: u128,
    pub percent_fee: u8,
    // Commission in percents over transferring amount. for example, 10
    // (like 10%)
    pub accounts: LookupMap<AccountId, VAccount>,
    pub nfts: NftManager,
    // Owner of the contract. Example, `Realis.near` or `Volvo.near`
    pub owner_id: AccountId,
    // Allowed user from backend, with admin permission.
    pub backend_ids: UnorderedSet<AccountId>,
    // Fee collector.
    pub beneficiary_id: AccountId,
    // State of contract.
    pub state: State,
    // API accounts.
    pub registered_accounts: LookupMap<PublicKey, AccountId>,
    pub staking: Staking,
}

#[derive(BorshStorageKey, BorshSerialize, BorshDeserialize)]
pub(crate) enum StorageKey {
    Accounts,
    NftsMap,
    NftsMarketplace,
    NftsAuction,
    NftsAuctionBids,
    NftId,
    RegisteredAccounts,
    Lockups,
    BackendIds,
    AccountLockup { hash: Vec<u8> },
    AccountNftId { hash: Vec<u8> },
}

#[near_bindgen]
impl Contract {
    #[init]
    pub fn new(
        total_supply: Option<U128>,
        constant_fee: Option<U128>,
        percent_fee: Option<u8>,
        beneficiary_id: Option<AccountId>,
        backend_id: Option<AccountId>,
    ) -> Self {
        let owner_id = env::signer_account_id();

        let mut accounts = LookupMap::new(StorageKey::Accounts);
        accounts.insert(
            &owner_id,
            &Account::new(
                owner_id.clone(),
                total_supply.unwrap_or(U128(3_000_000_000 * ONE_LIS)).0,
            )
            .into(),
        );

        let mut backend_ids = UnorderedSet::new(StorageKey::BackendIds);
        backend_ids.insert(&backend_id.unwrap_or_else(|| owner_id.clone()));

        Self {
            constant_fee: constant_fee.unwrap_or(U128(ONE_LIS)).0,
            percent_fee: percent_fee.unwrap_or(10),
            nfts: NftManager::default(),
            owner_id: owner_id.clone(),
            backend_ids,
            beneficiary_id: beneficiary_id.unwrap_or(owner_id),
            state: State::Running,
            accounts,
            registered_accounts: LookupMap::new(StorageKey::RegisteredAccounts),
            staking: Staking::default(),
        }
    }
}

impl Default for Contract {
    fn default() -> Self {
        Self::new(None, None, None, None, None)
    }
}
