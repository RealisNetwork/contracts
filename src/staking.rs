use crate::{
    lockup::{Lockup, SimpleLockup},
    utils::DAY,
    *,
};
use near_sdk::{env, require, AccountId, Balance, Timestamp};
use primitive_types::U256;

pub const STARTED_COST: u128 = 1000;
pub const DEFAULT_LOCKUP_TIME: Timestamp = 7 * DAY;

#[derive(BorshSerialize, BorshDeserialize, PartialEq, Debug)]
struct XCost {
    pub amount: u128,
    pub x_amount: u128,
}

impl XCost {
    pub fn new(amount: u128, x_amount: u128) -> Self {
        Self { amount, x_amount }
    }
}

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Staking {
    total_supply: Balance,
    total_x_supply: Balance,
    x_cost: XCost,

    pub default_lockup_time: Timestamp,
}

impl Default for Staking {
    fn default() -> Self {
        Self {
            total_supply: 0,
            total_x_supply: 0,
            x_cost: XCost::new(1, STARTED_COST),
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

    fn unstake(&mut self, x_amount: u128) -> u128 {
        let amount = self.convert_to_amount(x_amount);
        self.total_supply -= amount;
        self.total_x_supply -= x_amount;
        amount
    }

    fn add_to_pool(&mut self, amount: u128) -> u128 {
        self.total_supply += amount;
        if self.total_x_supply == 0 {
            return self.total_supply;
        }
        self.x_cost = XCost::new(self.total_supply, self.total_x_supply);
        self.total_supply
    }

    pub fn convert_to_x(&self, amount: u128) -> u128 {
        (U256::from(amount) * U256::from(self.x_cost.x_amount) / U256::from(self.x_cost.amount))
            .as_u128()
    }

    pub fn convert_to_amount(&self, x_amount: u128) -> u128 {
        (U256::from(x_amount) * U256::from(self.x_cost.amount) / U256::from(self.x_cost.x_amount))
            .as_u128()
    }
}

impl Contract {
    pub fn internal_stake(&mut self, staker_id: AccountId, amount: u128) -> u128 {
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
        x_amount
    }

    pub fn internal_unstake(&mut self, staker_id: AccountId, x_amount: u128) -> u128 {
        require!(x_amount > 0, "You can't unstake zero x tokens");

        let mut staker_account: Account = self
            .accounts
            .get(&staker_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        require!(staker_account.x_staked >= x_amount, "Not enough x balance");
        staker_account.x_staked -= x_amount;

        let amount = self.staking.unstake(x_amount);
        staker_account
            .lockups
            .insert(&Lockup::Staking(SimpleLockup {
                amount,
                expire_on: self.staking.default_lockup_time,
            }));
        self.accounts.insert(&staker_id, &staker_account.into());
        amount
    }

    pub fn internal_add_pool(&mut self, account_id: AccountId, amount: u128) -> u128 {
        require!(amount > 0, "You can't add to pool zero tokens");
        require!(self.staking.total_x_supply > 0, "Zero pool balance");

        let mut pool_account: Account = self
            .accounts
            .get(&account_id)
            .unwrap_or_else(|| env::panic_str("No such account"))
            .into();
        require!(pool_account.free >= amount, "Not enough balance");
        pool_account.free -= amount;
        let pool_total_supply = self.staking.add_to_pool(amount);
        self.accounts.insert(&account_id, &pool_account.into());
        pool_total_supply
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        staking::{Staking, XCost, STARTED_COST},
        ONE_LIS,
    };

    #[test]
    fn staking_case_1() {
        let mut staking = Staking::default();

        // State: 1
        staking.stake(100 * ONE_LIS); // 100_000_000_000_000
        assert_eq!(staking.total_supply, 100 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 100 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, XCost::new(1, STARTED_COST));

        // State: 2
        staking.add_to_pool(100 * ONE_LIS);
        assert_eq!(staking.total_supply, 200 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 100 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(200 * ONE_LIS, 100 * STARTED_COST * ONE_LIS)
        );

        // State: 3
        staking.stake(100 * ONE_LIS);
        assert_eq!(staking.total_supply, 300 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(200 * ONE_LIS, 100 * STARTED_COST * ONE_LIS)
        );

        // State: 4
        staking.add_to_pool(300 * ONE_LIS);
        assert_eq!(staking.total_supply, 600 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(600 * ONE_LIS, 150 * STARTED_COST * ONE_LIS)
        );

        // State: 5
        staking.stake(200 * ONE_LIS);
        assert_eq!(staking.total_supply, 800 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 200 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(600 * ONE_LIS, 150 * STARTED_COST * ONE_LIS)
        );

        // State: 6
        staking.unstake(50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 600 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(600 * ONE_LIS, 150 * STARTED_COST * ONE_LIS)
        );

        // State: 7
        staking.unstake(100 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 200 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 50 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(600 * ONE_LIS, 150 * STARTED_COST * ONE_LIS)
        );

        // State: 8
        staking.unstake(50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 0 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 0 * STARTED_COST * ONE_LIS);
        assert_eq!(
            staking.x_cost,
            XCost::new(600 * ONE_LIS, 150 * STARTED_COST * ONE_LIS)
        );

        staking.stake(25 * ONE_LIS);
        assert_eq!(staking.total_supply, 25 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 625 * STARTED_COST * ONE_LIS / 100);
        assert_eq!(
            staking.x_cost,
            XCost::new(600 * ONE_LIS, 150 * STARTED_COST * ONE_LIS)
        );

        staking.add_to_pool(100 * ONE_LIS);
        assert_eq!(staking.total_supply, 125 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 625 * STARTED_COST * ONE_LIS / 100);
        assert_eq!(
            staking.x_cost,
            XCost::new(125 * ONE_LIS, 625 * STARTED_COST * ONE_LIS / 100)
        );

        staking.stake(20 * ONE_LIS);
        assert_eq!(staking.total_supply, 145 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 725 * STARTED_COST * ONE_LIS / 100);
        assert_eq!(
            staking.x_cost,
            XCost::new(125 * ONE_LIS, 625 * STARTED_COST * ONE_LIS / 100)
        );

        staking.add_to_pool(1000 * ONE_LIS);
        assert_eq!(staking.total_supply, 1145 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 725 * STARTED_COST * ONE_LIS / 100);
        assert_eq!(
            staking.x_cost,
            XCost::new(1145 * ONE_LIS, 725 * STARTED_COST * ONE_LIS / 100)
        );
    }
}
