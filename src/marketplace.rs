use near_sdk::{AccountId, Balance,  near_bindgen, require};

use crate::*;



#[near_bindgen]
impl Contract {
    pub fn start_auction(&mut self, nft_id: NftId, price: Balance, deadline: near_sdk::Timestamp) {
        self.nfts.sell_nft(nft_id, price, deadline);
    }
    /// set
    pub fn make_bit(&mut self, account_id: AccountId, nft_id: NftId, new_price: Balance) {
        let acc = self.accounts.get(&account_id);
        require!(acc.is_some(),"User not found");

        let VAccount::V1(mut account) = acc.unwrap();
        require!(account.free >= new_price,"Not enough tokens");

        let bit = self.nfts.change_price_nft(nft_id, new_price, Some(account_id.clone()));

        account.free -= new_price;
        account.lockups.insert(&Lockup::new(new_price, Some(bit.deadline)));
        self.accounts.insert(&account_id, &VAccount::V1(account));

        if bit.account_id.is_some() {
            if let Some(VAccount::V1(mut account)) = self.accounts.get(&bit.account_id.unwrap()) {
            account.free += bit.price;
            account.lockups.remove(&Lockup::new(bit.price, Some(bit.deadline)));
            self.accounts.insert(&account_id, &VAccount::V1(account));
        }
    }
    }
}