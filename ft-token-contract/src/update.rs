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
    #[init(ignore_state)]
    pub fn update(owner_id: Option<AccountId>) -> Self {
        #[derive(BorshDeserialize, BorshSerialize, PanicOnDefault)]
        pub struct OldContract {
            pub ft: FungibleToken,
        }

        let contract: OldContract =
            env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"));

        Self {
            owner_id: owner_id.unwrap_or_else(env::predecessor_account_id),
            ft: contract.ft,
        }
    }
}
