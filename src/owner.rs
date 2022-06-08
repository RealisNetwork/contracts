
use near_sdk::{AccountId, env, Timestamp};
use near_sdk::json_types::U128;
use near_sdk::require;
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

        if u128::MAX == self.nft_id_counter {
            self.nft_id_counter = 0;
        }
        while self.nfts.contains_key(&self.nft_id_counter) {
            self.nft_id_counter += 1;
        }

        let nft = Nft {
            meta_data: nft_metadata
        };
        self.nfts.insert(&self.nft_id_counter, &nft);

       let VAccount::V1(mut set_of_nft) = self.accounts.get(&recipient_id).unwrap_or_default();
            set_of_nft.nfts.insert(&self.nft_id_counter);

        self.nft_id_counter
    }
    fn only_owner(&mut self) {
        require!(env::predecessor_account_id() == self.owner_id.clone(), "Only owner can mint nft");
    }

    pub fn change_state(&mut self, state: State) {
        todo!()
    }

    pub fn change_beneficiary(&mut self, new_beneficiary_id: AccountId) {
        todo!()
    }

    pub fn create_lockup(&mut self, recipient_id: AccountId, amount: U128, duration: Option<Timestamp>) -> Timestamp {
        todo!()
    }

    pub fn create_account() {
        todo!()
    }
}

#[cfg(test)]
mod tests {
    use near_sdk::{testing_env, VMContext};
    use near_sdk::test_utils::VMContextBuilder;

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
            let context = get_context("not an owner".to_string());
            testing_env!(context);
            let res = contract.mint(AccountId::new_unchecked("user_id".to_string()), "some_metadata".to_string());
        }


        #[test]
        fn mint_nft_test() {
            let mut contract = get_contract();
            let context = get_context("owner".to_string());
            testing_env!(context);
            let res = contract.mint(AccountId::new_unchecked("user_id".to_string()), "some_metadata".to_string());
            println!("{}", res);
            assert!(contract.nfts.contains_key(&res));
            if let Some(VAccount::V1(mut set_of_nft)) = contract.accounts.get(&AccountId::new_unchecked("user_id".to_string())) {
                assert!(set_of_nft.nfts.contains(&res));
            }
            }




}