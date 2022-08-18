use crate::*;

#[derive(BorshDeserialize, BorshSerialize)]
pub struct ContractOld {
    pub owner_id: AccountId,
    pub backend_id: AccountId,
    pub token_by_id: UnorderedMap<TokenId, Token>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

#[near_bindgen]
impl Contract {
    #[init(ignore_state)]
    pub fn update() -> Self {
        let contract_old: ContractOld =
            env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"));

        Self {
            owner_id: contract_old.owner_id,
            backend_id: contract_old.backend_id,
            token_by_id: contract_old.token_by_id,
            tokens_per_owner: contract_old.tokens_per_owner,
            locked_tokens_per_owner: LookupMap::new(StorageKey::LockedTokens),
        }
    }
}
