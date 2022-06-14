use crate::*;
use near_sdk::{json_types::U128, AccountId};

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        let sender_id = env::signer_account_id();
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    pub fn burn(&mut self, nft_id: U128) {
        self.assert_running();
        self.nfts.burn_nft(nft_id.0);
    }

    pub fn transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) {
        self.assert_running();
        self.nfts.transfer_nft(recipient_id, nft_id.0);
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
    fn burn_nft_test() {
        let mut owner = accounts(0);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.nfts.mint_nft(owner, "Duck".to_string());
        assert_eq!(contract.nfts.nft_count(), 1);
        contract.burn(U128(nft_id));
        assert_eq!(contract.nfts.nft_count(), 0);
    }

    #[test]
    #[should_panic = "Nft not exist"]
    fn burn_nft_test_not_exists() {
        let mut owner = accounts(0);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
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
    fn transfer_nft_test() {
        let mut owner = accounts(0);
        let mut reciver = accounts(1);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.nfts.mint_nft(owner, "Duck".to_string());
        contract.transfer_nft(reciver.clone(), U128(nft_id));
        assert_eq!(contract.nfts.get_nft(nft_id).owner_id, reciver);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn buy_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);

        contract.state = State::Paused;
        contract.buy_nft(U128(1));
    }
}
