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
    pub token_by_id: UnorderedMap<TokenId, Token>,
    pub tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
    pub locked_tokens_per_owner: LookupMap<AccountId, UnorderedSet<TokenId>>,
}

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn update() -> Self {
        let mut contract: ContractV1 = env::state_read().unwrap_or_else(|| env::panic_str("Not initialized"));
        
        let tokens = contract.token_by_id.into_iter().map(|(k, v)| (k, VersionedToken::from(v))).collect::<Vec<_>>();
        contract.token_by_id.clear();

        let mut token_by_id = UnorderedMap::new(StorageKey::TokenById);
        token_by_id.extend(tokens);

        let mut this = Self {
            owner_id: contract.owner_id,
            backend_id: contract.backend_id,
            token_by_id,
            tokens_per_owner: contract.tokens_per_owner,
            locked_tokens_per_owner: contract.locked_tokens_per_owner,
        };

        this.measure_nft_storage_usage();

        this
    }
}
