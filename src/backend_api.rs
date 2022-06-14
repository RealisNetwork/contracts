use crate::*;
use near_sdk::{json_types::U128, near_bindgen, AccountId};

#[near_bindgen]
impl Contract {
    pub fn backend_transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.internal_transfer(sender_id, recipient_id, amount.0)
            .into()
    }

    pub fn backend_burn(&mut self, nft_id: U128) {
        self.assert_running();
        self.assert_backend();
        self.nfts.burn_nft(nft_id.0);
    }

    pub fn backend_transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) {
        self.assert_running();
        self.assert_backend();
        self.nfts.transfer_nft(recipient_id, nft_id.0);
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

    //TODO check lockups
    pub fn backend_claim_lockup(&mut self, expire_on: u64) -> U128 {
        self.assert_running();
        self.assert_backend();
        let target_id = self.resolve_account(env::signer_account_pk());
        let mut target_account: Account = self.accounts.get(&target_id).unwrap().into();
        U128(target_account.claim_lockup(expire_on))
    }


    pub fn backend_claim_all_lockup(& mut self) ->U128 {
        self.assert_running();
        self.assert_backend();
        let mut target_account: Account = self.accounts.get(&env::signer_account_id()).unwrap().into();
        U128(target_account.claim_all_lockups())
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
        contract.backend_burn(U128(1));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_burn_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));

        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_burn(U128(1));
    }

    #[test]
    #[should_panic = "Nft not exist"]
    fn backend_burn_nft_test_not_exists() {
        let mut owner = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.backend_burn(U128(1));
    }

    #[test]
    fn backend_burn_nft_test() {
        let mut owner = accounts(0);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.nfts.mint_nft(owner, "Duck".to_string());
        assert_eq!(contract.nfts.nft_count(), 1);
        contract.backend_burn(U128(nft_id));
        assert_eq!(contract.nfts.nft_count(), 0);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_sell_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.state = State::Paused;
        contract.backend_sell_nft(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_sell_nft_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));
        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_sell_nft(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_change_price_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.state = State::Paused;
        contract.backend_change_price(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_change_price_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));
        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_change_price(U128(1), U128(100));
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_transfer_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.state = State::Paused;
        contract.backend_transfer_nft(accounts(1), U128(100));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_transfer_nft_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));
        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_transfer_nft(accounts(1), U128(1));
    }

    #[test]
    fn backend_transfer_nft_test() {
        let mut owner = accounts(0);
        let mut reciver = accounts(1);
        let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.nfts.mint_nft(owner, "Duck".to_string());
        contract.backend_transfer_nft(reciver.clone(), U128(nft_id));
        assert_eq!(contract.nfts.get_nft(nft_id).owner_id, reciver);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_buy_nft_assert_running() {
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.state = State::Paused;
        contract.backend_buy_nft(U128(1));
    }

    #[test]
    #[should_panic = "Not allowed"]
    fn backend_buy_nft_assert_backend() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));
        testing_env!(context.predecessor_account_id(accounts(2)).build());
        contract.backend_buy_nft(U128(100));
    }

    #[test]
    fn backend_claim_all_loockups() {
        let mut owner = accounts(0);
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, Some(owner.clone()));

        let mut owner_account = Account::new(5);
        owner_account.lockups.insert(&Lockup::new(5, None));
        contract.accounts.insert(&owner, &owner_account.into());

        testing_env!(context.signer_account_id(accounts(0)).block_timestamp(99999999999999).build());

        println!("{:#?}",contract.backend_claim_all_lockup());
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 10);
    }

    // #[test]
    // fn backend_claim_loockup(){
    //     let mut owner = accounts(0);
    //     let (mut contract, _context) = init_test_env(Some(owner.clone()), None, None);
    //     let mut owner_account = Account::new(50);
    // }
}
