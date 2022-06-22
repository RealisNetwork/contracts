use near_sdk::{
    json_types::{U128, U64},
    require, AccountId,
};

use crate::{
    events::{
        BackendId, ChangeBeneficiary, ChangeState, EventLog, EventLogVariant, LockupCreated,
        LockupRefund, NftMint,
    },
    lockup::Lockup,
    *,
};

#[near_bindgen]
impl Contract {
    /// `fn mint` creates new nft with uniq id.
    /// `fn mint` could be used only by the contract owner.
    ///  # Examples
    /// ```
    /// use near_sdk::json_types::U128;
    /// use near_sdk::test_utils::accounts;
    /// use realis_near::Contract;
    /// use realis_near::nft::Nft;
    ///
    /// let mut contract = Contract::new(
    ///     Some(U128(3000000000)),
    ///     Some(U128(50)),
    ///     Some(10),
    ///     None,
    ///     None
    /// );
    /// let nft_id = contract.nfts.mint_nft(&accounts(0), "Duck".to_string());
    /// let inserted_nft: Nft = contract.nfts.get_nft(&nft_id).into();
    /// assert_eq!(inserted_nft.owner_id, accounts(0));
    /// assert_eq!(inserted_nft.get_metadata(), "Duck".to_string());
    ///
    /// ```
    /// # Arguments
    ///  * `recipient_id`- `AccountId` of future nft owner.
    ///  * `nft_metadata`-specific for new nft metadata.

    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) -> U128 {
        self.assert_owner();

        EventLog::from(EventLogVariant::NftMint(NftMint {
            owner_id: &recipient_id,
            meta_data: &nft_metadata,
        }))
        .emit();

        let mut nft_owner = Account::from(
            self.accounts
                .get(&recipient_id)
                .unwrap_or_else(|| Account::new(recipient_id.clone(), 0).into()),
        );

        let nft_id = self.nfts.mint_nft(&recipient_id, nft_metadata);
        nft_owner.nfts.insert(&nft_id);
        self.accounts
            .insert(&recipient_id, &VAccount::V1(nft_owner));

        nft_id.into()
    }

    pub fn change_state(&mut self, state: State) {
        self.assert_owner();
        require!(self.state != state, "State can't be the same");
        EventLog::from(EventLogVariant::ChangeState(ChangeState {
            from: self.state.clone(),
            to: state.clone(),
        }))
        .emit();

        self.state = state;
    }

    pub fn change_beneficiary(&mut self, new_beneficiary_id: AccountId) {
        self.assert_owner();
        require!(
            self.beneficiary_id != new_beneficiary_id,
            "Beneficiary can't be the same"
        );
        EventLog::from(EventLogVariant::ChangeBeneficiary(ChangeBeneficiary {
            from: &self.beneficiary_id,
            to: &new_beneficiary_id,
        }))
        .emit();

        self.beneficiary_id = new_beneficiary_id;
    }

    /// Create lockup for account and get tokens from owner account
    pub fn create_lockup(
        &mut self,
        recipient_id: AccountId,
        amount: U128,
        duration: Option<U64>,
    ) -> U64 {
        self.assert_owner();

        let mut owner_account: Account = self
            .accounts
            .get(&self.owner_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        owner_account.free = owner_account
            .free
            .checked_sub(amount.0)
            .unwrap_or_else(|| env::panic_str("Not enough balance"));
        self.accounts.insert(&self.owner_id, &owner_account.into());

        let mut recipient_account: Account = self
            .accounts
            .get(&recipient_id.clone())
            .unwrap_or_else(|| Account::new(recipient_id.clone(), 0).into())
            .into();
        let lockup = Lockup::new(amount.0, duration.map(|value| value.0));
        recipient_account.lockups.insert(&lockup);
        self.accounts
            .insert(&recipient_id, &recipient_account.into());
        EventLog::from(EventLogVariant::LockupCreated(LockupCreated {
            amount: U128(lockup.amount),
            recipient_id: &recipient_id,
            expire_on: U64(lockup.expire_on),
        }))
        .emit();
        lockup.expire_on.into()
    }

    /// Remove lockup from account and return balance to owner
    pub fn refund_lockup(&mut self, recipient_id: AccountId, expire_on: U64) -> U128 {
        self.assert_owner();

        let mut recipient_account: Account = self
            .accounts
            .get(&recipient_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        let lockup = recipient_account
            .lockups
            .iter()
            .find(|lockup| lockup.expire_on == expire_on.0)
            .unwrap_or_else(|| env::panic_str("No such lockup"));
        recipient_account.lockups.remove(&lockup);
        self.accounts
            .insert(&recipient_id, &recipient_account.into());

        let mut owner_account: Account = self
            .accounts
            .get(&self.owner_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        owner_account.free += lockup.amount;
        self.accounts.insert(&self.owner_id, &owner_account.into());

        EventLog::from(EventLogVariant::LockupRefund(LockupRefund {
            amount: U128(lockup.amount),
            account_id: &recipient_id,
            timestamp: U64(lockup.expire_on),
        }))
        .emit();
        lockup.amount.into()
    }

    /// Inserts each backend account id in backend accounts of contract
    pub fn owner_add_backends(&mut self, account_ids: Vec<AccountId>) {
        self.assert_owner();

        // Checks that elements will not be duplicated and inserts it
        account_ids.iter().for_each(|account_id| {
            if !self.backend_ids.insert(account_id) {
                env::panic_str("Can't insert twice");
            }
        });

        // Throws an event
        EventLog::from(EventLogVariant::AddBackendId(BackendId {
            accounts: &account_ids,
        }))
        .emit();
    }

    /// Removes each account id from backend account of contract
    pub fn owner_remove_backends(&mut self, account_ids: Vec<AccountId>) {
        self.assert_owner();

        // Checks if every element of account_ids is unique
        account_ids.iter().for_each(|account_id| {
            if account_ids
                .iter()
                .filter(|account_id_ext| account_id_ext == &account_id)
                .count()
                > 1
            {
                env::panic_str("Can't remove twice");
            }
        });

        // Checks if removable element exists and removes it
        account_ids.iter().for_each(|account_id| {
            if !self.backend_ids.remove(account_id) {
                env::panic_str("No such account_id");
            }
        });

        // Throws an event
        EventLog::from(EventLogVariant::RemoveBackendId(BackendId {
            accounts: &account_ids,
        }))
        .emit();
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;

    #[test]
    fn mint_nft_test() {
        let (mut contract, _context) =
            init_test_env(Some(accounts(0)), Some(accounts(0)), Some(accounts(0)));

        contract.owner_id = accounts(0);

        contract
            .accounts
            .insert(&accounts(1), &Account::new(accounts(0), 0).into());

        let res = contract.mint(accounts(1), "some_metadata".to_string());

        let assertion = contract.nfts.get_nft_map().keys().any(|key| key == res.0);
        assert!(assertion);
        let account: Account = contract.accounts.get(&accounts(1)).unwrap().into();
        assert!(account.nfts.contains(&res.0));
    }

    #[test]
    fn change_beneficiary_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(Some(owner_id.clone()), None, None);
        contract.owner_id = owner_id;

        let account_id_new = accounts(1);
        contract.change_beneficiary(account_id_new.clone());
        assert_eq!(contract.beneficiary_id, account_id_new);
    }

    #[test]
    #[should_panic = "Beneficiary can't be the same"]
    fn change_the_same_beneficiary_test() {
        let owner_id = accounts(0);
        let beneficiary_id = accounts(1);
        let (mut contract, _context) =
            init_test_env(Some(owner_id.clone()), Some(beneficiary_id.clone()), None);
        contract.owner_id = owner_id;

        contract.change_beneficiary(beneficiary_id.clone());
        assert_eq!(contract.beneficiary_id, beneficiary_id);
    }

    #[test]
    #[should_panic = "State can't be the same"]
    fn change_the_same_state_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        let contract_new_state = State::Running;
        contract.change_state(contract_new_state.clone());
        assert_eq!(contract.state, contract_new_state)
    }

    #[test]
    fn change_state_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        let contract_new_state = State::Paused;
        contract.change_state(contract_new_state.clone());
        assert_eq!(contract.state, contract_new_state)
    }

    #[test]
    fn refund_lockup_test() {
        let (mut contract, _context) =
            init_test_env(Some(accounts(0)), Some(accounts(0)), Some(accounts(0)));

        contract.owner_id = accounts(0);

        contract
            .accounts
            .insert(&accounts(1), &Account::new(accounts(0), 0).into());

        let res = contract.create_lockup(accounts(1), U128(300000 * ONE_LIS), None);

        let assertion = contract.refund_lockup(accounts(1), res);
        assert_eq!(assertion, U128(300000 * ONE_LIS));
    }

    #[test]
    fn owner_add_backends_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_add_backends(vec![accounts(1), accounts(2), accounts(3)]);

        let expected_backend_ids = vec![accounts(0), accounts(1), accounts(2), accounts(3)];

        expected_backend_ids.iter().for_each(|epx_backend_id| {
            assert!(contract.backend_ids.contains(epx_backend_id));
        });
    }

    #[test]
    #[should_panic = "Can't insert twice"]
    fn owner_add_the_same_backends_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_add_backends(vec![accounts(1), accounts(1), accounts(2)]);

        let expected_backend_ids = vec![accounts(0), accounts(1), accounts(2)];

        expected_backend_ids.iter().for_each(|epx_backend_id| {
            assert!(contract.backend_ids.contains(epx_backend_id));
        });
    }

    #[test]
    fn owner_remove_backends_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_add_backends(vec![accounts(1), accounts(2), accounts(3)]);
        contract.owner_remove_backends(vec![accounts(1), accounts(2)]);

        let unexpected_backend_ids = vec![accounts(1), accounts(2)];
        unexpected_backend_ids.iter().for_each(|unepx_backend_id| {
            assert!(!contract.backend_ids.contains(unepx_backend_id));
        });
    }

    #[test]
    #[should_panic = "No such account_id"]
    fn owner_remove_backends_not_exist_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_add_backends(vec![accounts(1), accounts(2), accounts(3)]);
        contract.owner_remove_backends(vec![accounts(1), accounts(4)]);

        let expected_backend_ids = vec![accounts(0), accounts(1), accounts(2), accounts(3)];
        expected_backend_ids.iter().for_each(|epx_backend_id| {
            assert!(contract.backend_ids.contains(epx_backend_id));
        });
    }
    #[test]
    #[should_panic = "Can't remove twice"]
    fn owner_remove_backends_the_same_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_add_backends(vec![accounts(1), accounts(2), accounts(3)]);
        contract.owner_remove_backends(vec![accounts(1), accounts(1)]);

        let expected_backend_ids = vec![accounts(0), accounts(2), accounts(3)];

        expected_backend_ids.iter().for_each(|epx_backend_id| {
            assert!(contract.backend_ids.contains(epx_backend_id));
        });
    }

    #[test]
    #[should_panic = "Only owner can do this"]
    fn owner_not_set_add_backends_test() {
        let (mut contract, mut context) = init_test_env(None, None, None);

        testing_env!(context.signer_account_id(accounts(2)).build());

        contract.owner_add_backends(vec![accounts(1), accounts(2), accounts(3)]);
    }

    #[test]
    #[should_panic = "Only owner can do this"]
    fn owner_not_set_remove_backends_test() {
        let (mut contract, mut context) = init_test_env(None, None, None);

        testing_env!(context.signer_account_id(accounts(2)).build());

        contract.owner_remove_backends(vec![accounts(1), accounts(2)]);
    }
}
