use crate::*;
use near_sdk::{json_types::U128, near_bindgen, AccountId, Timestamp};

#[near_bindgen]
impl Contract {
    pub fn backend_transfer(&mut self, recipient_id: AccountId, amount: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        self.internal_transfer(
            self.resolve_account(env::signer_account_pk()),
            recipient_id,
            amount.0,
            true,
        )
        .into()
    }

    pub fn backend_burn(&mut self, nft_id: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        let sender_free = self.take_fee(sender_id.clone(), None, true);
        self.nfts.burn_nft(&nft_id.0, sender_id);
        sender_free.into()
    }

    pub fn backend_transfer_nft(&mut self, recipient_id: AccountId, nft_id: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        let sender_free = self.take_fee(sender_id.clone(), None, true);
        self.nfts
            .transfer_nft(sender_id.clone(), recipient_id.clone(), &nft_id.0);
        let mut last_owner: Account = self
            .accounts
            .get(&sender_id)
            .unwrap_or_else(|| env::panic_str("No such account id (signer)"))
            .into();
        last_owner.nfts.remove(&nft_id.0);
        let mut new_owner: Account = self
            .accounts
            .get(&recipient_id)
            .unwrap_or_else(|| env::panic_str("No such account id (recipient)"))
            .into();
        new_owner.nfts.insert(&nft_id.0);
        self.accounts.insert(&sender_id, &last_owner.into());
        self.accounts.insert(&recipient_id, &new_owner.into());
        sender_free.into()
    }

    #[allow(unused_variables)]
    pub fn backend_sell_nft(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.take_fee(sender_id, None, true);
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_buy_nft(&mut self, nft_id: U128) -> U128 {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.take_fee(sender_id, None, true);
        todo!()
    }

    #[allow(unused_variables)]
    pub fn backend_change_price(&mut self, nft_id: U128, price: U128) {
        self.assert_running();
        self.assert_backend();
        let sender_id = self.resolve_account(env::signer_account_pk());
        self.take_fee(sender_id, None, true);
        todo!()
    }

    // TODO check lockups
    pub fn backend_claim_lockup(&mut self, expire_on: Timestamp) -> U128 {
        self.assert_running();
        self.assert_backend();
        let target_id = self.resolve_account(env::signer_account_pk());
        let mut target_account: Account = self
            .accounts
            .get(&target_id)
            .unwrap_or_else(|| env::panic_str("No such account id"))
            .into();
        let res = target_account.claim_lockup(expire_on);
        self.accounts.insert(&target_id, &target_account.into());
        U128(res)
    }

    pub fn backend_claim_all_lockup(&mut self) -> U128 {
        self.assert_running();
        self.assert_backend();
        let target_id = self.resolve_account(env::signer_account_pk());

        let mut target_account: Account = self
            .accounts
            .get(&target_id)
            .unwrap_or_else(|| env::panic_str("No such account id"))
            .into();
        let res = target_account.claim_all_lockups();
        self.accounts.insert(&target_id, &target_account.into());
        U128(res)
    }

    // TODO: delegate nft
    // Discuss general structure of delegation
}

#[cfg(test)]
mod tests {
    use crate::{nft::Nft, utils::tests_utils::*};

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
    fn backend_transfer() {
        let owner = accounts(0);
        let (mut contract, mut context) = init_test_env(None, None, Some(owner.clone()));
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let account_2 = Account::new(accounts(2), 10 * ONE_LIS);

        contract.accounts.insert(&accounts(2), &account_2.into());
        contract.registered_accounts.insert(&owner_pk, &owner);

        testing_env!(context
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .build());

        contract.backend_transfer(accounts(2), U128(20 * ONE_LIS));

        let account_2: Account = contract.accounts.get(&accounts(2)).unwrap().into();
        let owner_acc: Account = contract.accounts.get(&owner).unwrap().into();

        assert_eq!(owner_acc.free, 2999999980 * ONE_LIS);
        assert_eq!(account_2.free, 30 * ONE_LIS);
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
        let owner = accounts(0);
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let (mut contract, mut context) = init_test_env(None, None, None);
        contract.registered_accounts.insert(&owner_pk, &owner);

        testing_env!(context
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .build());

        contract.backend_burn(U128(1));
    }

    #[test]
    fn backend_burn_nft_test() {
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let owner = accounts(0);
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.nfts.mint_nft(&owner, "Duck".to_string());
        contract.registered_accounts.insert(&owner_pk, &owner);

        testing_env!(context
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .build());

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
        let owner = accounts(0);
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let receiver = accounts(1);

        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);
        let nft_id = contract.nfts.mint_nft(&owner, "Duck".to_string());
        contract.registered_accounts.insert(&owner_pk, &owner);
        contract.accounts.insert(&receiver, &Account::new(receiver.clone(),0).into());

        testing_env!(context
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .build());

        contract.backend_transfer_nft(receiver.clone(), U128(nft_id));
        let nft: Nft = contract.nfts.get_nft(&nft_id).into();

        assert_eq!(nft.owner_id, receiver);
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
    fn backend_claim_all_lockups() {
        let owner = accounts(0);
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));

        let mut owner_account = Account::new(accounts(1), 5);
        owner_account.lockups.insert(&Lockup::new(5, None));
        contract.accounts.insert(&owner, &owner_account.into());
        contract.registered_accounts.insert(&owner_pk, &owner);

        testing_env!(context
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .block_timestamp(99999999999999999)
            .build());

        contract.backend_claim_all_lockup();
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 10);
    }
    #[test]
    #[should_panic = "Not allowed"]
    fn backend_claim_all_lockups_panic() {
        let owner = accounts(0);
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, Some(accounts(1)));
        contract.registered_accounts.insert(&owner_pk, &owner);

        testing_env!(context
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .block_timestamp(99999999999999999)
            .build());

        let mut owner_account = Account::new(owner.clone(), 5);
        owner_account.lockups.insert(&Lockup::new(5, None));
        contract.accounts.insert(&owner, &owner_account.into());
        contract.backend_claim_all_lockup();
    }

    #[test]
    fn backend_claim_lockup() {
        let owner_pk = PublicKey::from_str("ed25519:7fVmPQUiCCw783pxBYYnskeyuQX9NprUe6tM3WsdRLVA").unwrap();
        let owner = accounts(0);
        let (mut contract, mut context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));
        contract.registered_accounts.insert(&owner_pk, &owner);

        let mut owner_account = Account::new(owner.clone(), 50);
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
            .signer_account_id(owner.clone())
            .signer_account_pk(owner_pk)
            .block_timestamp(2)
            .build());
        contract.backend_claim_lockup(1);
        let res_owner_account: Account = contract.accounts.get(&owner).unwrap().into();
        assert_eq!(res_owner_account.free, 55);
    }

    #[test]
    #[should_panic = "Contract is paused"]
    fn backend_claim_lockup_panic() {
        let owner = accounts(0);
        let (mut contract, _context) =
            init_test_env(Some(owner.clone()), None, Some(owner.clone()));
        contract.state = State::Paused;

        contract.backend_claim_lockup(1);
    }
}
