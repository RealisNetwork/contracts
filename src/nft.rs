//! Designed to interact with the NFT, NFT marketplace.
use near_sdk::{AccountId, Balance, env, require};
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::{LookupMap, LookupSet, UnorderedMap, Vector};
use near_sdk::json_types::U128;

use crate::{NftId, StorageKey};

/// State of NFT.
/// Displays the current state of an NFT.
/// # States
/// * `AVAILABLE` - NFT unlocked, could be burn or transferred.
/// * `LOCK` - NFT locked. Allow read access.
#[derive(BorshSerialize, BorshDeserialize, Debug, Eq, PartialEq, Clone)]
enum NftState {
    Available,
    Lock,
}

/// Describe NFT.
/// # Fields
/// * `owner_id` - NFT owner `AccountID`.
/// * `metadata` - NFT metadata.
/// * `state` - state of NFT.
#[derive(BorshSerialize, BorshDeserialize, Debug, Clone)]
pub struct Nft {
    // TODO add fields
    pub owner_id: AccountId,
    metadata: String,
    state: NftState,
}

impl Nft {
    pub fn new(owner_id: AccountId, metadata: String) -> Self {
        Self {
            owner_id,
            metadata,
            state: NftState::Available,
        }
    }

    pub fn get_metadata(&self) -> String {
        self.metadata.clone()
    }

    pub fn set_owner_id(self, id: AccountId) -> Self {
        Self {
            owner_id: id,
            ..self
        }
    }

    /// Check if current NFT available.
    pub fn assert_available(self) -> Self {
        require!(self.state == NftState::Available, "Nft locked up");
        self
    }

    /// Deny any operations with NFT
    pub fn lock_nft(self) -> Self {
        Self {
            state: NftState::Lock,
            ..self
        }
    }

    /// Allow any operations with NFT
    pub fn unlock_nft(self) -> Self {
        Self {
            state: NftState::Available,
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

impl Default for NftMap {
    fn default() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsMap),
            marketplace_nft_map: UnorderedMap::new(StorageKey::NftsOnSale),
            nft_id_counter: 0,
        }
    }
}

impl NftMap {
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
        let nft = self.get_if_available(nft_id).set_owner_id(new_owner);
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
    pub fn generate_nft_id(&mut self) -> NftId {
        if u128::MAX == self.nft_id_counter {
            self.nft_id_counter = 0;
        }

        while self.nft_map.get(&self.nft_id_counter).is_some() {
            self.nft_id_counter += 1;
        }

        self.nft_id_counter
    }
}

#[cfg(test)]
mod tests {
    use near_contract_standards::non_fungible_token::enumeration::NonFungibleTokenEnumeration;
    use near_sdk::{AccountId, Gas, RuntimeFeesConfig, testing_env, VMConfig, VMContext};
    use near_sdk::collections::{LookupMap, UnorderedMap, UnorderedSet};
    use near_sdk::json_types::U128;
    use near_sdk::test_utils::VMContextBuilder;

    use crate::{Contract, Nft, NftMap, State};

    pub fn get_contract() -> Contract {
        let mut contract = Contract {
            constant_fee: 0,
            percent_fee: 0,
            accounts: LookupMap::new(b"m"),
            nfts: NftMap::new(),
            owner_id: AccountId::new_unchecked("id".to_string()),
            backend_id: AccountId::new_unchecked("id".to_string()),
            beneficiary_id: AccountId::new_unchecked("id".to_string()),
            state: State::Paused,
            registered_accounts: LookupMap::new(b"a"),
        };
        for i in 0..10 {
            let nft_id = contract.nfts
                .mint_nft(
                    AccountId::new_unchecked("id".to_string()),
                    String::from("metadata"),
                );
        }
        contract
    }


    pub fn get_context(caller_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::new_unchecked(caller_id))
            .is_view(false)
            .build()
    }

    #[test]
    fn id_test() {
        let mut contract = get_contract();
        let context = get_context("smbd".to_string());
        testing_env!(context, VMConfig::free(), RuntimeFeesConfig::free());
        let m_id = contract.nfts.mint_nft(AccountId::new_unchecked("id".to_string()), String::from("metadata"));
        assert_eq!(m_id, 10);
        contract.nfts.burn_nft(m_id);
        let f_id = contract.nfts.mint_nft(AccountId::new_unchecked("id".to_string()), String::from("metadata"));
        assert_eq!(f_id, 10);
    }
}
