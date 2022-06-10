use crate::{types::NftId, *};
use near_sdk::{json_types::U128, AccountId};

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, _recipient_id: AccountId, _amount: U128) -> U128 {
        todo!()
    }

    pub fn burn(&mut self, _nft_id: NftId) {
        todo!()
    }

    pub fn transfer_nft(&mut self, _recipient_id: AccountId, _nft_id: NftId) {
        todo!()
    }

    pub fn sell_nft(&mut self, _nft_id: NftId, _price: U128) {
        todo!()
    }

    pub fn buy_nft(&mut self, _nft_id: NftId) -> U128 {
        todo!()
    }

    pub fn change_price(&mut self, _nft_id: NftId, _price: U128) {
        todo!()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}
