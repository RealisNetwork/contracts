use near_contract_standards::non_fungible_token::TokenId;
use near_sdk::borsh::{self, BorshDeserialize, BorshSerialize};
use near_sdk::collections::LookupSet;
use near_sdk::{env, Balance};

// TODO: wrap by VAccount
#[derive(BorshSerialize, BorshDeserialize)]
pub struct Account {
    pub free: Balance,
    pub nft: LookupSet<TokenId>,
}

impl Account {
    pub fn new(balance: Balance) -> Self {
        Self {
            free: balance,
            nft: LookupSet::new(b'n'),
        }
    }

    pub fn increase_balance(mut self, amount: Balance) -> Self {
        self.free += amount;

        self
    }

    pub fn decrease_balance(mut self, amount: Balance) -> Self {
        self.free = self
            .free
            .checked_sub(amount)
            .unwrap_or_else(|| env::panic_str("Not enough free balance"));

        self
    }
}

// TODO: add doc @Artem_Levchuk
impl Default for Account {
    fn default() -> Self {
        Self {
            free: 0,
            nft: LookupSet::new(b'b'),
        }
    }
}
