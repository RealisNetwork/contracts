use near_sdk::{
    borsh::{self, BorshDeserialize, BorshSerialize},
    collections::LookupMap,
    env, require, AccountId, Balance, IntoStorageKey, Timestamp,
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
struct ShareCost {
    pub amount: Balance,
    pub shares_amount: Balance,
}

impl Default for ShareCost {
    fn default() -> Self {
        Self {
            amount: 1,
            shares_amount: STARTED_COST,
        }
    }
}

impl ShareCost {
    pub fn new(amount: Balance, shares_amount: Balance) -> Self {
        Self {
            amount,
            shares_amount,
        }
    }

    pub fn convert_to_shares(&self, amount: Balance) -> Balance {
        (U256::from(amount) * U256::from(self.shares_amount) / U256::from(self.amount)).as_u128()
    }

    pub fn convert_to_amount(&self, share_amount: Balance) -> Balance {
        (U256::from(share_amount) * U256::from(self.amount) / U256::from(self.shares_amount))
            .as_u128()
    }
}

pub struct FtStaking {
    /// AccountID -> Account shares.
    accounts: LookupMap<AccountId, Balance>,
    /// Total supply of the all tokens in staking
    total_supply: Balance,
    /// Total supply of the all shares
    total_shares_supply: Balance,
    /// Determine the price ration between tokens and shares
    share_cost: ShareCost,
}

impl FtStaking {
    fn new<S: IntoStorageKey>(prefix: S) -> Self {
        Self {
            accounts: LookupMap::new(prefix),
            total_supply: 0,
            total_shares_supply: 0,
            share_cost: Default::default(),
        }
    }

    fn stake(&mut self, account_id: &AccountId, amount: Balance) -> Balance {
        require!(amount > 0, "The amount should be a positive number");
        let shares_amount = self.share_cost.convert_to_shares(amount);
        self.total_supply
            .checked_add(amount)
            .unwrap_or_else(|| env::panic_str("Staking total supply overflow"));
        self.total_shares_supply
            .checked_add(shares_amount)
            .unwrap_or_else(|| env::panic_str("Staking total shares supply overflow"));
        let account_shares_amount = self.accounts.get(account_id).unwrap_or_default();
        self.accounts.insert(
            account_id,
            &account_shares_amount
                .checked_add(shares_amount)
                .unwrap_or_else(|| env::panic_str("Shares balance overflow")),
        );
        shares_amount
    }

    fn unstake(&mut self, account_id: &AccountId, shares_amount: Balance) -> Balance {
        require!(
            shares_amount > 0,
            "The shares_amount should be a positive number"
        );
        let amount = self.share_cost.convert_to_amount(shares_amount);
        self.total_supply
            .checked_sub(amount)
            .unwrap_or_else(|| env::panic_str("Staking total supply overflow"));
        self.total_shares_supply
            .checked_sub(shares_amount)
            .unwrap_or_else(|| env::panic_str("Staking total shares supply overflow"));
        let account_shares_amount = self.accounts.get(account_id).unwrap_or_default();
        self.accounts.insert(
            account_id,
            &account_shares_amount
                .checked_sub(shares_amount)
                .unwrap_or_else(|| {
                    env::panic_str("The account doesn't have enough shares balance")
                }),
        );
        amount
    }

    fn add_to_pool(&mut self, amount: Balance) -> Balance {
        require!(amount > 0, "The amount should be a positive number");
        self.total_supply
            .checked_add(amount)
            .unwrap_or_else(|| env::panic_str("Staking total supply overflow"));
        if self.total_shares_supply == 0 {
            return self.total_supply;
        }
        self.share_cost = ShareCost::new(self.total_supply, self.total_shares_supply);
        self.total_supply
    }
}
