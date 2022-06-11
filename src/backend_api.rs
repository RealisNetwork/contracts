use crate::{types::NftId, *};
use near_sdk::{json_types::U128, require, near_bindgen, AccountId};

#[near_bindgen]
impl Contract {
    #[allow(unused_variables)]
    pub fn backend_transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        require!(self.state == State::Running, "Contract is paused");
        require!(env::signer_account_id() == self.backend_id, "Not allowed");
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    #[allow(unused_variables)]
    pub fn backend_burn(&mut self, nft_id: NftId) {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_transfer_nft(&mut self, recipient_id: AccountId, nft_id: NftId) {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_sell_nft(&mut self, nft_id: NftId, price: U128) {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_buy_nft(&mut self, nft_id: NftId) -> U128 {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_change_price(&mut self, nft_id: NftId, price: U128) {
        todo!()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}
