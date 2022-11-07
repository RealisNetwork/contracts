use crate::*;
use near_contract_standards::upgrade::Ownable;

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
    #[init(ignore_state)]
    pub fn update() -> Self {
        env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"))
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractV0 {
    pub owner_id: AccountId,
    pub token_account_id: AccountId,
    pub lockup_account_id: AccountId,
    accounts: LookupMap<AccountId, Balance>,
    total_supply: Balance,
    total_xtoken_supply: Balance,
    xtoken_cost: XTokenCost,
    account_storage_usage: u64,
}
