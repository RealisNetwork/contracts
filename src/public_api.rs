use crate::{types::NftId, *};
use near_sdk::{json_types::U128, AccountId};

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        let sender_id = env::signer_account_id();
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    pub fn burn(&mut self, _nft_id: NftId) {
        self.assert_running();
        todo!()
    }

    pub fn transfer_nft(&mut self, _recipient_id: AccountId, _nft_id: NftId) {
        self.assert_running();
        todo!()
    }

    pub fn sell_nft(&mut self, _nft_id: NftId, _price: U128) {
        self.assert_running();
        todo!()
    }

    pub fn buy_nft(&mut self, _nft_id: NftId) -> U128 {
        self.assert_running();
        todo!()
    }

    pub fn change_price(&mut self, _nft_id: NftId, _price: U128) {
        self.assert_running();
        todo!()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}
