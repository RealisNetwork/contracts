use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractV020 {
    pub owner_id: AccountId,
    pub staking_contract: AccountId,
    pub lockup_contract: AccountId,
    pub ft: FungibleToken,
    pub last_mint: Timestamp,
    pub backend: UnorderedSet<AccountId>,
    pub suspensioned_accounts: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn update() -> Self {
        let mut contract_old: ContractV020 = env::state_read().unwrap();
        contract_old.suspensioned_accounts.clear();
        Self {
            owner_id: contract_old.owner_id,
            staking_contract: contract_old.staking_contract,
            ft: contract_old.ft,
            last_mint: contract_old.last_mint,
            backend: contract_old.backend,
        }
    }
}
