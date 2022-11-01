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
        let contract: ContractV0 = env::state_read()
            .unwrap_or_else(|| env::panic_str("Not initialized"));

        Self {
            owner_id: contract.owner_id,
            staking_contract: contract.staking_contract,
            ft: contract.ft,
            last_mint: contract.last_mint,
            backend: UnorderedSet::new(b"b".to_vec()),
        }
    }
}

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractV0 {
    pub owner_id: AccountId,
    pub staking_contract: AccountId,
    pub ft: FungibleToken,
    pub last_mint: Timestamp,
}
