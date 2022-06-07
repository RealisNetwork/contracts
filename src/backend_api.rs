use near_sdk::AccountId;
use near_sdk::json_types::U128;
use crate::*;
use crate::types::NftId;

#[near_bindgen]
impl Contract {
    pub fn backend_transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        todo!()
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