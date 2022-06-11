use near_sdk::{AccountId, Balance, near_bindgen, require};
use near_sdk::env::panic_str;


use crate::*;

#[near_bindgen]
impl Contract {
    pub fn start_auction(&mut self, nft_id: NftId, price: Balance, deadline: near_sdk::Timestamp) {
        self.nfts.sell_nft(nft_id, price, deadline);
    }

    pub fn make_bit(&mut self, account_id: AccountId, nft_id: NftId, new_price: Balance) {
        let acc = self.accounts.get(&account_id);
        require!(acc.is_some(),"User not found");

        let mut account =Account::from( acc.unwrap());
        require!(account.free >= new_price,"Not enough tokens");

        let bit = self.nfts.change_price_nft(nft_id, new_price, Some(account_id.clone()));

        account.free -= new_price;
        account.lockups.insert(&Lockup::new(new_price, Some(bit.deadline)));
        self.accounts.insert(&account_id, &VAccount::V1(account));

        if bit.account_id.is_some() {
            let mut account:Account = Account::from(self.accounts.get(&bit.account_id.unwrap())
                .unwrap_or_else(||panic_str("Account not exist")));

                account.free += bit.price;
                account.lockups.remove(&Lockup::new(bit.price, Some(bit.deadline)));
                self.accounts.insert(&account_id, &VAccount::V1(account));
            }
    }
    pub fn confirm_deal(&mut self, nft_id: NftId, account_id: AccountId) {
        let last_bit = self.nfts.get_bit(nft_id);
        let nft = self.nfts.get_nft(nft_id);

        require!(last_bit.get_deadline() < &env::block_timestamp(),"Auction in progress");
        if last_bit.account_id.is_none() {
            require!(nft.owner_id==account_id,"Only for nft owner");
            self.nfts.unlock_nft(nft_id);
            return;
        }
        let new_nft_owner = last_bit.account_id.unwrap();
        require!((&nft.owner_id==&account_id)
            ||(&new_nft_owner==&account_id),"Only for nft owner or owner of max bit");

    let mut account = Account::from(self.accounts.get(&new_nft_owner)
        .unwrap_or_else(||panic_str("Account not exist")));
        account.lockups.remove(&Lockup::new(last_bit.price, Some(last_bit.deadline)));
        self.accounts.insert(&new_nft_owner, &VAccount::V1(account));

        let mut account = Account::from(self.accounts.get(&nft.owner_id)
            .unwrap_or_else(||panic_str("Account not exist")));

        account.free += last_bit.price;
        self.nfts.unlock_nft(nft_id);

        self.nfts.transfer_nft(new_nft_owner, nft_id);
    }
}