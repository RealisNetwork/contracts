//! All the logic described here applies to the NFT marketplace.
use crate::{Account, Contract, ContractExt, NftId, StorageKey, VAccount};
use near_sdk::{collections::UnorderedMap, env, env::panic_str, near_bindgen, require, Balance};

use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
/// Structure for working with NFT in Marketplace.
/// Contains all NFTs available for sale.
/// # Fields
/// * `nft_map` - key: uniq NFT id. value: NFT price.
#[derive(Debug, BorshSerialize, BorshDeserialize)]
pub struct Marketplace {
    nft_map: UnorderedMap<NftId, Balance>,
}

impl Default for Marketplace {
    fn default() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsMarketplace),
        }
    }
}

impl Marketplace {
    pub fn new() -> Self {
        Self {
            nft_map: UnorderedMap::new(StorageKey::NftsMarketplace),
        }
    }

    /// Return all available for sale NFTs.
    pub fn get_map(&self) -> &UnorderedMap<NftId, Balance> {
        &self.nft_map
    }

    /// Return count of available for sale NFTs.
    pub fn marketplace_nft_count(&self) -> u64 {
        self.nft_map.len()
    }

    /// Return true if NFT for sale in marketplace.
    pub fn is_on_marketplace(&self, nft_id: &NftId) -> bool {
        self.nft_map.get(nft_id).is_some()
    }

    /// Add NFT to list for sale.
    pub fn sell_nft(&mut self, nft_id: &NftId, price: &Balance) {
        self.nft_map.insert(nft_id, price);
    }

    /// Remove NFT from list for sale.
    pub fn buy_nft(&mut self, nft_id: &NftId, price: Balance) -> Balance {
        let balance = self
            .nft_map
            .get(nft_id)
            .unwrap_or_else(|| panic_str("Nft not in marketplace."));
        require!(balance == price, "Wrong price.");

        self.nft_map.remove(nft_id).unwrap()
    }

    /// Change price NFT in marketplace.
    pub fn change_price_nft(&mut self, nft_id: &NftId, new_price: Balance) {
        require!(self.nft_map.get(nft_id).is_some(), "NFT not in marketplace");
        self.nft_map.insert(nft_id, &new_price);
    }
}

#[near_bindgen]
impl Contract {
    // TODO: this is need to be here?
    pub fn internal_sell_nft(&mut self, nft_id: NftId, price: Balance) {
        self.nfts
            .sell_nft(&nft_id, &price, env::signer_account_id())
    }

    pub fn internal_buy_nft(&mut self, nft_id: NftId, price: Balance) {
        let mut buyer_account = Account::from(
            self.accounts
                .get(&env::signer_account_id())
                .unwrap_or_else(|| panic_str("Account not found")),
        );
        require!(buyer_account.free >= price, "Not enough money");

        let nft = self.nfts.get_nft(nft_id);

        let price = self.nfts.buy_nft(&nft_id, price, env::signer_account_id());

        let mut nft_owner_account = Account::from(
            self.accounts
                .get(&nft.owner_id)
                .unwrap_or_else(|| panic_str("Account not found")),
        );

        nft_owner_account.free += price;
        buyer_account.free -= price;

        self.accounts
            .insert(&nft.owner_id, &VAccount::V1(nft_owner_account));
        self.accounts
            .insert(&env::signer_account_id(), &VAccount::V1(buyer_account));
    }

    pub fn internal_change_price_nft(&mut self, nft_id: NftId, price: Balance) {
        let nft = self.nfts.get_nft(nft_id);
        require!(
            env::signer_account_id() == nft.owner_id,
            "Only for NFT owner."
        );
        self.nfts.change_price_nft(&nft_id, price);
    }
}
