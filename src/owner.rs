use near_sdk::{AccountId, env, Timestamp};
use near_sdk::json_types::U128;

use crate::*;

#[near_bindgen]
impl Contract {

    pub fn mint(&mut self, recipient_id: AccountId, nft_metadata: String) -> String {
        self.only_owner();

        let nft_id = env::sha256(nft_metadata.as_bytes());
        let nft_id = &hex::encode(nft_id);

        assert!(self.nft_ids.insert(nft_id), "Nft already exist");

        match self.users_nft.get(&recipient_id) {
            Some(mut set_of_nft) =>
                assert!(set_of_nft.insert(nft_id), "User already have this nft "),
            None => {
                let mut new_set = LookupSet::new(env::sha256(recipient_id.as_bytes()));
                new_set.insert(nft_id);
                self.users_nft.insert(&recipient_id, &new_set);
            }
        }
        nft_id.to_string()
    }
    fn only_owner(&mut self) {
        assert_eq!(env::predecessor_account_id(), self.owner_id.clone(), "Only owner can mint nft");
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
            .predecessor_account_id(AccountId::new_unchecked(caller_id))
            .is_view(false)
            .build()
    }


    pub fn get_contract() -> Contract {
        Contract {
            owner_id: "owner".to_string().parse().unwrap(),
            nft_ids: LookupSet::new(b"s"),
            users_nft: LookupMap::new(b"m"),
        }
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
        assert!(contract.nft_ids.contains(&res));
        assert!(contract.users_nft.get(&AccountId::new_unchecked("user_id".to_string())).is_some())
    }

    #[test]
    #[should_panic]
    fn the_same_meta_data_panic() {
        let mut contract = get_contract();
        let context = get_context("owner".to_string());
        testing_env!(context);
        contract.mint(AccountId::new_unchecked("user_id".to_string()), "some_metadata".to_string());
        contract.mint(AccountId::new_unchecked("user_id".to_string()), "some_metadata".to_string());
    }
}