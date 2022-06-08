use near_contract_standards::non_fungible_token::refund_deposit;
use near_sdk::json_types::U128;
use near_sdk::require;
use near_sdk::{env, AccountId, StorageUsage, Timestamp};

use crate::events::EventLogVariant::NftMint;
use crate::events::{EventLog, NftMintLog};
use crate::*;

#[near_bindgen]
impl Contract {
    /// `fn mint` creates new nft with uniq id.
    /// `fn mint` could be used only by the contract owner.
    ///  # Examples
    /// ```
    /// self.only_owner();
    /// ```
    /// # Arguments
    ///  * `recipient_id`- `AccountId` of future nft owner.
    ///  * `nft_metadata`-specific for new nft metadata.

    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) -> u128 {
        self.only_owner();

        /// we need this?
        let storage_usage = env::storage_usage();

        if u128::MAX == self.nft_id_counter {
            self.nft_id_counter = 0;
        }
        while self.nfts.contains_key(&self.nft_id_counter) {
            self.nft_id_counter += 1;
        }

        let nft = Nft {
            meta_data: nft_metadata.clone(),
        };
        self.nfts.insert(&self.nft_id_counter, &nft);

        let VAccount::V1(mut set_of_nft) = self.accounts.get(&recipient_id).unwrap_or_default();
        set_of_nft.nfts.insert(&self.nft_id_counter);

        Self::event_mint_nft(recipient_id, nft_metadata, storage_usage);

        self.nft_id_counter
    }

    fn only_owner(&mut self) {
        require!(
            env::predecessor_account_id() == self.owner_id.clone(),
            "Only owner can mint nft"
        );
    }

    /// `fn event_mint_nft` make log of mint nft,
    /// for getting to know `Near` about it.
    /// # Arguments
    /// * `account_id` AccountId of new owner.
    /// * `nft_metadata` some specific nft data.
    /// * `initial_storage_usage` measure the initial storage being used on the contract
    fn event_mint_nft(
        account_id: AccountId,
        nft_metadata: String,
        initial_storage_usage: StorageUsage,
    ) {
        let mint_log = EventLog::new(NftMint(vec![NftMintLog {
            owner_id: String::from(account_id),
            /// what should in logs , except ac_id ?
            meta_data: nft_metadata,
        }]));
        env::log_str(&mint_log.to_string());
        /// we need this logic?
        let required_storage_in_bytes = env::storage_usage() - initial_storage_usage;
        refund_deposit(required_storage_in_bytes);
    }

    pub fn change_state(&mut self, state: State) {
        todo!()
    }

    pub fn change_beneficiary(&mut self, new_beneficiary_id: AccountId) {
        todo!()
    }

    pub fn create_lockup(
        &mut self,
        recipient_id: AccountId,
        amount: U128,
        duration: Option<Timestamp>,
    ) -> Timestamp {
        todo!()
    }

    pub fn create_account() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::test_utils::VMContextBuilder;
    use near_sdk::{testing_env, VMContext};

    use super::*;

    pub fn get_context(caller_id: String) -> VMContext {
        VMContextBuilder::new()
            .signer_account_id(AccountId::new_unchecked(caller_id))
            .is_view(false)
            .build()
    }

    pub fn get_contract() -> Contract {
        Contract::new(U128::from(123), 1, None, None)
    }

    #[test]
    #[should_panic]
    fn mint_nft_test_panic() {
        let mut contract = get_contract();
        let context = get_context("not owner".to_string());
        testing_env!(context);
        let res = contract.mint(
            AccountId::new_unchecked("user_id".to_string()),
            "some_metadata".to_string(),
        );
    }

    #[test]
    fn mint_nft_test() {
        let mut contract = get_contract();
        let context = get_context("owner".to_string());
        testing_env!(context);
        let res = contract.mint(
            AccountId::new_unchecked("user_id".to_string()),
            "some_metadata".to_string(),
        );
        println!("{}", res);
        assert!(contract.nfts.contains_key(&res));
        if let Some(VAccount::V1(mut set_of_nft)) = contract
            .accounts
            .get(&AccountId::new_unchecked("user_id".to_string()))
        {
            assert!(set_of_nft.nfts.contains(&res));
        }
    }
}
