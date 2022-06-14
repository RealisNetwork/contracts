//! Designed to interact with the NFT, NFT marketplace.
use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::{UnorderedMap, Vector},
    env, require, AccountId, Balance, Timestamp,
};

use crate::{
    auction::{Auction, Bid, DealData},
    marketplace::Marketplace,
    NftId, StorageKey,
};

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

    pub fn is_owner(&self, account_id: &AccountId) -> bool {
        self.owner_id == *account_id
    }

    pub fn set_owner_id(self, id: &AccountId) -> Self {
        Self {
            owner_id: id.clone(),
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
pub struct NftManager {
    nft_map: UnorderedMap<NftId, Nft>,
    marketplace_nft_map: Marketplace,
    auction_nft_map: Auction,
    nft_id_counter: NftId,
}

impl Default for NftManager {
    fn default() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsMap),
            marketplace_nft_map: Marketplace::default(),
            auction_nft_map: Auction::default(),
            nft_id_counter: 0,
        }
    }
}

impl NftManager {
    /// Get count of all NFTs.
    pub fn nft_count(&self) -> u64 {
        self.nft_map.len()
    }

    /// Get count of all NFTs listed on the marketplace.
    pub fn marketplace_nft_count(&self) -> u64 {
        self.marketplace_nft_map.marketplace_nft_count()
    }

    /// Get count of all NFTs listed on the auction.
    pub fn auction_nft_count(&self) -> u64 {
        self.auction_nft_map.auction_lots_count()
    }

    /// Get all NFTs with ID.
    pub fn get_nft_map(&self) -> &UnorderedMap<NftId, Nft> {
        &self.nft_map
    }

    /// Get all NFTs.
    pub fn get_all_nft(&self) -> &Vector<Nft> {
        self.nft_map.values_as_vector()
    }

    /// Return map of NFTs listed on the marketplace.
    pub fn get_marketplace_nft_map(&self) -> &UnorderedMap<NftId, Balance> {
        self.marketplace_nft_map.get_marketplace_nfts()
    }

    /// Return map of NFTs listed on the auction.
    pub fn get_auction_nft_map(&self) -> &UnorderedMap<NftId, DealData> {
        self.auction_nft_map.get_auction_lots()
    }

    pub fn get_deal_data(&self, nft_id: &NftId) -> DealData {
        self.auction_nft_map.get_deal_info(nft_id)
    }

