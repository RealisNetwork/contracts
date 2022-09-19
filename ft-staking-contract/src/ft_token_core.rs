use crate::*;
use near_contract_standards::fungible_token::core::FungibleTokenCore;

#[near_bindgen]
impl FungibleTokenCore for Contract {
    #[allow(unused_variables)]
    fn ft_transfer(&mut self, receiver_id: AccountId, amount: U128, memo: Option<String>) {
        env::panic_str("Not transferable token!");
    }

    #[allow(unused_variables)]
    fn ft_transfer_call(
        &mut self,
        receiver_id: AccountId,
        amount: U128,
        memo: Option<String>,
        msg: String,
    ) -> PromiseOrValue<U128> {
        env::panic_str("Not transferable token!");
    }

    fn ft_total_supply(&self) -> U128 {
        self.total_xtoken_supply.into()
    }

    fn ft_balance_of(&self, account_id: AccountId) -> U128 {
        self.accounts.get(&account_id).unwrap_or_default().into()
    }
}
