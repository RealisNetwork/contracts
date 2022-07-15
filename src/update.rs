use crate::{Contract, ContractExt};
use near_sdk::{env, near_bindgen};

#[cfg(feature = "dev")]
use crate::{Account, State, StorageKey, ONE_LIS};
#[cfg(feature = "dev")]
use near_sdk::{
    collections::{LookupMap, UnorderedSet},
    json_types::Base64VecU8,
    AccountId,
};

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
        let mut accounts = LookupMap::new(StorageKey::Accounts);
        accounts.insert(
            &owner_id,
            &Account::new(owner_id.clone(), 3_000_000_000 * ONE_LIS).into(),
        );

        let mut backend_ids = UnorderedSet::new(StorageKey::BackendIds);
        backend_ids.insert(&owner_id.clone());

        Self {
            constant_fee: 0,
            percent_fee: 0,
            nfts: Default::default(),
            owner_id: owner_id.clone(),
            backend_ids,
            beneficiary_id: owner_id,
            state: State::Running,
            accounts,
            registered_accounts: LookupMap::new(StorageKey::RegisteredAccounts),
            staking: Default::default(),
        }
    }
}
