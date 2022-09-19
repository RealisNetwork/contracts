use crate::*;
use near_contract_standards::{fungible_token::receiver::ext_ft_receiver, upgrade::Ownable};
use near_sdk::{env, is_promise_success, json_types::U128, Gas};

pub const MILLISECOND: u64 = 1_000_000;
pub const SECOND: u64 = 1000 * MILLISECOND;
pub const MINUTE: u64 = 60 * SECOND;
pub const HOUR: u64 = 60 * MINUTE;
pub const DAY: u64 = 24 * HOUR;
pub const WEEK: u64 = 7 * DAY;

pub const MINT_AMOUNT: Balance = 410_000_000_000_000_000;

pub const GAS_FOR_MINT: Gas = Gas(20_000_000_000_000);
pub const GAS_FOR_MINT_CALLBACK: Gas = Gas(20_000_000_000_000);

#[near_bindgen]
impl Contract {
    pub fn ft_mint(&mut self) {
        self.assert_owner();
        let time = env::block_timestamp() / WEEK * WEEK;
        require!(self.last_mint + WEEK <= time);

        ext_ft_receiver::ext(self.staking_contract.clone())
            .with_static_gas(env::prepaid_gas() - GAS_FOR_MINT)
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
        require!(amount > 0, "The amount should be a positive number");
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
            self.ft
                .internal_deposit(&self.staking_contract, MINT_AMOUNT);
            self.last_mint = env::block_timestamp() / WEEK * WEEK;
        }
        // TODO: mint event
    }
}
