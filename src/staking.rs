use crate::{lockup::Lockup, utils::DAY, *};
use near_sdk::{env, require, AccountId, Balance, Timestamp};

pub const STARTED_COST: u128 = 1000;
pub const DEFAULT_LOCKUP_TIME: Timestamp = 7 * DAY;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Staking {
    total_supply: Balance,
    total_x_supply: Balance,
    /// xLIS = x_cost * LIS default x_cost = 0.001;
    x_cost: Balance,

    pub default_lockup_time: Timestamp,
}

impl Default for Staking {
    fn default() -> Self {
        Self {
            total_supply: 0,
            total_x_supply: 0,
            x_cost: 1,
            default_lockup_time: DEFAULT_LOCKUP_TIME,
        }
    }
}

impl Staking {
    fn stake(&mut self, amount: u128) -> u128 {
        let x_amount = self.convert_to_x(amount);
        self.total_supply += amount;
        self.total_x_supply += x_amount;
        x_amount
    }

    fn un_stake(&mut self, x_amount: u128) -> u128 {
        let amount = self.convert_to_amount(x_amount);
        self.total_supply -= amount;
        self.total_x_supply -= x_amount;
        amount
    }

    fn add_to_pool(&mut self, amount: u128) {
        self.total_supply += amount;
        if self.total_x_supply == 0 {
            return;
        }
        self.x_cost = self.total_supply * STARTED_COST / self.total_x_supply;
    }

    pub fn convert_to_x(&self, amount: u128) -> u128 {
        amount * STARTED_COST / self.x_cost
    }

    pub fn convert_to_amount(&self, x_amount: u128) -> u128 {
        x_amount * self.x_cost / STARTED_COST
    }
}

impl Contract {
    pub fn internal_stake(&mut self, staker_id: AccountId, amount: u128) {
        require!(amount > 0, "You can't stake zero tokens");

        let mut staker_account: Account = self
            .accounts
            .get(&staker_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        require!(staker_account.free >= amount, "Not enough balance");
        staker_account.free -= amount;
        let x_amount = self.staking.stake(amount);
        staker_account.x_staked += x_amount;
        self.accounts.insert(&staker_id, &staker_account.into());
    }

    pub fn internal_un_stake(&mut self, staker_id: AccountId, x_amount: u128) {
        require!(x_amount > 0, "You can't unstake zero x tokens");

        let mut staker_account: Account = self
            .accounts
            .get(&staker_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        require!(staker_account.x_staked >= x_amount, "Not enough x balance");
        staker_account.x_staked -= x_amount;

        let amount = self.staking.un_stake(x_amount);
        staker_account.lockups.insert(&Lockup {
            amount,
            expire_on: DEFAULT_LOCKUP_TIME,
        });
        self.accounts.insert(&staker_id, &staker_account.into());
    }

    pub fn internal_add_pool(&mut self, account_id: AccountId, amount: u128) {
        require!(amount > 0, "You can't add to pool zero tokens");

        let mut pool_account: Account = self
            .accounts
            .get(&account_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        require!(pool_account.free >= amount, "Not enough balance");
        pool_account.free -= amount;
        self.staking.add_to_pool(amount);
        self.accounts.insert(&account_id, &pool_account.into());
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        staking::{Staking, STARTED_COST},
        ONE_LIS,
    };

    #[test]
    fn staking_case_1() {
        let mut staking = Staking::default();

        // State: 1
        staking.stake(100 * ONE_LIS);
        assert_eq!(staking.total_supply, 100 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 100 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 1);

        // State: 2
        staking.add_to_pool(100 * ONE_LIS);
        assert_eq!(staking.total_supply, 200 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 100 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 2);

        // State: 3
        staking.stake(100 * ONE_LIS);
        assert_eq!(staking.total_supply, 300 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 2);

        // State: 4
        staking.add_to_pool(300 * ONE_LIS);
        assert_eq!(staking.total_supply, 600 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);

        // State: 5
        staking.stake(200 * ONE_LIS);
        assert_eq!(staking.total_supply, 800 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 200 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);

        // State: 6
        staking.un_stake(50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 600 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);

        // State: 7
        staking.un_stake(100 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 200 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);

        // State: 8
        staking.un_stake(50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 0 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 0 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);
    }
}