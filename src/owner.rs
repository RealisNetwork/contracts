use near_sdk::{env::panic_str, json_types::U128, AccountId, Timestamp};

use crate::{
    events::{EventLog, EventLogVariant, NftMintLog},
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

        let mut nft_owner_id = Account::from(
            self.accounts
                .get(&recipient_id)
                .unwrap_or_else(|| panic_str("Account not found")),
        );

        let nft_id = self.nfts.mint_nft(&recipient_id, nft_metadata);
        nft_owner_id.nfts.insert(&nft_id);
        self.accounts
            .insert(&recipient_id, &VAccount::V1(nft_owner_id));

        nft_id
    }

    #[allow(unused_variables)]
    pub fn change_state(&mut self, state: State) {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn change_beneficiary(&mut self, new_beneficiary_id: AccountId) {
        todo!()
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
    use near_sdk::{test_utils::VMContextBuilder, testing_env, VMContext};

    use super::*;

    pub fn get_context(caller_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::new_unchecked(caller_id))
            .is_view(false)
            .build()
    }

    pub fn get_contract() -> Contract {
        Contract::new(U128::from(123), U128::from(1), 10, None, None)
    }

    #[test]
    #[should_panic]
    fn mint_nft_test_panic() {
        let mut contract = get_contract();
        let context = get_context("not owner".to_string());
        testing_env!(context);

        contract.mint(
            AccountId::new_unchecked("user_id".to_string()),
            "some_metadata".to_string(),
        );
    }

    #[test]
    fn mint_nft_test() {
        let mut contract = get_contract();
        let context = get_context("user_id".to_string());
        testing_env!(context);

        contract.accounts.insert(&AccountId::new_unchecked("user_id".to_string()), &Account::default().into());

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
}
