use crate::{
    lockup::{Lockup, SimpleLockup},
    utils::DAY,
    *,
};
use near_sdk::{env, require, AccountId, Balance, Timestamp};

pub const STARTED_COST: u128 = 1000;
pub const DEFAULT_LOCKUP_TIME: Timestamp = 7 * DAY;

#[derive(BorshSerialize, BorshDeserialize)]
pub struct Staking {
    pub total_supply: Balance,
    pub total_x_supply: Balance,
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
        self.x_cost = self.total_supply * STARTED_COST / self.total_x_supply;
        self.total_supply
    }

    pub fn convert_to_x(&self, amount: u128) -> u128 {
        amount * STARTED_COST / self.x_cost
    }

    pub fn convert_to_amount(&self, x_amount: u128) -> u128 {
        x_amount * self.x_cost / STARTED_COST
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
    use super::*;
    use crate::{
        staking::{Staking, STARTED_COST},
        utils::tests_utils::*,
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
        staking.unstake(50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 600 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 150 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);

        // State: 7
        staking.unstake(100 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 200 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);

        // State: 8
        staking.unstake(50 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.total_supply, 0 * ONE_LIS);
        assert_eq!(staking.total_x_supply, 0 * STARTED_COST * ONE_LIS);
        assert_eq!(staking.x_cost, 4);
    }

    #[test]
    fn full_staking_cycle() {
        // create Owner
        let owner = accounts(2);

        // Init contract
        let (mut contract, mut context) = init_test_env(Some(owner.clone()), None, None);

        // create User 1
        let user1 = accounts(0);

        // register User 1 with 250 LiS
        contract
            .accounts
            .insert(&user1, &Account::new(accounts(0), 250 * ONE_LIS).into());

        let account: Account = contract.accounts.get(&user1).unwrap().into();
        assert_eq!(account.free, 250 * ONE_LIS);

        // create User 2
        let user2 = accounts(1);

        // register User 2 with 150 LiS
        contract
            .accounts
            .insert(&user2, &Account::new(accounts(1), 150 * ONE_LIS).into());
        let account: Account = contract.accounts.get(&user2).unwrap().into();
        assert_eq!(account.free, 150 * ONE_LIS);

        // set signer as User 1
        testing_env!(context.signer_account_id(user1.clone()).build());

        // stake as User 1  100 LiS
        let user1_staked = contract.stake(U128(100 * ONE_LIS));

        // Assert user1 tokens was taken
        let account: Account = contract.accounts.get(&user1).unwrap().into();
        assert_eq!(account.free, 150 * ONE_LIS);

        // set signer as owner
        testing_env!(context.signer_account_id(owner).build());

        // Airdrop 100 LIS
        contract.owner_add_to_staking_pool(U128(100 * ONE_LIS));
        let pool_staking = contract.staking.total_supply;

        // set signer as User 2
        testing_env!(context.signer_account_id(user2.clone()).build());

        // stake as User 2  100 LiS
        let user2_staked = contract.stake(U128(100 * ONE_LIS));

        // Assert user2 tokens was taken
        let account: Account = contract.accounts.get(&user2).unwrap().into();
        assert_eq!(account.free, 50 * ONE_LIS);

        // set signer as user1
        testing_env!(context.signer_account_id(user1.clone()).build());

        // user1 unstake
        contract.unstake(user1_staked);

        // set signer as user2
        testing_env!(context.signer_account_id(user2.clone()).build());

        // user2 unstake
        contract.unstake(user2_staked);

        // Wait till lockups are expired
        testing_env!(context.block_timestamp(9999999999999999).build());

        // claim loockup for staiking for User 2
        contract.claim_all_lockup();

        testing_env!(context.signer_account_id(user1.clone()).build());

        // Claim lockups for user1
        contract.claim_all_lockup();

        // Assert user1 balance == 350
        let account: Account = contract.accounts.get(&user1).unwrap().into();
        assert_eq!(account.free, 350 * ONE_LIS);

        // Assert user2 balance == 150
        let account: Account = contract.accounts.get(&user2).unwrap().into();
        assert_eq!(account.free, 150 * ONE_LIS);
    }
}
