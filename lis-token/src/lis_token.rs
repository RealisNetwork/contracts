use crate::*;
use near_sdk::{env, json_types::U128};

#[near_bindgen]
impl Contract {
    pub fn ft_mint() {
        todo!()
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
}
