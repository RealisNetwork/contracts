use crate::types::NftId;
use crate::*;
use near_sdk::json_types::U128;
use near_sdk::AccountId;
use near_sdk::{near_bindgen, require};

#[near_bindgen]
impl Contract {
    pub fn backend_transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        require!(env::signer_account_id() == self.backend_id, "Not allowed");
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    pub fn backend_burn(&mut self, nft_id: NftId) {
        todo!()
    }

    pub fn backend_transfer_nft(&mut self, recipient_id: AccountId, nft_id: NftId) {
        todo!()
    }

    pub fn backend_sell_nft(&mut self, nft_id: NftId, price: U128) {
        todo!()
    }

    pub fn backend_buy_nft(&mut self, nft_id: NftId) -> U128 {
        todo!()
    }

    pub fn backend_change_price(&mut self, nft_id: NftId, price: U128) {
        todo!()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}
