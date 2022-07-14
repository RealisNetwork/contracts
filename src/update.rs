use crate::{Contract, ContractExt};
use near_sdk::{env, near_bindgen};

#[cfg(feature = "dev")]
use near_sdk::{json_types::Base64VecU8, AccountId};

#[near_bindgen]
impl Contract {
    #[private]
    #[init(ignore_state)]
    pub fn migrate() -> Self {
        let old_contract: Contract = env::state_read().expect("failed");
        Self {
            constant_fee: old_contract.constant_fee,
            percent_fee: old_contract.percent_fee,
            accounts: old_contract.accounts,
            nfts: old_contract.nfts,
            owner_id: old_contract.owner_id,
            backend_ids: old_contract.backend_ids,
            beneficiary_id: old_contract.beneficiary_id,
            state: old_contract.state,
            registered_accounts: old_contract.registered_accounts,
            staking: old_contract.staking,
        }
    }

    #[cfg(feature = "dev")]
    #[private]
    #[init(ignore_state)]
    pub fn clean(keys: Vec<Base64VecU8>, owner_id: AccountId) -> Self {
        for key in keys.iter() {
            env::storage_remove(&key.0);
        }
        Self {
            owner_id,
            ..Default::default()
        }
    }
}
