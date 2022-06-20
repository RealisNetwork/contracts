use crate::*;
use near_sdk::{env, require, Balance};

pub const NANOSECOND: u64 = 1;
pub const MILLISECOND: u64 = 1_000_000 * NANOSECOND;
pub const SECOND: u64 = 1000 * MILLISECOND;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;

pub const DEFAULT_LOCK_LIFE_TIME: u64 = 3 * DAY;

pub const DECIMALS: u8 = 12;
pub const ONE_LIS: Balance = 10_u128.pow(DECIMALS as _);

impl Contract {
    pub fn assert_owner(&self) {
        require!(
            env::signer_account_id() == self.owner_id.clone(),
            "Only owner can do this"
        );
    }

    pub fn assert_backend(&self) {
        require!(
            self.backend_ids.contains(&env::signer_account_id()),
            "Not allowed"
        );
    }

    pub fn assert_running(&self) {
        require!(self.state == State::Running, "Contract is paused");
    }
}

#[cfg(test)]
pub mod tests_utils {
    pub use crate::{lockup::Lockup, utils::ONE_LIS, *};
    pub use near_sdk::{
        collections::LookupMap,
        json_types::U128,
        test_utils::{accounts, VMContextBuilder},
        testing_env, AccountId, Balance, Gas,
    };
    pub use std::str::FromStr;

    /// If you need to change context config outside of
    /// this function,you need to use testing_env! macro after
    /// changes.

    #[allow(dead_code)]
    pub fn init_test_env(
        owner_id: Option<AccountId>,
        beneficiary_id: Option<AccountId>,
        backend_id: Option<AccountId>,
    ) -> (Contract, VMContextBuilder) {
        let mut context = VMContextBuilder::new();
        context.prepaid_gas(Gas::ONE_TERA * 100);

        testing_env!(context
            .block_timestamp(0)
            .signer_account_id(owner_id.clone().unwrap_or_else(|| accounts(0)))
            .build());
        let contract = Contract::new(
            None,
            Some(U128(5 * ONE_LIS)),
            None,
            beneficiary_id,
            backend_id,
        );

        (contract, context)
    }
}