    /// Get NFT by ID if ID exist.
    pub fn get_nft(&self, nft_id: NftId) -> Nft {
        self.nft_map
            .get(&nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"))
    }

    /// Get NFT by ID if ID exist and NFT is available.
    pub fn get_if_available(&self, nft_id: &NftId) -> Nft {
        self.nft_map
            .get(nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"))
            .assert_available()
    }

    /// Get all available NFTs.
    pub fn get_available_nft_id(&self) -> Vec<NftId> {
        self.nft_map
            .keys()
            .filter(|key| {
                !self.auction_nft_map.is_in_auction(key)
                    && !self.marketplace_nft_map.is_on_marketplace(key)
            })
            .collect()
    }

    /// Make asserts of:
    ///  - NFT exist and available to sale,
    ///  - NFT owner is equal to accound_id in params.
    /// Make NFT available to buy in auction.
    /// Lock NFT for other operations till auction will be finished.
    pub fn start_auction(
        &mut self,
        nft_id: &NftId,
        price: Balance,
        deadline: Timestamp,
        account_id: AccountId,
    ) {
        let nft = self.get_if_available(nft_id);
        require!(nft.is_owner(&account_id), "Not the owner of NFT");
        self.auction_nft_map
            .start_auction(nft_id, price, deadline, account_id);
        self.nft_map.insert(nft_id, &nft.lock_nft());
    }

    /// Make asserts of:
    ///  - Auction expired,
    ///  - account id is not NFT owner id.
    ///  - Price less or eq last on.
    /// Change currant bid for new one.
    pub fn make_bid(
        &mut self,
        account_id: AccountId,
        nft_id: &NftId,
        price: Balance,
    ) -> Option<Bid> {
        self.auction_nft_map.make_bid(account_id, nft_id, price)
    }

    /// Make asserts of:
    ///  - Auction in progress,
    ///  - Account id belong to NFT owner or highest bid maker.
    /// Unlock NFT if nobody made bids.
    /// Change NFT owner and unlock for future operations.
    pub fn confirm_deal(&mut self, nft_id: &NftId, account_id: AccountId) -> DealData {
        let nft = self.get_if_available(nft_id);
        let deal_data = self.auction_nft_map.confirm_deal(nft_id, account_id);
        match &deal_data.get_bid() {
            None => self.nft_map.insert(nft_id, &nft.unlock_nft()),
            Some(bid) => {
                let nft = nft.unlock_nft().set_owner_id(bid.get_owner());
                self.nft_map.insert(nft_id, &nft)
            }
        };
        deal_data
    }

    /// Make assert of:
    ///  - Price in params eq to NFT price.
    ///  - buyer is not an owner of NTF.
    /// Remove NFT from for sale list.
    /// Change NFT owner and unlock for future operations.
    pub fn buy_nft(&mut self, nft_id: &NftId, price: Balance, new_owner: AccountId) -> Balance {
        let nft = self.get_if_available(nft_id);
        require!(nft.owner_id != new_owner, "Owner can't buy own NFT.");
        let balance = self.marketplace_nft_map.buy_nft(nft_id, price);
        self.nft_map
            .insert(nft_id, &nft.unlock_nft().set_owner_id(&new_owner));
        balance
    }

    /// Change price of NFT.
    pub fn change_price_nft(&mut self, nft_id: &NftId, new_price: Balance) {
        self.marketplace_nft_map.change_price_nft(nft_id, new_price);
    }

    /// Manage of sell NFT.
    pub fn sell_nft(&mut self, nft_id: &NftId, price: &Balance, account_id: AccountId) {
        let nft = self.get_if_available(nft_id);
        require!(nft.is_owner(&account_id), "Not the owner of NFT");
        self.marketplace_nft_map.sell_nft(nft_id, price);
        self.nft_map.insert(nft_id, &nft.lock_nft());
    }

    /// Remove NFT if NFT available.
    /// For remove need to unlock NFT if it was locked up.
    pub fn burn_nft(&mut self, nft_id: &NftId) {
        require!(
            !self.marketplace_nft_map.is_on_marketplace(nft_id)
                && !self.auction_nft_map.is_in_auction(nft_id),
            "Nft locked up"
        );
        self.nft_map
            .remove(nft_id)
            .unwrap_or_else(|| env::panic_str("Nft not exist"));
    }

    /// Mint new `NFT` with generated id.
    pub fn mint_nft(&mut self, owner_id: &AccountId, metadata: String) -> u128 {
        let new_nft_id = self.generate_nft_id();
        let nft = Nft::new(owner_id.clone(), metadata);

        self.nft_map.insert(&new_nft_id, &nft);

        new_nft_id
    }

    /// Transfer `NFT` between two users if NFT available.
    pub fn transfer_nft(&mut self, new_owner: AccountId, nft_id: &NftId) {
        let nft = self.get_if_available(nft_id).set_owner_id(&new_owner);
        self.nft_map.insert(nft_id, &nft);
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

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;

    #[test]
    fn id_test() {
       let (mut contract,context)  =
           init_test_env(
               Some(AccountId::new_unchecked("user_id".to_string())), None,None);
 contract.accounts.insert(&AccountId::new_unchecked("id".to_string()),&VAccount::V1(Account::new(0)));

        let m_id = contract.nfts.mint_nft(
            &AccountId::new_unchecked("id".to_string()),
            String::from("metadata"),
        );
        assert_eq!(m_id, 0);
        contract.nfts.burn_nft(&m_id);
        let f_id = contract.nfts.mint_nft(
            &AccountId::new_unchecked("id".to_string()),
            String::from("metadata"),
        );
        assert_eq!(f_id, 0);
    }
}
