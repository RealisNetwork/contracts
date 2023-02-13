use crate::*;

// pub struct Contract {
//     pub owner_id: AccountId,
//     pub staking_contract: AccountId,
//     pub lockup_contract: AccountId,
//     pub ft: FungibleToken,
//     pub last_mint: Timestamp,
//     pub backend: UnorderedSet<AccountId>,
//     pub suspensioned_accounts: UnorderedSet<AccountId>,
// }

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractV010 {
    pub owner_id: AccountId,
    pub staking_contract: AccountId,
    pub ft: FungibleToken,
    pub last_mint: Timestamp,
    pub backend: UnorderedSet<AccountId>,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn update(lockup_id: AccountId, suspensioned_accounts: Vec<AccountId>) -> Self {
        let contract_old: ContractV010 = env::state_read().unwrap();
        let mut this = Self {
            owner_id: contract_old.owner_id,
            staking_contract: contract_old.staking_contract,
            lockup_contract: lockup_id,
            ft: contract_old.ft,
            last_mint: contract_old.last_mint,
            backend: contract_old.backend,
            suspensioned_accounts: UnorderedSet::new(b"c".to_vec()),
        };
        this.suspensioned_accounts.extend(suspensioned_accounts);
        this
    }
}
