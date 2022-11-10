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
    #[private]
    #[init(ignore_state)]
    pub fn update() -> Self {
        let contract: ContractV0 =
            env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"));

        let mut contract = Self {
            owner_id: contract.owner_id,
            token_account_id: contract.token_account_id,
            lockup_account_id: contract.lockup_account_id,
            accounts: contract.accounts,
            total_supply: contract.total_supply,
            total_xtoken_supply: contract.total_xtoken_supply,
            xtoken_cost: contract.xtoken_cost,
            account_storage_usage: 0,
        };
        contract.measure_account_storage_usage();
        contract
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
}
