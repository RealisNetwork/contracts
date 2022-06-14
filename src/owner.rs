use near_sdk::{json_types::U128, AccountId, Timestamp};

use crate::{
    events::{EventLog, EventLogVariant, NftMintLog},
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

    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) -> u128 {
        self.assert_owner();

        EventLog::from(EventLogVariant::NftMint(NftMintLog {
            owner_id: String::from(recipient_id.clone()),
            meta_data: nft_metadata.clone(),
        }))
        .emit();

        self.nfts
            .mint_nft(recipient_id.clone(), nft_metadata.clone())
    }

    #[allow(unused_variables)]
    pub fn change_state(&mut self, state: State) {
        todo!()
    }

    #[allow(unused_variables)]
    pub fn change_beneficiary(&mut self, new_beneficiary_id: AccountId) {
        todo!()
    }

    /// Create lockup for account and get tokens from owner account
    pub fn create_lockup(
        &mut self,
        recipient_id: AccountId,
        amount: U128,
        duration: Option<Timestamp>,
    ) {
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
        self.accounts.insert(&self.owner_id, owner_account.into());

        let mut recipient_account: Account = self
            .accounts
            .get(&recipient_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        recipient_account
            .lockups
            .insert(&Lockup::new(amount.0, duration));
        self.accounts
            .insert(&recipient_id, &recipient_account.into());
    }

    /// Remove lockup from account and return balance to owner
    pub fn refund_lockup(&mut self, recipient_id: AccountId, duration: Timestamp) -> U128 {
        self.assert_owner();

        let mut recipient_account: Account = self
            .accounts
            .get(&recipient_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        let lockup = recipient_account
            .lockups
            .iter()
            .find(|lockup| lockup.expire_on == duration)
            .unwrap_or_else(|| env::panic_str("No such lockup"));
        recipient_account.lockups.remove(&lockup);
        self.accounts
            .insert(&recipient_id, &recipient_account.into());

        let mut owner_account: Account = self
            .accounts
            .get(&self.owner_id)
            .unwrap_or_else("No such account")
            .into();
        owner_account.free += lockup.amount;
        self.accounts.insert(&self.owner_id, &owner_account.into());

        lockup.amount.into()
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
        Contract::new(U128::from(123), U128(1), 10, None, None)
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
}
