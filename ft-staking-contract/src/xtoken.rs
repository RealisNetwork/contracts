use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    env, Balance,
};
use primitive_types::U256;

pub const STARTED_COST: Balance = 1000;

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
        (U256::from(amount)
            .checked_mul(U256::from(self.xtokens_amount))
            .unwrap_or_else(|| env::panic_str("Mul will overflow"))
            .checked_div(U256::from(self.amount))
            .unwrap_or_else(|| env::panic_str("Div will overflow")))
        .as_u128()
    }

    pub fn convert_to_amount(&self, xtokens_amount: Balance) -> Balance {
        (U256::from(xtokens_amount)
            .checked_mul(U256::from(self.amount))
            .unwrap_or_else(|| env::panic_str("Mul will overflow"))
            .checked_div(U256::from(self.xtokens_amount))
            .unwrap_or_else(|| env::panic_str("Div will overflow")))
        .as_u128()
    }
}
