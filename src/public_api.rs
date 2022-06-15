use crate::*;
use near_sdk::{json_types::U128, AccountId};

#[near_bindgen]
impl Contract {
    pub fn transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.internal_transfer(sender_id, recipient_id, amount.0, false)
            .into()
    }

    pub fn burn(&mut self, nft_id: U128) {
        self.assert_running();
        self.nfts.burn_nft(&nft_id.0);
    }

    pub fn transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) {
        self.assert_running();
        self.nfts.transfer_nft(recipient_id, &nft_id.0);
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

    // TODO check lockups
    pub fn claim_lockup(&mut self, expire_on: u64) -> U128 {
        self.assert_running();
        let target_id = self.resolve_account(env::signer_account_pk());
        let mut target_account: Account = self
            .accounts
            .get(&target_id)
            .unwrap_or_else(|| env::panic_str("No such account id"))
            .into();
        let res = target_account.claim_lockup(expire_on);
        self.accounts
            .insert(&env::signer_account_id(), &target_account.into());
        U128(res)
    }

    pub fn claim_all_lockup(&mut self) -> U128 {
        self.assert_running();
        let target_id = self.resolve_account(env::signer_account_pk());
        let mut target_account: Account = self
            .accounts
            .get(&target_id)
            .unwrap_or_else(|| env::panic_str("No such account id"))
            .into();
        let res = target_account.claim_all_lockups();
        self.accounts
            .insert(&env::signer_account_id(), &target_account.into());
        U128(res)
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
    fn transfer() {
        let (mut contract, mut context) = init_test_env(None, None, Some(accounts(1)));
        let account_1 = Account::new(50);
        let account_2 = Account::new(10);

        contract.accounts.insert(&accounts(1), &account_1.into());
        contract.accounts.insert(&accounts(2), &account_2.into());

        testing_env!(context.signer_account_id(accounts(1)).build());

        contract.transfer(accounts(2), U128(25));

        let account_1: Account = contract.accounts.get(&accounts(1)).unwrap().into();
        let account_2: Account = contract.accounts.get(&accounts(2)).unwrap().into();
        assert_eq!(account_1.free, 25);
        assert_eq!(account_2.free, 35);
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

    #[test]
    #[ignore]
    fn claim_all_loockups() {
        // TODO fix me
        let mut owner = accounts(0);
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));

        let mut owner_account = Account::new(5);
        owner_account.lockups.insert(&Lockup::new(5, None));
        contract.accounts.insert(&owner, &owner_account.into());

        testing_env!(context
            .signer_account_id(accounts(0))
            .block_timestamp(99999999999999)
            .build());

        contract.claim_all_lockup();
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 10);
    }

    #[test]
    #[ignore]
    fn claim_loockup() {
        // TODO fix me
        let mut owner = accounts(0);
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));

        let mut owner_account = Account::new(50);
        owner_account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 0,
        });
        owner_account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 1,
        });
        owner_account.lockups.insert(&Lockup {
            amount: 5,
            expire_on: 3,
        });
        contract.accounts.insert(&owner, &owner_account.into());
        testing_env!(context
            .signer_account_id(accounts(0))
            .block_timestamp(2)
            .build());
        contract.claim_lockup(1);
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 55);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn claim_loockup_panic() {
        let mut owner = accounts(0);
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));
        contract.state = State::Paused;

        contract.claim_lockup(1);
    }
}
