//! All the logic described here applies to the NFT marketplace.
use crate::{Account, Contract, NftId, StorageKey};
use near_sdk::{collections::UnorderedMap, env::panic_str, require, AccountId, Balance};

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
    /// Return all available for sale NFTs.
    pub fn get_marketplace_nfts(&self) -> &UnorderedMap<NftId, Balance> {
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
    pub fn buy_nft(&mut self, nft_id: &NftId) -> Balance {
        self.nft_map
            .remove(nft_id)
            .unwrap_or_else(|| panic_str("Nft not in marketplace."))
    }

    /// Change price NFT in marketplace.
    pub fn change_price_nft(&mut self, nft_id: &NftId, new_price: Balance) {
        require!(self.nft_map.get(nft_id).is_some(), "NFT not in marketplace");
        self.nft_map.insert(nft_id, &new_price);
    }
}

impl Contract {
    pub fn internal_sell_nft(&mut self, nft_id: NftId, price: Balance, account_id: AccountId) {
        self.nfts.sell_nft(&nft_id, &price, account_id)
    }

    pub fn internal_buy_nft(&mut self, nft_id: NftId, price: Balance, account_id: AccountId) {
        let mut buyer_account = Account::from(
            self.accounts
                .get(&account_id)
                .unwrap_or_else(|| panic_str("Account not found")),
        );
        require!(buyer_account.free >= price, "Not enough money");

        let nft = self.nfts.get_nft(nft_id);

        let price = self.nfts.buy_nft(&nft_id, &account_id);

        let mut nft_owner_account: Account = self
            .accounts
            .get(&nft.owner_id)
            .unwrap_or_else(|| panic_str("Account not found"))
            .into();

        nft_owner_account.free += price;
        buyer_account.free -= price;

        self.accounts
            .insert(&nft.owner_id, &nft_owner_account.into());
        self.accounts.insert(&account_id, &buyer_account.into());
    }

    pub fn internal_change_price_nft(
        &mut self,
        nft_id: NftId,
        price: Balance,
        account_id: AccountId,
    ) {
        let nft = self.nfts.get_nft(nft_id);
        require!(account_id == nft.owner_id, "Only for NFT owner.");
        self.nfts.change_price_nft(&nft_id, price);
    }
}
