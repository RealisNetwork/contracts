use near_sdk::{
    json_types::{U128, U64},
    require, AccountId,
};

use crate::{
    events::{
        BackendId, ChangeBeneficiary, ChangeConstantFee, ChangeDefaultLockupTime, ChangeOwnerId,
        ChangePercentFee, ChangeState, EventLog, EventLogVariant, LockupCreated, LockupRefund,
        NftMint,
    },
    lockup::{Lockup, SimpleLockup},
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
        owner_account.decrease_balance(amount.0);
        self.accounts.insert(&self.owner_id, &owner_account.into());

        let mut recipient_account: Account = self
            .accounts
            .get(&recipient_id.clone())
            .unwrap_or_else(|| Account::new(recipient_id.clone(), 0).into())
            .into();
        let lockup = SimpleLockup::new(amount.0, duration.map(|value| value.0));
        recipient_account
            .lockups
            .insert(&Lockup::GooglePlayBuy(lockup.clone()));
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
            .filter_map(|lockup| match lockup {
                Lockup::GooglePlayBuy(lockup) => Some(lockup),
                _ => None,
            })
            .find(|lockup| lockup.expire_on == expire_on.0)
            .unwrap_or_else(|| env::panic_str("No such lockup"));
        recipient_account
            .lockups
            .remove(&Lockup::GooglePlayBuy(lockup.clone()));
        self.accounts
            .insert(&recipient_id, &recipient_account.into());

        let mut owner_account: Account = self
            .accounts
            .get(&self.owner_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        owner_account.increase_balance(lockup.amount);
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

    pub fn owner_add_to_staking_pool(&mut self, amount: U128) -> U128 {
        self.assert_owner();
        let owner_id = env::signer_account_id();
        self.internal_add_pool(owner_id, amount.0).into()
    }

    pub fn owner_contract_setting(
        &mut self,
        constant_fee: Option<U128>,
        percent_fee: Option<u8>,
        owner_id: Option<AccountId>,
        beneficiary_id: Option<AccountId>,
        state: Option<State>,
        default_lockup_time: Option<U64>,
    ) {
        self.assert_owner();

        if let Some(constant_fee) = constant_fee {
            require!(
                self.constant_fee != constant_fee.0,
                "Constant fee can't be the same"
            );
            EventLog::from(EventLogVariant::ChangeConstantFee(ChangeConstantFee {
                from: &U128(self.constant_fee),
                to: &constant_fee.clone(),
            }))
            .emit();
            self.constant_fee = constant_fee.0;
        }

        if let Some(percent_fee) = percent_fee {
            require!(
                self.percent_fee != percent_fee,
                "Percent fee can't be the same"
            );
            EventLog::from(EventLogVariant::ChangePercentFee(ChangePercentFee {
                from: self.percent_fee,
                to: percent_fee,
            }))
            .emit();
            self.percent_fee = percent_fee;
        }

        if let Some(owner_id) = owner_id {
            require!(self.owner_id != owner_id, "Owner id can't be the same");
            EventLog::from(EventLogVariant::ChangeOwnerId(ChangeOwnerId {
                from: &self.owner_id.clone(),
                to: &owner_id,
            }))
            .emit();
            self.owner_id = owner_id;
        }

        if let Some(beneficiary_id) = beneficiary_id {
            require!(
                self.beneficiary_id != beneficiary_id,
                "Beneficiary can't be the same"
            );
            EventLog::from(EventLogVariant::ChangeBeneficiary(ChangeBeneficiary {
                from: &self.beneficiary_id.clone(),
                to: &beneficiary_id,
            }))
            .emit();
            self.beneficiary_id = beneficiary_id;
        }

        if let Some(state) = state {
            require!(self.state != state, "State can't be the same");
            EventLog::from(EventLogVariant::ChangeState(ChangeState {
                from: self.state.clone(),
                to: state.clone(),
            }))
            .emit();
            self.state = state;
        }

        if let Some(default_lockup_time) = default_lockup_time {
            require!(
                self.staking.default_lockup_time != default_lockup_time.0,
                "Lockup time can't be the same"
            );
            EventLog::from(EventLogVariant::ChangeDefaultLockupTime(
                ChangeDefaultLockupTime {
                    from: &U64(self.staking.default_lockup_time),
                    to: &default_lockup_time,
                },
            ))
            .emit();
            self.staking.default_lockup_time = default_lockup_time.0;
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;
    use near_sdk::json_types::U64;

    #[test]
    fn contract_settings() {
        let (mut contract, mut context) =
            init_test_env(Some(accounts(0)), Some(accounts(0)), Some(accounts(0)));

        contract.owner_id = accounts(0);

        contract
            .accounts
            .insert(&accounts(1), &Account::new(accounts(0), 0).into());

        contract.owner_contract_setting(None, None, None, None, None, None);
        contract.owner_contract_setting(Some(U128(100)), None, None, None, None, None);
        contract.owner_contract_setting(None, Some(15), None, None, None, None);
        contract.owner_contract_setting(None, None, Some(accounts(3)), None, None, None);

        testing_env!(context.signer_account_id(accounts(3)).build());

        contract.owner_contract_setting(None, None, None, Some(accounts(5)), None, None);
        contract.owner_contract_setting(None, None, None, None, Some(State::Paused), None);
        contract.owner_contract_setting(None, None, None, None, None, Some(U64(10)));

        let current_settings = contract.get_contract_settings();
        assert_eq!(current_settings.state, State::Paused);
        assert_eq!(current_settings.owner_id, accounts(3));
        assert_eq!(current_settings.percent_fee, 15);
        assert_eq!(current_settings.constant_fee.0, 100);
        assert_eq!(current_settings.beneficiary_id, accounts(5));
        assert_eq!(contract.staking.default_lockup_time, 10);
    }

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
        contract.owner_contract_setting(None, None, None, Some(account_id_new.clone()), None, None);
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

        contract.owner_contract_setting(None, None, None, Some(beneficiary_id.clone()), None, None);
        assert_eq!(contract.beneficiary_id, beneficiary_id);
    }

    #[test]
    #[should_panic = "State can't be the same"]
    fn change_the_same_state_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_contract_setting(None, None, None, None, Some(State::Running), None);

        assert_eq!(contract.state, State::Running);
    }

    #[test]
    fn change_state_test() {
        let owner_id = accounts(0);
        let (mut contract, _context) = init_test_env(None, None, None);
        contract.owner_id = owner_id;

        contract.owner_contract_setting(None, None, None, None, Some(State::Paused), None);
        assert_eq!(contract.state, State::Paused)
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

    #[test]
    fn owner_add_to_pull() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        testing_env!(context.signer_account_id(user1).build());

        // user 1 stakes 4 lis
        contract.stake(U128(4 * ONE_LIS));

        // set signer as owner
        testing_env!(context.signer_account_id(owner).build());

        contract.owner_add_to_staking_pool(U128(100 * ONE_LIS));

        assert_eq!(contract.staking.get_total_supply(), 104 * ONE_LIS);
        assert_eq!(contract.staking.get_total_x_supply(), 4 * ONE_LIS * 1000);
    }

    #[test]
    #[should_panic = "Zero pool balance"]
    fn owner_add_to_zero_pull() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // set signer as owner
        testing_env!(context.signer_account_id(owner).build());

        contract.owner_add_to_staking_pool(U128(100 * ONE_LIS));

        assert_eq!(contract.staking.get_total_supply(), 100 * ONE_LIS);
        assert_eq!(contract.staking.get_total_x_supply(), 0);
    }

    #[test]
    #[should_panic = "Not enough balance"]
    fn owner_add_to_pull_over_balance() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        testing_env!(context.signer_account_id(user1).build());

        // user 1 stakes 4 lis
        contract.stake(U128(4 * ONE_LIS));

        // set signer as owner
        testing_env!(context.signer_account_id(owner).build());

        contract.owner_add_to_staking_pool(U128(3_000_000_001 * ONE_LIS));
    }

    #[test]
    #[should_panic = "Only owner can do this"]
    fn owner_add_to_pull_not_owner() {
        // Init contract
        let (mut contract, mut context) = init_test_env(Some(accounts(1)), None, None);

        // set signer as owner
        testing_env!(context.signer_account_id(accounts(0)).build());

        contract.owner_add_to_staking_pool(U128(3_000_000_000 * ONE_LIS));
    }
}
