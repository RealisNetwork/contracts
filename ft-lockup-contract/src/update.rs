use crate::*;
use near_contract_standards::upgrade::Ownable;

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
    pub fn get_metadata(&self) -> String {
        String::from("1.0.1")
    }

    #[private]
    #[init(ignore_state)]
    pub fn update() -> Self {
        env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"))
    }
}
