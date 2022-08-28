use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    Balance, Timestamp,
};
use primitive_types::U256;

pub const NANOSECOND: u64 = 1;
pub const MILLISECOND: u64 = 1_000_000 * NANOSECOND;
pub const SECOND: u64 = 1000 * MILLISECOND;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;

pub const STARTED_COST: Balance = 1000;
pub const DEFAULT_LOCKUP_TIME: Timestamp = 7 * DAY;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Eq, Debug)]
pub struct XTokenCost {
    pub amount: Balance,
    pub xtokens_amount: Balance,
}

impl Default for XTokenCost {
    fn default() -> Self {
        Self {
            amount: 1,
            xtokens_amount: STARTED_COST,
        }
    }
}

impl XTokenCost {
    pub fn new(amount: Balance, xtokens_amount: Balance) -> Self {
        Self {
            amount,
            xtokens_amount,
        }
    }

    pub fn convert_to_xtokens(&self, amount: Balance) -> Balance {
        (U256::from(amount) * U256::from(self.xtokens_amount) / U256::from(self.amount)).as_u128()
    }

    pub fn convert_to_amount(&self, xtokens_amount: Balance) -> Balance {
        (U256::from(xtokens_amount) * U256::from(self.amount) / U256::from(self.xtokens_amount))
            .as_u128()
    }
}
