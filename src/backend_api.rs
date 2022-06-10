use crate::{types::NftId, *};
use near_sdk::{json_types::U128, near_bindgen, AccountId};

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn backend_transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    #[allow(unused_variables)]
    pub fn backend_burn(&mut self, nft_id: NftId) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_transfer_nft(&mut self, recipient_id: AccountId, nft_id: NftId) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_sell_nft(&mut self, nft_id: NftId, price: U128) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_buy_nft(&mut self, nft_id: NftId) -> U128 {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_change_price(&mut self, nft_id: NftId, price: U128) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}
