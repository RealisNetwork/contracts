use crate::*;

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
    pub fn update() -> Self {
        // TODO: migration
        todo!()
    }
}
