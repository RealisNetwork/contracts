use near_sdk::{env,near_bindgen};
use crate::ContractExt;
use crate::Contract;

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
            backend_id: old_contract.backend_id,
            beneficiary_id: old_contract.beneficiary_id,
            state: old_contract.state,
            registered_accounts: old_contract.registered_accounts,
        }
    }

}
