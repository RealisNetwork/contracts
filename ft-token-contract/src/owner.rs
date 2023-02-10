use crate::*;
use near_contract_standards::upgrade::Ownable;
use near_sdk::assert_one_yocto;

#[near_bindgen]
impl Ownable for Contract {
    fn get_owner(&self) -> AccountId {
        self.owner_id.clone()
    }

    fn set_owner(&mut self, owner: AccountId) {
        self.assert_owner();
        self.owner_id = owner;
    }
}

#[near_bindgen]
impl Contract {
    #[payable]
    pub fn owner_add_backend(&mut self, backend_ids: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();

        self.backend.extend(backend_ids.into_iter());
    }

    #[payable]
    pub fn owner_remove_backend(&mut self, backend_ids: Vec<AccountId>) {
        assert_one_yocto();
        self.assert_owner();

        backend_ids.iter().for_each(|v| {
            self.backend.remove(v);
        });
    }

    pub fn get_backend_accounts(&self) -> Vec<AccountId> {
        self.backend.iter().collect()
    }
}
