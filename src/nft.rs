//! Designed to interact with the NFT, NFT marketplace.
use crate::{NftId, StorageKey};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap, Vector};
use near_sdk::json_types::U128;
use near_sdk::{env, require, AccountId, Balance};

/// State of NFT.
/// Displays the current state of an NFT.
/// # States
/// `AVAILABLE` - NFT unlocked, could be burn or transferred.
/// `LOCK` - NFT locked. Allow read access.
#[derive(BorshSerialize, BorshDeserialize, Debug, Eq, PartialEq, Clone)]
enum NftState {
    AVAILABLE,
    LOCK,
}

/// Describe NFT.
/// # Fields
/// * `owner_id` - NFT owner `AccountID`.
/// * `metadata` - NFT metadata.
/// * `state` - state of NFT.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Nft {
    // TODO add fields
    owner_id: AccountId,
    metadata: String,
    state: NftState,
}

impl Nft {
    pub fn new(owner_id: AccountId, metadata: String) -> Self {
        Self {
            owner_id,
            metadata,
            state: NftState::AVAILABLE,
        }
    }

    pub fn get_metadata(&self) -> String {
        self.metadata.clone()
    }

    pub fn get_owner_id(&self) -> AccountId {
        self.owner_id.clone()
    }

    pub fn set_owner_id(self, id: AccountId) -> Self {
        Self {
            owner_id: id,
            ..self
        }
    }

    /// Check if current NFT available.
    pub fn assert_available(self) -> Self {
        require!(self.state == NftState::AVAILABLE, "Nft locked up");
        self
    }

    /// Deny any operations with NFT
    pub fn lock_nft(self) -> Self {
        Self {
            state: NftState::LOCK,
            ..self
        }
    }

    /// Allow any operations with NFT
    pub fn unlock_nft(self) -> Self {
        Self {
            state: NftState::AVAILABLE,
            ..self
        }
    }
}

/// `NftMap` Structure for internal NFT management.
///
/// # Fields
/// * `nft_map` - All NFTs of the contract.
/// * `marketplace_nft_map` - Map of all NFTs listed on the marketplace.
/// * `nft_id_counter` - counter for generating NFT id.
#[derive(BorshSerialize, BorshDeserialize, Debug)]
pub struct NftMap {
    nft_map: UnorderedMap<NftId, Nft>,
    marketplace_nft_map: UnorderedMap<NftId, Balance>,
    nft_id_counter: NftId,
}

impl NftMap {
    pub fn new() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsMap),
            marketplace_nft_map: UnorderedMap::new(StorageKey::NftsOnSale),
            nft_id_counter: 0,
        }
    }
    /// Get count of all NFTs.
    pub fn nft_count(&self) -> u64 {
        self.nft_map.len()
    }
    /// Get count of all NFTs listed on the marketplace.
    pub fn marketplace_nft_count(&self) -> u64 {
        self.marketplace_nft_map.len()
    }
    /// Get NFT by ID if ID exist.
    pub fn get_nft(&self, nft_id: NftId) -> Nft {
        self.nft_map
            .get(&nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"))
    }
    /// Get NFT by ID if ID exist and NFT is available.
    pub fn get_if_available(&self, nft_id: NftId) -> Nft {
        self.nft_map
            .get(&nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"))
            .assert_available()
    }

    /// Get all NFTs with ID.
    pub fn get_nft_map(&self) -> &UnorderedMap<NftId, Nft> {
        &self.nft_map
    }

    /// Get all NFTs.
    pub fn get_all_nft(&self) -> &Vector<Nft> {
        &self.nft_map.values_as_vector()
    }

    /// Get all available NFTs.
    pub fn get_available_nft_id(&self) -> Vec<NftId> {
        self.nft_map
            .keys()
            .filter(|key| self.marketplace_nft_map.get(key).is_none())
            .collect()
    }
    /// Return map of NFTs listed on the marketplace.
    pub fn get_marketplace_nft_map(&self) -> &UnorderedMap<NftId, Balance> {
        &self.marketplace_nft_map
    }

    /// Remove NFT if NFT available.
    /// For remove need to unlock NFT if it was locked up.
    pub fn burn_nft(&mut self, nft_id: NftId) {
        require!(
            self.marketplace_nft_map.get(&nft_id).is_none(),
            "Nft locked up"
        );
        self.nft_map
            .remove(&nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"));
    }

    /// Mint new `NFT` with generated id.
    pub fn mint_nft(&mut self, owner_id: AccountId, metadata: String) -> u128 {
        let new_nft_id = self.generate_nft_id();
        let nft = Nft::new(owner_id.clone(), metadata.clone());

        self.nft_map.insert(&new_nft_id, &nft);

        new_nft_id
    }
    /// Transfer `NFT` between two users if NFT available.
    pub fn transfer_nft(&mut self, new_owner: AccountId, nft_id: NftId) {
        require!(
            self.marketplace_nft_map.get(&nft_id).is_none(),
            "Nft locked up"
        );
        let mut nft = self
            .nft_map
            .get(&nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"))
            .set_owner_id(new_owner);
        self.nft_map.insert(&nft_id, &nft);
    }
    /// List `NFT` with `price` on marketplace.
    pub fn sell_nft(&mut self, nft_id: u128, price: Balance) {
        let nft = self
            .nft_map
            .get(&nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"));

        self.marketplace_nft_map.insert(&nft_id, &price);
        self.nft_map.insert(&nft_id, &nft.lock_nft());
    }
    /// Remove `NFT` from `marketplace_nft_map` and transfer to the new owner.
    pub fn buy_nft(&mut self, nft_id: u128, new_owner_id: AccountId) {
        require!(self.nft_map.get(&nft_id).is_some(), "Nft not exist");
        self.marketplace_nft_map.remove(&nft_id);
        self.transfer_nft(new_owner_id, nft_id);
    }

    /// Change price of `NFT` that already on sale.
    pub fn change_price_nft(&mut self, nft_id: u128, new_price: Balance) {
        require!(
            self.marketplace_nft_map.get(&nft_id).is_some(),
            "Nft isn't exist or isn't on sale"
        );

        self.marketplace_nft_map.insert(&nft_id, &new_price);
    }

    /// Generate new id for new `NFT`.
    fn generate_nft_id(&mut self) -> NftId {
        if u128::MAX == self.nft_id_counter {
            self.nft_id_counter = 0;
        }

        while self.nft_map.get(&self.nft_id_counter).is_some() {
            self.nft_id_counter += 1;
        }

        self.nft_id_counter
    }
}
