use crate::*;
use near_sdk::{env, require, AccountId, PublicKey};

/// Converts `PublicKey` to `AccountId`
pub fn convert_pk_to_account_id(pk: PublicKey) -> AccountId {
    hex::encode(&pk.as_bytes()[1..])
        .try_into()
        .unwrap_or_else(|_| env::panic_str("Fail to convert PublicKey to AccountId"))
}

impl Contract {
    pub fn assert_owner(&self) {
        require!(
            env::predecessor_account_id() == self.owner_id.clone(),
            "Only owner can do this"
        );
    }

    pub fn assert_backend(&self) {
        require!(env::signer_account_id() == self.backend_id, "Not allowed");
    }

    pub fn assert_running(&self) {
        require!(self.state == State::Running, "Contract is paused");
    }
}

#[cfg(test)]
pub mod tests_utils {
    pub use near_sdk::test_utils::{accounts, VMContextBuilder};
    pub use near_sdk::{Balance, Gas, testing_env};
    pub use near_sdk::json_types::U128;
    pub use crate::*;

    pub const DECIMALS: u8 = 12;
    pub const ONE_LIS: Balance = 10_u128.pow(DECIMALS as _);

    #[allow(dead_code)]
    pub fn init_test_env(
        owner_id: Option<AccountId>,
        beneficiary_id: Option<AccountId>,
        backend_id: Option<AccountId>,
    ) -> (Contract, VMContextBuilder) {
        let mut context = VMContextBuilder::new();
        context.prepaid_gas(Gas::ONE_TERA * 100);

        testing_env!(
            context
                .block_timestamp(0)
                .predecessor_account_id(owner_id.unwrap_or_else(|| accounts(0)))
                .build()
        );
        let contract = Contract::new(
            U128(3_000_000_000 * ONE_LIS),
            U128(10 * ONE_LIS),
            10,
            beneficiary_id,
            backend_id
        );

        (contract, context)
    }
}