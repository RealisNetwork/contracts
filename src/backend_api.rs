use crate::*;
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
    pub fn backend_burn(&mut self, nft_id: U128) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_sell_nft(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_buy_nft(&mut self, nft_id: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_change_price(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        self.assert_backend();
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
    fn backend_transfer_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.backend_transfer(accounts(1), U128(100));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_transfer_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_transfer(accounts(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_burn_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.backend_burn(1);
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_burn_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_burn(1);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_sell_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.backend_sell_nft(1, U128(100));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_sell_nft_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_sell_nft(1, U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_change_price_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.backend_change_price(1, U128(100));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_change_price_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_change_price(1, U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_transfer_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.backend_transfer_nft(accounts(1), 100);
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_transfer_nft_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_transfer_nft(accounts(1), 1);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_buy_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.backend_buy_nft(1);
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_buy_nft_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_buy_nft(100);
    }
}
