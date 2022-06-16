use near_sdk::{
    json_types::{U128, U64},
    require, AccountId,
};

use crate::{
    events::{
        ChangeBeneficiaryLog, ChangeStateLog, EventLog, EventLogVariant, LockupCreatedLog,
        LockupRefundLog, NftMintLog,
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
    /// assert_owner();
    /// ```
    /// # Arguments
    ///  * `recipient_id`- `AccountId` of future nft owner.
    ///  * `nft_metadata`-specific for new nft metadata.

    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) -> U128 {
        self.assert_owner();

        EventLog::from(EventLogVariant::NftMint(NftMintLog {
            owner_id: String::from(recipient_id.clone()),
            meta_data: nft_metadata.clone(),
        }))
        .emit();

        let mut nft_owner_id = Account::from(
            self.accounts
                .get(&recipient_id)
                .unwrap_or_else(|| env::panic_str("Account not found")),
        );

        let nft_id = self.nfts.mint_nft(&recipient_id, nft_metadata);
        nft_owner_id.nfts.insert(&nft_id);
        self.accounts
            .insert(&recipient_id, &VAccount::V1(nft_owner_id));

        nft_id.into()
    }

    pub fn change_state(&mut self, state: State) {
        self.assert_owner();
        require!(self.state != state, "State can't be the same");
        EventLog::from(EventLogVariant::ChangeState(ChangeStateLog {
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
        EventLog::from(EventLogVariant::ChangeBeneficiary(ChangeBeneficiaryLog {
            from: self.beneficiary_id.clone(),
            to: new_beneficiary_id.clone(),
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
        EventLog::from(EventLogVariant::LockupCreatedLog(LockupCreatedLog {
            amount: U128(lockup.amount),
            recipient_id,
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

        EventLog::from(EventLogVariant::LockupRefundLog(LockupRefundLog {
            amount: U128(lockup.amount),
            account_id: recipient_id,
            timestamp: U64(lockup.expire_on),
        }))
        .emit();
        lockup.amount.into()
    }
}

#[cfg(test)]
mod tests {
    use crate::utils::tests_utils::*;

    #[test]
    #[should_panic]
    fn mint_nft_test_panic() {
        let (mut contract, _context) =
            init_test_env(Some(accounts(0)), Some(accounts(0)), Some(accounts(0)));

        contract.mint(accounts(2), "some_metadata".to_string());
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
}
