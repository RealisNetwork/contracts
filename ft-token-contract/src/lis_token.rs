use crate::*;
use near_contract_standards::{fungible_token::receiver::ext_ft_receiver, upgrade::Ownable};
use near_sdk::{
    env, is_promise_success,
    json_types::{U128, U64},
    Gas,
};

/// The values of the constants do not exceed the u64 limits,
/// but changing the value of these constants is not provided!
/// If you need to change their values, be careful!
pub const MILLISECOND: U64 = U64(1_000);
pub const SECOND: U64 = U64(1000 * MILLISECOND.0);
pub const MINUTE: U64 = U64(60 * SECOND.0);
pub const HOUR: U64 = U64(60 * MINUTE.0);
pub const DAY: U64 = U64(24 * HOUR.0);
pub const WEEK: U64 = U64(7 * DAY.0);

pub const MINT_AMOUNT: Balance = 410_000_000_000_000_000;

pub const GAS_FOR_MINT: Gas = Gas(50_000_000_000_000);
pub const GAS_FOR_MINT_CALLBACK: Gas = Gas(20_000_000_000_000);

#[near_bindgen]
impl Contract {
    pub fn ft_mint(&mut self) {
        self.assert_owner();
        let time = env::block_timestamp()
            .checked_div(WEEK.0)
            .unwrap_or_else(|| env::panic_str("Div will overflow"))
            .checked_mul(WEEK.0)
            .unwrap_or_else(|| env::panic_str("Mul will overflow"));
        require!(
            self.last_mint
                .checked_add(WEEK.0)
                .unwrap_or_else(|| env::panic_str("Add will overflow"))
                <= time,
            "Too early"
        );
        self.ft
            .internal_deposit(&self.staking_contract, MINT_AMOUNT);
        self.last_mint = time;

        ext_ft_receiver::ext(self.staking_contract.clone())
            .with_static_gas(
                env::prepaid_gas()
                    .0
                    .checked_sub(GAS_FOR_MINT.0)
                    .unwrap_or_else(|| env::panic_str("Sub will overflow"))
                    .into(),
            )
            .ft_on_transfer(
                env::current_account_id(),
                MINT_AMOUNT.into(),
                "\"AddToPool\"".to_string(),
            )
            .then(
                Self::ext(env::current_account_id())
                    .with_static_gas(GAS_FOR_MINT_CALLBACK)
                    .transfer_on_mint_callback(),
            );
    }

    pub fn ft_burn(&mut self, amount: U128) {
        let amount: Balance = amount.into();
        require!(amount > 0, "The amount should not be zero");
        let sender_id = env::predecessor_account_id();
        self.ft.internal_withdraw(&sender_id, amount);
        near_contract_standards::fungible_token::events::FtBurn {
            owner_id: &sender_id,
            amount: &amount.into(),
            memo: None,
        }
        .emit();
    }

    #[private]
    pub fn transfer_on_mint_callback(&mut self) {
        if is_promise_success() {
            near_contract_standards::fungible_token::events::FtMint {
                owner_id: &self.staking_contract,
                amount: &MINT_AMOUNT.into(),
                memo: None,
            }
            .emit();
            return;
        }
        // Rollback deposit tokens if transfer fail
        self.ft
            .internal_withdraw(&self.staking_contract, MINT_AMOUNT);
    }
}

#[cfg(test)]
mod tests {
    use crate::{
        lis_token::{DAY, MINT_AMOUNT, WEEK},
        *,
    };
    use near_contract_standards::fungible_token::core::FungibleTokenCore;
    use near_sdk::{test_utils::*, testing_env};

    #[test]
    fn mint_time_check() {
        let owner_id = accounts(0);
        let staking_id = accounts(1);
        let initial_total_supply = 3_000_000_000 * 10_u128.pow(12);
        let context = VMContextBuilder::new();

        // init contract
        testing_env!(context.clone().block_timestamp(WEEK.0).build());
        let mut contract = Contract::new(Some(owner_id.clone()), None, staking_id);
        assert_eq!(contract.ft_total_supply().0, initial_total_supply);

        // wait week
        testing_env!(context
            .clone()
            .predecessor_account_id(owner_id.clone())
            .block_timestamp(env::block_timestamp() + WEEK.0)
            .build());
        // can mint
        contract.ft_mint();
        assert_eq!(
            contract.ft_total_supply().0,
            initial_total_supply + MINT_AMOUNT
        );

        // wait week
        testing_env!(context
            .clone()
            .predecessor_account_id(owner_id.clone())
            .block_timestamp(env::block_timestamp() + WEEK.0)
            .build());
        // can mint
        contract.ft_mint();
        assert_eq!(
            contract.ft_total_supply().0,
            initial_total_supply + 2 * MINT_AMOUNT
        );

        // wait week + 3 days; /// thursday
        testing_env!(context
            .clone()
            .predecessor_account_id(owner_id.clone())
            .block_timestamp(env::block_timestamp() + WEEK.0 + 3 * DAY.0)
            .build());
        // can mint
        contract.ft_mint();
        assert_eq!(
            contract.ft_total_supply().0,
            initial_total_supply + 3 * MINT_AMOUNT
        );

        // wait 5 days /// tuesday
        testing_env!(context
            .clone()
            .predecessor_account_id(owner_id.clone())
            .block_timestamp(env::block_timestamp() + 5 * DAY.0)
            .build());
        // can mint
        contract.ft_mint();
        assert_eq!(
            contract.ft_total_supply().0,
            initial_total_supply + 4 * MINT_AMOUNT
        );
    }

    #[test]
    #[should_panic = "Too early"]
    fn mint_before_new_week() {
        let owner_id = accounts(0);
        let staking_id = accounts(1);
        let initial_total_supply = 3_000_000_000 * 10_u128.pow(12);
        let context = VMContextBuilder::new();

        // init contract
        testing_env!(context.clone().block_timestamp(WEEK.0).build());
        let mut contract = Contract::new(Some(owner_id.clone()), None, staking_id);
        assert_eq!(contract.ft_total_supply().0, initial_total_supply);

        // wait 3 days
        testing_env!(context
            .clone()
            .predecessor_account_id(owner_id.clone())
            .block_timestamp(env::block_timestamp() + 3 * DAY.0)
            .build());
        // cannot mint
        contract.ft_mint();
    }

    #[test]
    #[should_panic = "Owner must be predecessor"]
    fn can_mint_only_owner() {
        let owner_id = accounts(0);
        let staking_id = accounts(1);
        let context = VMContextBuilder::new();

        // init contract
        testing_env!(context.clone().block_timestamp(WEEK.0).build());
        let mut contract = Contract::new(Some(owner_id.clone()), None, staking_id);

        // wait week
        testing_env!(context
            .clone()
            .predecessor_account_id(accounts(3))
            .block_timestamp(env::block_timestamp() + WEEK.0)
            .build());
        // can mint
        contract.ft_mint();
    }
}
