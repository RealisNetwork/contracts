//! All the logic described here applies to the NFT marketplace.
use crate::{Account, Contract, NftId, StorageKey};
use near_sdk::{collections::UnorderedMap, env::panic_str, require, AccountId, Balance, env};

use crate::nft::Nft;
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
    pub fn internal_get_nft_marketplace_info(&self, nft_id: NftId) -> u128 {
        self.nfts.get_marketplace_nft_map().get(&nft_id).unwrap_or_else(|| env::panic_str("Not found"))
    }

    pub fn internal_sell_nft(&mut self, nft_id: NftId, price: Balance, account_id: AccountId) {
        self.nfts.sell_nft(&nft_id, &price, account_id)
    }

    pub fn internal_buy_nft(&mut self, nft_id: NftId, account_id: AccountId)->Balance {
        let mut buyer_account: Account = self
            .accounts
            .get(&account_id)
            .unwrap_or_else(|| panic_str("Account not found"))
            .into();



        let nft: Nft = self.nfts.get_nft(&nft_id).into();

        require!(buyer_account.free >= self.nfts.get_marketplace_nft_map().get(&nft_id).unwrap_or_else(||panic_str("Nft not found")), "Not enough money");

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
        price
    }

    pub fn internal_change_price_nft(
        &mut self,
        nft_id: NftId,
        price: Balance,
        account_id: AccountId,
    ) {
        let nft: Nft = self.nfts.get_nft(&nft_id).into();
        require!(account_id == nft.owner_id, "Only for NFT owner.");
        self.nfts.change_price_nft(&nft_id, price,env::signer_account_id());
    }
}

#[cfg(test)]
mod tests {
    use crate::{nft::Nft, utils::tests_utils::*};

    fn get_contract() -> (Contract, VMContextBuilder) {
        let (mut cn, ct) = init_test_env(Some(accounts(0)), Some(accounts(0)), Some(accounts(0)));

        let ac: VAccount = Account::new(1000).into();
        cn.accounts.insert(&accounts(1), &ac);
        let id = cn.nfts.mint_nft(&accounts(1), "metadata".to_string());
        cn.nfts.mint_nft(&accounts(1), "metadata".to_string());
        cn.internal_sell_nft(id, 1000, accounts(1));

        let ac: VAccount = Account::new(0).into();
        cn.accounts.insert(&accounts(2), &ac);

        let ac: VAccount = Account::new(1000).into();
        cn.accounts.insert(&accounts(3), &ac);

        (cn, ct)
    }

    #[test]
    #[should_panic(expected = "Not enough money")]
    fn buy_with_out_money_test() {
        let (mut contract, context) = get_contract();
        contract.internal_buy_nft(0, 1000, accounts(2))
    }

    #[test]
    fn correct_deal_test() {
        let (mut contract, context) = get_contract();
        contract.internal_buy_nft(0, 1000, accounts(3));

        let new_own: Account = contract.accounts.get(&accounts(3)).unwrap().into();
        let prev_own: Account = contract.accounts.get(&accounts(1)).unwrap().into();

        let nft: Nft = contract.nfts.get_nft(&0).into();

        assert_eq!(new_own.free, 0);
        assert_eq!(prev_own.free, 2000);
        assert_eq!(nft.owner_id, accounts(3));
    }

    #[test]
    #[should_panic(expected = "Nft not in marketplace.")]
    fn buy_if_not_on_sale_test() {
        let (mut contract, context) = get_contract();
        contract.internal_buy_nft(1, 1000, accounts(3));
    }

    #[test]
    #[should_panic(expected = "Owner can't buy own NFT.")]
    fn buy_own_nft_test() {
        let (mut contract, context) = get_contract();
        contract.internal_buy_nft(0, 1000, accounts(1));
    }

    #[test]
    #[should_panic(expected = "Nft locked up")]
    fn sell_again_test() {
        let (mut contract, context) = get_contract();
        contract.start_auction(0, 1000, env::block_timestamp() + 100, accounts(1));
    }

    #[test]
    #[should_panic(expected = "Nft locked up")]
    fn sell_again2_test() {
        let (mut contract, context) = get_contract();
        contract.internal_sell_nft(0, 1000, accounts(1));
    }

    #[test]
    #[should_panic(expected = "Not the owner of NFT")]
    fn sell_not_nft_owner_test() {
        let (mut contract, context) = get_contract();
        contract.nfts.mint_nft(&accounts(1), "metadata".to_string());
        contract.internal_sell_nft(1, 1000, accounts(2));
    }

    #[test]
    fn change_price_test() {
        let (mut contract, context) = get_contract();
        contract.internal_change_price_nft(0, 2000, accounts(1));
        let res = contract.nfts.get_marketplace_nft_map().get(&0).unwrap();
        assert_eq!(res, 2000);
    }

    #[test]
    #[should_panic(expected = "Only for NFT owner.")]
    fn change_price_not_nft_owner_test() {
        let (mut contract, context) = get_contract();
        contract.internal_change_price_nft(0, 2000, accounts(2));
    }

    #[test]
    fn sell_nft_test() {
        let (mut contract, context) = get_contract();
        let nft_id = contract.nfts.mint_nft(&accounts(3), "".to_owned());
        contract.internal_sell_nft(nft_id, 1000, accounts(3));
        let nft_map = contract.nfts.get_marketplace_nft_map();
        assert!(nft_map.get(&nft_id).is_some())
    }

    #[test]
    #[should_panic(expected = "Not the owner of NFT")]
    fn sell_someone_nft_test() {
        let (mut contract, context) = get_contract();
        let nft_id = contract.nfts.mint_nft(&accounts(3), "".to_owned());
        contract.internal_sell_nft(nft_id, 1000, accounts(4));
    }

    #[test]
    #[should_panic(expected = "Nft locked up")]
    fn sell_locked_nft_test() {
        let (mut contract, context) = get_contract();
        contract.internal_sell_nft(0, 1000, accounts(3));
    }
}
