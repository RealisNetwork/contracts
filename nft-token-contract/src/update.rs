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

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractV1 {
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub token_by_id: UnorderedMap<TokenId, VersionedToken>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub locked_tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn update() -> Self {
        let contract: ContractV1 = env::state_read().expect("Failed to read contract data");

        Self {
            owner_id: contract.owner_id,
            backend_id: contract.backend_id,
            mint_accounts: UnorderedSet::new(StorageKey::MintAccounts),
            token_by_id: contract.token_by_id,
            tokens_per_owner: contract.tokens_per_owner,
            locked_tokens_per_owner: contract.locked_tokens_per_owner,
        }
    }
}
