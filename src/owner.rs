use near_sdk::{json_types::U128, require, AccountId, Timestamp};

use crate::{
    events::{ChangeBeneficiaryLog, ChangeStateLog, EventLog, EventLogVariant, NftMintLog},
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

    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) -> u128 {
        self.assert_owner();

        EventLog::from(EventLogVariant::NftMint(NftMintLog {
            owner_id: String::from(recipient_id.clone()),
            meta_data: nft_metadata.clone(),
        }))
        .emit();

        self.nfts.mint_nft(recipient_id, nft_metadata)
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

    #[allow(unused_variables)]
    pub fn create_lockup(
        &mut self,
        recipient_id: AccountId,
        amount: U128,
        duration: Option<Timestamp>,
    ) -> Timestamp {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::utils::tests_utils::*;

    #[test]
    #[should_panic]
    fn mint_nft_test_panic() {
        let (mut contract, mut context) = init_test_env(
            Some(AccountId::new_unchecked("not_owner".to_string())),
            None,
            None,
        );
        testing_env!(context
            .signer_account_id(AccountId::new_unchecked("not_owner".to_string()))
            .build());

        contract.mint(
            AccountId::new_unchecked("user_id".to_string()),
            "some_metadata".to_string(),
        );
    }

    #[test]
    fn mint_nft_test() {
        let owner_id = AccountId::new_unchecked("owner".to_string());
        let (mut contract, mut context) = init_test_env(
            Some(AccountId::new_unchecked("user_id".to_string())),
            None,
            None,
        );

        testing_env!(VMContextBuilder::new()
            .signer_account_id(owner_id)
            .is_view(false)
            .build());

        contract.accounts.insert(
            &AccountId::new_unchecked("user_id".to_string()),
            &Account::default().into(),
        );

        let res = contract.mint(
            AccountId::new_unchecked("user_id".to_string()),
            "some_metadata".to_string(),
        );

        let assertion = contract.nfts.get_nft_map().keys().any(|key| key == res);
        assert!(assertion);
        let account: Account = contract
            .accounts
            .get(&AccountId::new_unchecked("user_id".to_string()))
            .unwrap()
            .into();
        assert!(account.nfts.contains(&res));
    }

    #[test]
    fn change_beneficiary_test() {
        let owner_id = AccountId::new_unchecked("owner".to_string());
        let (mut contract, mut context) = init_test_env(Some(owner_id.clone()), None, None);

        testing_env!(VMContextBuilder::new()
            .signer_account_id(owner_id)
            .is_view(false)
            .build());

        let account_id_new = AccountId::from_str("new_beneficiary").unwrap();
        contract.change_beneficiary(account_id_new.clone());
        assert_eq!(contract.beneficiary_id, account_id_new);
    }

    #[test]
    fn change_state_test() {
        let owner_id = AccountId::new_unchecked("owner".to_string());
        let (mut contract, mut context) = init_test_env(None, None, None);

        testing_env!(VMContextBuilder::new()
            .signer_account_id(owner_id)
            .is_view(false)
            .build());

        let contract_new_state = State::Paused;
        contract.change_state(contract_new_state.clone());
        assert_eq!(contract.state, contract_new_state)
    }
}
