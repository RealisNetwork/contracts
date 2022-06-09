use crate::types::NftId;
use crate::*;
use near_sdk::json_types::U128;
use near_sdk::AccountId;

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        let sender_id = env::signer_account_id();
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
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
