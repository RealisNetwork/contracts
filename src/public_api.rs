use crate::*;
use near_sdk::{json_types::U128, AccountId};

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        let sender_id = env::signer_account_id();
        // TODO: do not take fee from sender
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    #[allow(unused_variables)]
    pub fn burn(&mut self, nft_id: U128) {
        self.assert_running();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) {
        self.assert_running();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn sell_nft(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn buy_nft(&mut self, nft_id: U128) -> U128 {
        self.assert_running();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn change_price(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        todo!()
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;

    #[test]
    #[should_panic = "Contract is paused"]
    fn transfer_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.transfer(accounts(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn burn_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.burn(U128(1));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn sell_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.sell_nft(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn change_price_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.change_price(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn transfer_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.transfer_nft(accounts(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn buy_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.buy_nft(U128(1));
    }
}
