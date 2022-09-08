use crate::*;
use near_contract_standards::fungible_token::receiver::FungibleTokenReceiver;
use near_sdk::{
    serde::{Deserialize, Serialize},
    serde_json,
};

#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "near_sdk::serde")]
pub enum FtMessage {
    Stake,
    StakeFor { account_id: AccountId },
    AddToPool,
}

#[near_bindgen]
impl FungibleTokenReceiver for Contract {
    fn ft_on_transfer(
        &mut self,
        sender_id: AccountId,
        amount: U128,
        msg: String,
    ) -> PromiseOrValue<U128> {
        require!(
            env::predecessor_account_id() == self.token_account_id,
            "Invalid token ID"
        );
        let amount = amount.0;

        match serde_json::from_str::<FtMessage>(&msg)
            .unwrap_or_else(|_| env::panic_str("Invalid msg"))
        {
            FtMessage::Stake => self.stake_internal(&sender_id, amount),
            FtMessage::StakeFor { account_id } => self.stake_internal(&account_id, amount),
            FtMessage::AddToPool => self.add_to_pool_internal(amount),
        }

        PromiseOrValue::Value(U128(0))
    }
}
