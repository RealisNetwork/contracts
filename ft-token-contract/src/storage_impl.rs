use crate::*;
use near_contract_standards::{
    fungible_token::core::FungibleTokenCore,
    storage_management::{StorageBalance, StorageBalanceBounds, StorageManagement},
};

#[near_bindgen]
impl StorageManagement for Contract {
    #[payable]
    fn storage_deposit(
        &mut self,
        account_id: Option<AccountId>,
        registration_only: Option<bool>,
    ) -> StorageBalance {
        self.ft.storage_deposit(account_id, registration_only)
    }

    #[payable]
    fn storage_withdraw(&mut self, amount: Option<U128>) -> StorageBalance {
        self.ft.storage_withdraw(amount)
    }

    #[payable]
    fn storage_unregister(&mut self, force: Option<bool>) -> bool {
        let account_id = env::predecessor_account_id();
        let balance = self.ft_balance_of(account_id).0;
        let force = force.unwrap_or_default();

        // Use `ft_burn` to emit burn event
        if balance > 0 && force {
            self.ft_burn(balance.into());
        }

        self.ft.storage_unregister(Some(force))
    }

    fn storage_balance_bounds(&self) -> StorageBalanceBounds {
        self.ft.storage_balance_bounds()
    }

    fn storage_balance_of(&self, account_id: AccountId) -> Option<StorageBalance> {
        self.ft.storage_balance_of(account_id)
    }
}
